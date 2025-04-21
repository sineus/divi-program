use crate::{
    constants::{ANCHOR_DISCRIMINATOR, PARTICIPANT_VAULT, PARTICIPANT_VAULT_AUTHORITY, VAULT},
    states::{ParticipantVault, PaymentVault},
};
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<CreateParticipantVault>, payment_id: u32, amount: u64) -> Result<()> {
    let participant_vault = &mut ctx.accounts.participant_vault;

    // Intialize the participant vault
    participant_vault.participant = ctx.accounts.participant.key();
    participant_vault.amount = amount;
    participant_vault.payment_id = payment_id;
    participant_vault.issuer = ctx.accounts.vault.issuer;
    participant_vault.bump = ctx.bumps.participant_vault;

    // Transfer SOL from participant wallet to participant vault authority
    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.participant.to_account_info(),
                to: ctx.accounts.participant_vault_authority.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(payment_id: u32, amount: u64)]
pub struct CreateParticipantVault<'info> {
    /// CHECK: Payment participant
    #[account(mut)]
    pub participant: Signer<'info>,

    /// CHECK: Main payment vault (from the issuer)
    #[account(
      mut,
      seeds = [VAULT.as_bytes(), vault.issuer.as_ref(), &payment_id.to_le_bytes()],
      bump = vault.bump,
  )]
    pub vault: Account<'info, PaymentVault>,

    /// CHECK: Participant vault
    #[account(
      init,
      payer = participant,
      space = ANCHOR_DISCRIMINATOR + ParticipantVault::INIT_SPACE,
      seeds = [
          PARTICIPANT_VAULT.as_bytes(),
          participant.key().as_ref(),
          &payment_id.to_le_bytes(),
      ],
      bump,
    )]
    pub participant_vault: Account<'info, ParticipantVault>,

    /// CHECK: Participant vault authority (funds)
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
