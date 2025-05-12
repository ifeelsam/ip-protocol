use anchor_lang::prelude::*;
use crate::state::RegistryState;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct UpgradeIPAccountImpl<'info> {
    #[account(mut)]
    pub registry: Account<'info, RegistryState>,
    
    pub authority: Signer<'info>,
}

impl<'info> UpgradeIPAccountImpl<'info> {
    pub fn upgrade(&mut self, new_implementation: Pubkey) -> Result<()> {
        let registry = &mut self.registry;
        
        require!(
            registry.authority == self.authority.key(),
            ErrorCode::Unauthorized
        );
        
        registry.ip_account_implementation = new_implementation;
        
        Ok(())
    }
}
