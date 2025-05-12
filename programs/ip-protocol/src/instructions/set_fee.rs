use anchor_lang::prelude::*;
use crate::state::RegistryState;
use crate::error::ErrorCode;

pub fn handler(
    ctx: Context<SetRegistrationFee>,
    treasury: Pubkey,
    fee_token: Pubkey,
    fee_amount: u64,
) -> Result<()> {
    let registry = &mut ctx.accounts.registry;
    
    require!(
        registry.authority == ctx.accounts.authority.key(),
        ErrorCode::Unauthorized
    );
    
    registry.treasury = treasury;
    registry.fee_token = fee_token;
    registry.fee_amount = fee_amount;

    Ok(())
}

#[derive(Accounts)]
pub struct SetRegistrationFee<'info> {
    #[account(mut)]
    pub registry: Account<'info, RegistryState>,
    
    pub authority: Signer<'info>,
}
