use anchor_lang::prelude::*;

#[event]
pub struct VaultCreated {
    pub issuer: Pubkey,
    pub payment_id: u32,
    pub bump: u8,
}

#[event]
pub struct ParticipantPaid {
    pub issuer: Pubkey,
    pub payer: Pubkey,
    pub payment_id: u32,
    pub bump: u8,
}

#[event]
pub struct VaultCompleted {
    pub issuer: Pubkey,
    pub payment_id: u32,
    pub bump: u8,
}

#[event]
pub struct VaultCancelled {
    pub issuer: Pubkey,
    pub payment_id: u32,
    pub bump: u8,
}

#[event]
pub struct ParticipantRefunded {
    pub issuer: Pubkey,
    pub payment_id: u32,
    pub bump: u8,
    pub participant: Pubkey,
}
