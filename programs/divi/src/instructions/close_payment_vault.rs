use anchor_lang::prelude::*;

use crate::{
    constants::VAULT, errors::DiviError, events::VaultCompleted, states::PaymentVault,
    utils::is_valid_participant_vault,
};

pub fn handler(ctx: Context<ClosePaymentVault>, payment_id: u32) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    vault.is_finalized = true;

    msg!("Payment vault {} closed successfully", payment_id);

    emit!(VaultCompleted {
        issuer: ctx.accounts.issuer.key().clone(),
        payment_id: payment_id.clone(),
        bump: vault.bump.clone(),
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(payment_id: u32)]
pub struct ClosePaymentVault<'info> {
    #[account(mut)]
    pub issuer: Signer<'info>,

    #[account(
        mut,
        close = issuer,
        seeds = [
            VAULT.as_bytes(),
            issuer.key().as_ref(),
            &payment_id.to_le_bytes(),
        ],
        bump = vault.bump,
        constraint = vault.issuer == issuer.key() @ DiviError::InvalidVaultAuthority,
        constraint = !vault.is_finalized @ DiviError::PaymentAlreadyFinalized,
    )]
    pub vault: Account<'info, PaymentVault>,

    pub system_program: Program<'info, System>,
}
