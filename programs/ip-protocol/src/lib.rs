pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("3N5eT8qudgR2Hy3zSQjaEJTc7tepRS4hkjRAvS6UZErR");

#[program]
pub mod ip_protocol {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        treasury: Pubkey,
        fee_token: Pubkey,
        fee_amount: u64,
    ) -> Result<()> {
        ctx.accounts.init(treasury, fee_token, fee_amount)?;
        Ok(())
    }

    pub fn set_registration_fee(
        ctx: Context<SetRegistrationFee>,
        treasury: Pubkey,
        fee_token: Pubkey,
        fee_amount: u64,
    ) -> Result<()> {
        ctx.accounts.set_fee(treasury, fee_token, fee_amount)
    }

    pub fn register(
        ctx: Context<Register>,
        chain_id: u64,
        token_contract: Pubkey,
        token_id: u64,
        name: String,
        uri: String,
    ) -> Result<()> {
        ctx.accounts
            .register(chain_id, token_contract, token_id, name, uri)
    }

    /// Upgrades the IP account implementation
    pub fn upgrade_ip_account_impl(
        ctx: Context<UpgradeIPAccountImpl>,
        new_implementation: Pubkey,
    ) -> Result<()> {
        ctx.accounts.upgrade(new_implementation)
    }

}
