use borsh::{BorshDeserialize, BorshSerialize};
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct Rational {
    pub num: u64,
    pub denom: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct LiquidityLinearParams {
    pub max_liq_remaining: Rational,
    pub zero_liq_remaining: Rational,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum FeeEnum {
    Flat,
    LiquidityLinear,
}
