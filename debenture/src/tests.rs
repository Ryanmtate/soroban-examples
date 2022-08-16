use core::ops::Add;

use super::*;
// use ed25519_dalek::Keypair;
use rand::{thread_rng, RngCore};
use soroban_sdk::{vec, Env, FixedBinary};

fn generate_contract_id() -> [u8; 32] {
    let mut id: [u8; 32] = Default::default();
    thread_rng().fill_bytes(&mut id);
    id
}

#[test]
fn test() {
    let env = Env::default();
    let contract_id = FixedBinary::from_array(&env, generate_contract_id());
    env.register_contract(&contract_id, DebentureContract);

    let maturity = BigInt::from_i64(
        &env,
        chrono::Utc::now()
            .add(chrono::Duration::days(365))
            .timestamp(),
    );
    let coupon_rate = BigInt::from_u32(&env, 750);
    let par_value = BigInt::from_u64(&env, 1e5 as u64);
    let debenture_holder = FixedBinary::from_array(&env, generate_contract_id());

    issue::invoke(
        &env,
        &contract_id,
        &maturity,
        &coupon_rate,
        &par_value,
        &(CouponPaymentFrequency::Annually as u32),
        &debenture_holder,
    );

    // return the maturity of the debenture
    let retrieved_maturity = maturity::invoke(&env, &contract_id);

    // assert the maturity is correct
    assert_eq!(maturity, retrieved_maturity, "maturity is incorrect");
}
