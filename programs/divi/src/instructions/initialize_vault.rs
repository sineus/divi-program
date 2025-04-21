use anchor_lang::prelude::*;

use crate::{constants::{ANCHOR_DISCRIMINATOR, VAULT, VAULT_AUTHORITY}, events::VaultCreated, states::PaymentVault};

pub fn handler(
    ctx: Context<InitializeVault>,
    payment_id: u32,
    total_amount: u64,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    vault.issuer = ctx.accounts.issuer.key();
    vault.total_amount = total_amount;
    vault.is_finalized = false;
    vault.payment_id = payment_id;
    vault.bump = ctx.bumps.vault;
    vault.authority = ctx.accounts.vault_authority.key();

    emit!(VaultCreated {
        issuer: ctx.accounts.issuer.key().clone(),
        payment_id: payment_id.clone(),
        bump: ctx.bumps.vault.clone(),
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    payment_id: u32
)]
pub struct InitializeVault<'info> {
    /// CHECK: Payment issuer
    #[account(mut, signer)]
    pub issuer: Signer<'info>,

    /// CHECK: Payment vault account
    #[account(
        init, 
        payer = issuer,
        space = ANCHOR_DISCRIMINATOR + PaymentVault::INIT_SPACE,
        seeds = [
            VAULT.as_bytes(),
            issuer.key().as_ref(),
            &payment_id.to_le_bytes(),
        ],
        bump,
    )]
    pub vault: Account<'info, PaymentVault>,
    
    /// CHECK: Vault authority - a PDA that will have authority over the vault
    #[account(
        seeds = [
            VAULT_AUTHORITY.as_bytes(),
            issuer.key().as_ref(),
            &payment_id.to_le_bytes(),
        ],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>
}