// programs/bitdsm/src/state.rs

#[account]
pub struct Registry {
    pub authority: Pubkey,
    pub min_stake_weight: u64,
    pub operator_count: u64,
}

#[account]
pub struct Operator {
    pub authority: Pubkey,
    pub btc_public_key: Vec<u8>,
    pub is_active: bool,
}

#[account]
pub struct BitcoinPod {
    pub owner: Pubkey,
    pub operator: Pubkey,
    pub btc_address: Vec<u8>,
    pub balance: u64,
}
