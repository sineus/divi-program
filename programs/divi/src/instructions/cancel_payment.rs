use anchor_lang::prelude::*;

use crate::{
    constants::{VAULT, VAULT_AUTHORITY},
    errors::DiviError,
    events::VaultCancelled,
    states::PaymentVault,
    utils::is_valid_participant_vault,
};

pub fn handler(ctx: Context<CancelPayment>, payment_id: u32) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    // Check if all payment participant account are closed
    if !ctx.remaining_accounts.is_empty() {
        for account_info in ctx.remaining_accounts.iter() {
            if is_valid_participant_vault(account_info, payment_id, ctx.program_id)? {
                return err!(DiviError::NotAllParticipantsRefunded);
            }
        }
    }

    vault.is_cancelled = true;

    emit!(VaultCancelled {
        issuer: ctx.accounts.issuer.key().clone(),
        payment_id: payment_id.clone(),
        bump: vault.bump.clone(),
    });

    msg!("Payment {} cancelled by issuer", payment_id);

    Ok(())
}

#[derive(Accounts)]
#[instruction(payment_id: u32)]
pub struct CancelPayment<'info> {
    #[account(mut)]
    pub issuer: Signer<'info>,

    #[account(
      mut,
      seeds = [
          VAULT.as_bytes(),
          issuer.key().as_ref(),
          &payment_id.to_le_bytes(),
      ],
      bump = vault.bump,
      constraint = vault.issuer == issuer.key() @ DiviError::InvalidVaultAuthority,
  )]
    pub vault: Account<'info, PaymentVault>,

    #[account(
      seeds = [
          VAULT_AUTHORITY.as_bytes(),
          issuer.key().as_ref(),
          &payment_id.to_le_bytes(),
      ],
      bump,
  )]
    /// CHECK: PDA for vault authority
    pub vault_authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}
