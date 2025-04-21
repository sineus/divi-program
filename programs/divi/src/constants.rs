use anchor_lang::prelude::*;

#[constant]
pub const VAULT: &str = "divi-vault";

#[constant]
pub const VAULT_AUTHORITY: &str = "divi-vault-authority";

pub const ANCHOR_DISCRIMINATOR: usize = 8;
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;
