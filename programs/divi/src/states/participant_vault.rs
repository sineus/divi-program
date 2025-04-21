use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct ParticipantVault {
    // The issuer of the main payment vault
    pub issuer: Pubkey,

    // The payment id
    pub payment_id: u32,

    // The current participant
    pub participant: Pubkey,

    // The amount paid
    pub amount: u64,

    // The bump
    pub bump: u8,
}
