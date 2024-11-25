// programs/bitdsm/src/contexts.rs

#[derive(Accounts)]
pub struct InitializeRegistry<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8
    )]
    pub registry: Account<'info, Registry>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterOperator<'info> {
    #[account(mut)]
    pub registry: Account<'info, Registry>,

    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 33 + 1
    )]
    pub operator: Account<'info, Operator>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateBitcoinPod<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 32 + 64 + 8
    )]
    pub pod: Account<'info, BitcoinPod>,

    pub operator: Account<'info, Operator>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
