use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct RegistryState {
    pub authority: Pubkey,
    pub ip_count: u64,
    pub treasury: Pubkey,
    pub fee_token: Pubkey,
    pub fee_amount: u64,
    pub ip_account_implementation: Pubkey,
}

// impl RegistryState {
//     pub const LEN: usize = 8 + // discriminator
//         32 + // authority
//         8 +  // ip_count
//         32 + // treasury
//         32 + // fee_token
//         8 +  // fee_amount
//         32;  // ip_account_implementation
// }

#[account]
#[derive(InitSpace)]
pub struct IPData {
    pub owner: Pubkey,
    pub registry: Pubkey,
    pub chain_id: u64,
    pub token_contract: Pubkey,
    pub token_id: u64,
    #[max_len(32)]
    pub name: String,
    #[max_len(256)]
    pub uri: String,
    pub registration_date: u64,
    pub is_registered: bool,
}

// impl IPData {
//     pub fn calculate_size(name: &str, uri: &str) -> usize {
//         8 +  // discriminator
//         32 + // owner
//         32 + // registry
//         8 +  // chain_id
//         32 + // token_contract
//         8 +  // token_id
//         4 + name.len() + // name (string with length prefix)
//         4 + uri.len() +  // uri (string with length prefix)
//         8 +  // registration_date
//         1    // is_registered
//     }
// }
