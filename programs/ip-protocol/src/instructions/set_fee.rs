use anchor_lang::prelude::*;
use crate::state::RegistryState;
use crate::error::ErrorCode;


#[derive(Accounts)]
pub struct SetRegistrationFee<'info> {
    #[account(mut)]
    pub registry: Account<'info, RegistryState>,
    
    pub authority: Signer<'info>,
}

impl<'info> SetRegistrationFee<'info> {
    pub fn set_fee(
        &mut self,
        treasury: Pubkey,
        fee_token: Pubkey,
        fee_amount: u64,
    ) -> Result<()> {
        let registry = &mut self.registry;
        
        require!(
            registry.authority == self.authority.key(),
            ErrorCode::Unauthorized
        );
        
        registry.treasury = treasury;
        registry.fee_token = fee_token;
        registry.fee_amount = fee_amount;

        Ok(())
    }
}
