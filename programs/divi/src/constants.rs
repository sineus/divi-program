use anchor_lang::prelude::*;

#[constant]
pub const VAULT: &str = "divi-vault";

#[constant]
pub const VAULT_AUTHORITY: &str = "divi-vault-authority";

#[constant]
pub const PARTICIPANT_VAULT: &str = "participant_vault";

#[constant]
pub const PARTICIPANT_VAULT_AUTHORITY: &str = "participant_vault_authority";

pub const ANCHOR_DISCRIMINATOR: usize = 8;
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;
