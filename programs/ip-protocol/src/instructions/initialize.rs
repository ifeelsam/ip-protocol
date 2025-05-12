use anchor_lang::prelude::*;
use crate::state::RegistryState;

pub fn handler(
    ctx: Context<Initialize>,
    treasury: Pubkey,
    fee_token: Pubkey,
    fee_amount: u64,
) -> Result<()> {
    let registry = &mut ctx.accounts.registry;
    registry.authority = ctx.accounts.authority.key();
    registry.ip_count = 0;
    registry.treasury = treasury;
    registry.fee_token = fee_token;
    registry.fee_amount = fee_amount;
    registry.ip_account_implementation = ctx.accounts.ip_account_implementation.key();

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = RegistryState::INIT_SPACE)]
    pub registry: Account<'info, RegistryState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// CHECK: Address of the IP account implementation
    pub ip_account_implementation: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}
