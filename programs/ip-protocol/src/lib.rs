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

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}
