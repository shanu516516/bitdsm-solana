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
use anchor_lang::solana_program::{program::invoke, system_instruction};

declare_id!("FSz1FrDprWyWHCiqBLK8g9Zs4w1ZQAkMQfFk1obrG3iP");

#[program]
pub mod bitdsm_solana {
    use super::*;

    // Initialize the BitDSM Registry
    pub fn initialize_registry(
        ctx: Context<InitializeRegistry>,
        min_stake_weight: u64,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.authority = ctx.accounts.authority.key();
        registry.min_stake_weight = min_stake_weight;
        Ok(())
    }

    // Register an application
    pub fn register_app(
        ctx: Context<RegisterApp>,
        app_name: String,
        app_metadata: String,
    ) -> Result<()> {
        let app = &mut ctx.accounts.app;
        app.authority = ctx.accounts.authority.key();
        app.name = app_name;
        app.metadata = app_metadata;
        Ok(())
    }

    // Create a Bitcoin Pod
    pub fn create_pod(ctx: Context<CreatePod>, btc_public_key: String) -> Result<()> {
        let pod = &mut ctx.accounts.pod;
        pod.authority = ctx.accounts.authority.key();
        pod.btc_public_key = btc_public_key;
        pod.is_active = true;
        Ok(())
    }

    // Confirm a deposit
    pub fn confirm_deposit(ctx: Context<ConfirmDeposit>, amount: u64) -> Result<()> {
        let pod = &mut ctx.accounts.pod;
        // Logic to confirm the deposit goes here
        // For example, you might want to update the pod's state or balance
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeRegistry<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterApp<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 64 + 256)]
    pub app: Account<'info, App>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePod<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 64 + 1)]
    pub pod: Account<'info, Pod>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmDeposit<'info> {
    #[account(mut)]
    pub pod: Account<'info, Pod>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Registry {
    pub authority: Pubkey,
    pub min_stake_weight: u64,
}

#[account]
pub struct App {
    pub authority: Pubkey,
    pub name: String,
    pub metadata: String,
}

#[account]
pub struct Pod {
    pub authority: Pubkey,
    pub btc_public_key: String,
    pub is_active: bool,
}
