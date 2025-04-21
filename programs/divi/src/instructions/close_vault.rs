use crate::{
    constants::{VAULT, VAULT_AUTHORITY},
    errors::DiviError,
    states::PaymentVault,
};
use anchor_lang::prelude::*;

/**
* Close the vault and send SOL amount and rent to the issuer
*/
pub fn handler(ctx: Context<CloseVault>, payment_id: u32) -> Result<()> {
    let issuer_key = ctx.accounts.issuer.key();

    // Get the PDA signer seeds for vault_authority
    let authority_seeds = &[
        VAULT_AUTHORITY.as_bytes(),
        issuer_key.as_ref(),
        &payment_id.to_le_bytes(),
        &[ctx.bumps.vault_authority],
    ];
    let signer_seeds = &[&authority_seeds[..]];

    // Get vault authority balance (minus rent for safety)
    let vault_authority = &ctx.accounts.vault_authority;
    let rent = Rent::get()?;
    let min_rent = rent.minimum_balance(0);
    let transfer_amount = vault_authority.lamports().saturating_sub(min_rent);

    // Transfer SOL from vault_authority to issuer
    if transfer_amount > 0 {
        anchor_lang::system_program::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: vault_authority.to_account_info(),
                    to: ctx.accounts.issuer.to_account_info(),
                },
                signer_seeds,
            ),
            transfer_amount,
        )?;

        msg!(
            "Transferred {} lamports from vault authority to issuer",
            transfer_amount
        );
    }

    msg!("Vault {} closed successfully", payment_id);
    Ok(())
}

#[derive(Accounts)]
#[instruction(
payment_id: u32
)]
pub struct CloseVault<'info> {
    /// Payment issuer who receives the funds
    #[account(mut)]
    pub issuer: Signer<'info>,

    /// Payment vault account with metadata
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
      constraint = vault.is_finalized == true @ DiviError::VaultIsNotFinalized,
  )]
    pub vault: Account<'info, PaymentVault>,

    /// Vault authority - a PDA that holds the SOL funds
    #[account(
      mut,
      seeds = [
          VAULT_AUTHORITY.as_bytes(),
          issuer.key().as_ref(),
          &payment_id.to_le_bytes(),
      ],
      bump,
  )]
    /// CHECK: This is a PDA that holds the funds
    pub vault_authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}
