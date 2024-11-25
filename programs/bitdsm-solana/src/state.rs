// programs/bitdsm/src/state.rs

#[account]
#[derive(Default)]
pub struct Registry {
    pub authority: Pubkey,            // Authority that can update registry settings
    pub min_stake_weight: u64,        // Minimum stake required for operators
    pub operator_count: u64,          // Total number of active operators
    pub total_stake: u64,             // Total stake in the system
}

#[account]
#[derive(Default)]
pub struct App {
    pub authority: Pubkey,            // Authority that manages the app
    pub name: String,                 // Application name
    pub metadata: String,             // Application metadata/configuration
    pub is_active: bool,              // Whether the app is currently active
    pub created_at: i64,              // Timestamp when app was created
}

#[account]
#[derive(Default)]
pub struct Pod {
    pub authority: Pubkey,            // Pod authority/owner
    pub btc_public_key: String,       // Bitcoin public key for the pod
    pub is_active: bool,              // Whether the pod is active
    pub balance: u64,                 // Current balance in the pod
    pub created_at: i64,              // Timestamp when pod was created
    pub last_updated: i64,            // Last update timestamp
}
