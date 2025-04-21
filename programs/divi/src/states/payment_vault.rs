use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct PaymentVault {
    // Payment issuer
    pub issuer: Pubkey,

    // Amount to pay
    pub total_amount: u64,

    // Flag to indicate if the total amount was paid
    pub is_finalized: bool,

    // Bump from the PDA generation
    pub bump: u8,

    // Unique ID for the PDA
    pub payment_id: u32,

    // Vault authority
    pub authority: Pubkey,
}
