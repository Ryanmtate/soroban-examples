#![no_std]

#[cfg(test)]
mod tests;

use core::ops::Div;

use soroban_sdk::{
    contractimpl, BigInt, Binary, Env, EnvType, FixedBinary, IntoVal, RawVal, Symbol, Vec,
};

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum DataKey {
    Maturity = 0,
    CouponRate = 1,
    ParValue = 2,
    DebentureHolder = 3,
    CouponPaymentFrequency = 4,
}

impl IntoVal<Env, RawVal> for DataKey {
    fn into_val(self, env: &Env) -> RawVal {
        (self as u32).into_val(env)
    }
}

/// How often the coupon payment is paid.
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum CouponPaymentFrequency {
    Annually = 0,
    Biannually = 1,
    Quarterly = 2,
    Monthly = 3,
    Weekly = 4,
    Daily = 5,
}

impl CouponPaymentFrequency {
    pub fn into_big_int(self, e: &Env) -> BigInt {
        match self {
            CouponPaymentFrequency::Annually => BigInt::from_u32(e, 1),
            CouponPaymentFrequency::Biannually => BigInt::from_u32(e, 2),
            CouponPaymentFrequency::Quarterly => BigInt::from_u32(e, 4),
            CouponPaymentFrequency::Monthly => BigInt::from_u32(e, 12),
            CouponPaymentFrequency::Weekly => BigInt::from_u32(e, 52),
            CouponPaymentFrequency::Daily => BigInt::from_u32(e, 365),
        }
    }
}

impl From<u32> for CouponPaymentFrequency {
    fn from(value: u32) -> Self {
        match value {
            0 => CouponPaymentFrequency::Annually,
            1 => CouponPaymentFrequency::Biannually,
            2 => CouponPaymentFrequency::Quarterly,
            3 => CouponPaymentFrequency::Monthly,
            4 => CouponPaymentFrequency::Weekly,
            5 => CouponPaymentFrequency::Daily,
            _ => panic!("Invalid value for CouponPaymentFrequency: {}", value),
        }
    }
}

impl IntoVal<Env, RawVal> for CouponPaymentFrequency {
    fn into_val(self, env: &Env) -> RawVal {
        (self as u32).into_val(env)
    }
}

/**
 * Debenture Interface
 *
 * Debentures are unsecured bonds, and pay a fixed interest rate.
 * Investors purchase a debenture, holding the debenture until maturity,
 * or may sell the debenture at any time.
 *
 */
pub trait Debenture {
    // Issue a new debenture, this is the initialization of the debenture contract.
    // Coupon rate is specified as basis points per annum.
    fn issue(
        e: Env,
        maturity: BigInt,
        coupon_rate: BigInt,
        par_value: BigInt,
        coupon_payment_frequency: u32,
        debenture_holder: FixedBinary<32>,
    );

    // Return the maturity date of the debenture
    fn maturity(e: Env) -> BigInt;

    // Return the par value of the debenture
    fn par_value(e: Env) -> BigInt;

    // Return the coupon payment for the current period.
    // fn coupon_payment(e: Env) -> BigInt;
}

fn get_maturity(e: &Env) -> BigInt {
    e.contract_data()
        .get(DataKey::Maturity)
        .unwrap_or(Ok(BigInt::zero(e)))
        .unwrap()
}

fn get_par_value(e: &Env) -> BigInt {
    e.contract_data()
        .get(DataKey::ParValue)
        .unwrap_or(Ok(BigInt::zero(e)))
        .unwrap()
}

fn get_coupon_rate(e: &Env) -> BigInt {
    e.contract_data()
        .get(DataKey::CouponRate)
        .unwrap_or(Ok(BigInt::zero(e)))
        .unwrap()
}

fn get_coupon_payment_frequency(e: &Env) -> u32 {
    e.contract_data()
        .get(DataKey::CouponPaymentFrequency)
        .unwrap_or(Ok(0))
        .unwrap()
}

// Calculate the coupon payment for the debenture
fn calculate_coupon_payment(e: &Env, timestamp: i64) -> BigInt {
    let maturity = get_maturity(e);
    let par_value = get_par_value(e);
    let coupon_rate = get_coupon_rate(e);
    let payment_frequency =
        CouponPaymentFrequency::from(get_coupon_payment_frequency(e)).into_big_int(e);

    // Return zero if the maturity has expired
    let now = BigInt::from_i64(e, timestamp);
    if now > maturity {
        return BigInt::zero(e);
    }

    (par_value * (coupon_rate / payment_frequency)) / BigInt::from_u32(e, 100)
}

pub struct DebentureContract;

#[contractimpl(export_if = "export")]
impl Debenture for DebentureContract {
    fn issue(
        e: Env,
        maturity: BigInt,
        coupon_rate: BigInt,
        par_value: BigInt,
        coupon_payment_frequency: u32,
        debenture_holder: FixedBinary<32>,
    ) {
        // Set the maturity of the debenture.
        e.contract_data().set(DataKey::Maturity, maturity);

        // Set the coupon rate of the debenture.
        e.contract_data().set(DataKey::CouponRate, coupon_rate);

        // Set the par value of the debenture.
        e.contract_data().set(DataKey::ParValue, par_value);

        // Set the address of the holder of the debenture.
        e.contract_data()
            .set(DataKey::DebentureHolder, debenture_holder);

        // Set the coupon payment frequency of the debenture.
        e.contract_data()
            .set(DataKey::CouponPaymentFrequency, coupon_payment_frequency);
    }

    fn maturity(e: Env) -> BigInt {
        get_maturity(&e)
    }

    // fn coupon_payment(e: Env) -> BigInt {
    //     // NOTE: need to identify how to create a timestamp from the env.
    //     todo!()
    // }

    // // Get the coupon rate of the debenture.
    // fn get_coupon_rate(e: Env) -> u32 {
    //     e.contract_data().get(COUPON_RATE).unwrap()
    // }

    // Get the par value of the debenture.
    fn par_value(e: Env) -> BigInt {
        get_par_value(&e)
    }

    // // Get the address of the holder of the debenture.
    // fn get_debenture_holder(e: Env) -> FixedBinary<32> {
    //     e.contract_data().get(DEBENTURE_HOLDER).unwrap()
    // }
}
