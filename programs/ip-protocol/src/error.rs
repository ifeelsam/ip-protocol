use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized operation")]
    Unauthorized,
    
    #[msg("IP already registered")]
    AlreadyRegistered,
    
    #[msg("Invalid fee parameters")]
    InvalidFeeParameters,
}