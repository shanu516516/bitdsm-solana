// programs/bitdsm/src/lib.rs

// const avsDirectoryAddress = coreDeploymentData.anvil.avsDirectory;

// const bitDSMServiceManagerAddress = avsDeploymentData.BitDSMServiceManagerProxy;
// const bitDSMRegistryAddress = avsDeploymentData.BitDSMRegistryProxy;
// const bitcoinPodManagerAddress = avsDeploymentData.BitcoinPodManagerProxy;

// // Load ABIs
// const delegationManagerABI = JSON.parse(
//   fs.readFileSync(
//     path.resolve(__dirname, "../abis/IDelegationManager.json"),
//     "utf8"

use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

declare_id!("FSz1FrDprWyWHCiqBLK8g9Zs4w1ZQAkMQfFk1obrG3iP");

#[program]
pub mod bitdsm_solana {
    use super::*;

    pub fn initialize_registry(
        ctx: Context<InitializeRegistry>,
        min_stake_weight: u64,
    ) -> Result<()> {
        require!(min_stake_weight > 0, BitDSMError::InvalidStakeWeight);
        
        let registry = &mut ctx.accounts.registry;
        registry.authority = ctx.accounts.authority.key();
        registry.min_stake_weight = min_stake_weight;
        registry.operator_count = 0;
        registry.total_stake = 0;
        Ok(())
    }

    pub fn register_app(
        ctx: Context<RegisterApp>,
        app_name: String,
        app_metadata: String,
    ) -> Result<()> {
        require!(!app_name.is_empty(), BitDSMError::InvalidAppName);
        require!(app_name.len() <= 32, BitDSMError::AppNameTooLong);
        
        let app = &mut ctx.accounts.app;
        let clock = Clock::get()?;
        
        app.authority = ctx.accounts.authority.key();
        app.name = app_name;
        app.metadata = app_metadata;
        app.is_active = true;
        app.created_at = clock.unix_timestamp;
        Ok(())
    }

    pub fn create_pod(
        ctx: Context<CreatePod>, 
        btc_public_key: String
    ) -> Result<()> {
        require!(!btc_public_key.is_empty(), BitDSMError::InvalidBtcKey);
        require!(btc_public_key.len() == 66, BitDSMError::InvalidBtcKey);
        require!(btc_public_key.chars().all(|c| c.is_ascii_hexdigit()), BitDSMError::InvalidBtcKey);
        
        let pod = &mut ctx.accounts.pod;
        let clock = Clock::get()?;
        
        pod.authority = ctx.accounts.authority.key();
        pod.btc_public_key = btc_public_key;
        pod.is_active = true;
        pod.balance = 0;
        pod.created_at = clock.unix_timestamp;
        pod.last_updated = clock.unix_timestamp;
        Ok(())
    }

    pub fn confirm_deposit(
        ctx: Context<ConfirmDeposit>, 
        amount: u64
    ) -> Result<()> {
        require!(amount > 0, BitDSMError::InvalidAmount);
        
        let pod = &mut ctx.accounts.pod;
        pod.balance = pod.balance.checked_add(amount).ok_or(BitDSMError::Overflow)?;
        pod.last_updated = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[error_code]
pub enum BitDSMError {
    #[msg("Invalid stake weight")]
    InvalidStakeWeight,
    #[msg("Invalid application name")]
    InvalidAppName,
    #[msg("Application name too long")]
    AppNameTooLong,
    #[msg("Invalid Bitcoin public key")]
    InvalidBtcKey,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Pod is inactive")]
    InactivePod,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Unauthorized")]
    Unauthorized,
}

#[derive(Accounts)]
pub struct InitializeRegistry<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8 + 8
    )]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterApp<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 200 + 1 + 8
    )]
    pub app: Account<'info, App>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(btc_public_key: String)]
pub struct CreatePod<'info> {
    #[account(
        init,
        payer = authority,
        space = 8      // discriminator
            + 32       // authority: Pubkey
            + 4 + 66   // btc_public_key: String (4 bytes for length + 66 bytes)
            + 1        // is_active: bool
            + 8        // balance: u64
            + 8        // created_at: i64
            + 8,       // last_updated: i64
        seeds = [
            b"pod",
            authority.key().as_ref(),
            &btc_public_key.as_bytes()[0..8] // Use first 8 bytes as seed
        ],
        bump
    )]
    pub pod: Account<'info, Pod>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmDeposit<'info> {
    #[account(
        mut,
        has_one = authority @ BitDSMError::Unauthorized,
        constraint = pod.is_active @ BitDSMError::InactivePod,
        seeds = [
            b"pod",
            authority.key().as_ref(),
            &pod.btc_public_key.as_bytes()[0..8] // Use first 8 bytes as seed
        ],
        bump
    )]
    pub pod: Account<'info, Pod>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Registry {
    pub authority: Pubkey,
    pub min_stake_weight: u64,
    pub operator_count: u64,
    pub total_stake: u64,
}

#[account]
pub struct App {
    pub authority: Pubkey,
    pub name: String,
    pub metadata: String,
    pub is_active: bool,
    pub created_at: i64,
}

#[account]
#[derive(Default)]
pub struct Pod {
    pub authority: Pubkey,            // 32 bytes
    pub btc_public_key: String,       // Dynamic
    pub is_active: bool,              // 1 byte
    pub balance: u64,                 // 8 bytes
    pub created_at: i64,              // 8 bytes
    pub last_updated: i64,            // 8 bytes
}
