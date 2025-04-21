use crate::{
    constants::{PARTICIPANT_VAULT, PARTICIPANT_VAULT_AUTHORITY, VAULT},
    errors::DiviError,
    events::ParticipantRefunded,
    states::{ParticipantVault, PaymentVault},
};
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<RefundParticipant>, payment_id: u32) -> Result<()> {
    let vault = &ctx.accounts.vault;
    let participant_key = ctx.accounts.participant.key();

    // Check if main payment vault is not finalized
    require!(!vault.is_finalized, DiviError::PaymentAlreadyFinalized);

    // Prepare seeds for the vault authority
    let authority_seeds = &[
        PARTICIPANT_VAULT_AUTHORITY.as_bytes(),
        participant_key.as_ref(),
        &payment_id.to_le_bytes(),
        &[ctx.bumps.participant_vault_authority],
    ];
    let signer_seeds = &[&authority_seeds[..]];

    // Transfer participant vault authority funds to participant
    let vault_balance = ctx.accounts.participant_vault_authority.lamports();

    if vault_balance > 0 {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.participant_vault_authority.key(),
            &ctx.accounts.participant.key(),
            vault_balance,
        );

        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &[
                ctx.accounts.participant_vault_authority.to_account_info(),
                ctx.accounts.participant.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            signer_seeds,
        )?;

        msg!(
            "Refunded {} lamports to participant {}",
            vault_balance,
            ctx.accounts.participant.key()
        );
    }

    // Close the participant vault and send the rent to the participant
    ctx.accounts
        .participant_vault
        .close(ctx.accounts.participant.to_account_info())?;

    emit!(ParticipantRefunded {
        issuer: ctx.accounts.issuer.key().clone(),
        payment_id: payment_id.clone(),
        bump: vault.bump.clone(),
        participant: participant_key.clone()
    });

    msg!(
        "Successfully closed participant vault for {}",
        ctx.accounts.participant.key()
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(payment_id: u32)]
pub struct RefundParticipant<'info> {
    /// CHECK: The main payment vault issuer
    #[account(
        mut,
        constraint = issuer.key() == vault.issuer @ DiviError::InvalidVaultAuthority
    )]
    pub issuer: Signer<'info>,

    /// CHECK: This is the participant receiving funds
    #[account(mut)]
    pub participant: UncheckedAccount<'info>,

    /// CHECK: The main payment vault
    #[account(
        seeds = [
            VAULT.as_bytes(),
            vault.issuer.as_ref(),
            &payment_id.to_le_bytes(),
        ],
        bump = vault.bump,
        constraint = !vault.is_finalized @ DiviError::PaymentAlreadyFinalized,
    )]
    pub vault: Account<'info, PaymentVault>,

    /// CHECK: The participant vault
    #[account(
        mut,
        seeds = [
            PARTICIPANT_VAULT.as_bytes(),
            participant.key().as_ref(),
            &payment_id.to_le_bytes(),
        ],
        bump,
        constraint = participant_vault.participant == participant.key() @ DiviError::InvalidParticipant,
        constraint = participant_vault.payment_id == payment_id @ DiviError::InvalidPaymentId,
    )]
    pub participant_vault: Account<'info, ParticipantVault>,

    /// CHECK: The participant vault authority (funds)
    #[account(
        mut,
        seeds = [
            PARTICIPANT_VAULT_AUTHORITY.as_bytes(),
            participant.key().as_ref(),
            &payment_id.to_le_bytes(),
        ],
        bump,
    )]
    pub participant_vault_authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}
