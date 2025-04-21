use anchor_lang::prelude::*;

use crate::{
    constants::{LAMPORTS_PER_SOL, VAULT, VAULT_AUTHORITY},
    errors::DiviError,
    events::{ParticipantPaid, VaultCompleted},
    states::PaymentVault,
};

pub fn handler(ctx: Context<Pay>, payment_id: u32, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let vault_authority_lamports = ctx.accounts.vault_authority.lamports();
    let amount_lamports = amount * LAMPORTS_PER_SOL;
    let remaining_amount_lamports = vault.total_amount - vault_authority_lamports;

    // Check if the amount is not greater than the vault total amount
    require!(
        amount_lamports <= vault.total_amount,
        DiviError::AmountIsGreaterThanVaultTotalAmount
    );

    // Check if amount in lamports is greater than the remaining amount needed
    require!(
        amount_lamports <= remaining_amount_lamports,
        DiviError::AmountIsGreaterThanRemainingVaultAmount
    );

    // Transfer SOL from payer to vault_authority using System Program
    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.vault_authority.to_account_info(),
            },
        ),
        amount_lamports,
    )?;

    emit!(ParticipantPaid {
        issuer: ctx.accounts.issuer.key(),
        payer: ctx.accounts.payer.key(),
        payment_id,
        bump: vault.bump,
    });

    // Check if vault is fully funded after this transfer
    if vault_authority_lamports + amount_lamports >= vault.total_amount {
        vault.is_finalized = true;

        emit!(VaultCompleted {
            issuer: ctx.accounts.issuer.key(),
            payment_id,
            bump: vault.bump,
        });
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    payment_id: u32
)]
pub struct Pay<'info> {
    /// CHECK: Participant payer
    #[account(mut, signer)]
    pub payer: Signer<'info>,

    /// CHECK: Payment issuer
    pub issuer: AccountInfo<'info>,

    /// CHECK: Payment vault account for metadata
    #[account(
        mut,
        seeds = [
            VAULT.as_bytes(),
            issuer.key().as_ref(),
            &payment_id.to_le_bytes(),
        ],
        bump,
        constraint = !vault.is_finalized @ DiviError::VaultIsAlreadyFinalized,
    )]
    pub vault: Account<'info, PaymentVault>,

    /// CHECK: This is a PDA that will hold the funds
    #[account(
        mut,
        seeds = [
            VAULT_AUTHORITY.as_bytes(),
            issuer.key().as_ref(),
            &payment_id.to_le_bytes(),
        ],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}
