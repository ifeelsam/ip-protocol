use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::state::{RegistryState, IPData};

#[derive(Accounts)]
#[instruction(chain_id: u64, token_contract: Pubkey, token_id: u64, name: String, uri: String)]
pub struct Register<'info> {
    #[account(mut)]
    pub registry: Account<'info, RegistryState>,
    
    #[account(
        init,
        payer = payer,
        space = IPData::INIT_SPACE,
        seeds = [
            b"ip",
            &(chain_id as u16).to_le_bytes(),
            token_contract.as_ref(),
            &token_id.to_le_bytes()
        ],
        bump
    )]
    pub ip_data: Account<'info, IPData>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(
        mut,
        constraint = payer_token_account.owner == payer.key(),
        constraint = payer_token_account.mint == registry.fee_token,
    )]
    pub payer_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = treasury_token_account.owner == registry.treasury,
        constraint = treasury_token_account.mint == registry.fee_token,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Register<'info> {
    pub fn register(
        &mut self,
        chain_id: u64,
        token_contract: Pubkey,
        token_id: u64,
        name: String,
        uri: String,
    ) -> Result<()> {
        let registry = &mut self.registry;
        let ip_data = &mut self.ip_data;
        let signer = self.payer.key();
        let clock = Clock::get()?;

        // process payment if fee is required
        if registry.fee_amount > 0 {
            let token_program = &self.token_program;
            let source_token_account = &self.payer_token_account;
            let destination_token_account = &self.treasury_token_account;
            
            let cpi_accounts = token::Transfer {
                from: source_token_account.to_account_info(),
                to: destination_token_account.to_account_info(),
                authority: self.payer.to_account_info(),
            };
            
            let cpi_program = token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            
            token::transfer(cpi_ctx, registry.fee_amount)?;
        }

        // init the IP
        ip_data.owner = signer;
        ip_data.registry = registry.key();
        ip_data.chain_id = chain_id;
        ip_data.token_contract = token_contract;
        ip_data.token_id = token_id;
        ip_data.name = name;
        ip_data.uri = uri;
        ip_data.registration_date = clock.unix_timestamp as u64;
        ip_data.is_registered = true;

        registry.ip_count += 1;

        Ok(())
    }
}

// #[derive(Clone)]
// pub struct RegisterBumps {
//     pub ip_data: u8,
// }
