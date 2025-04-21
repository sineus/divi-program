use anchor_lang::prelude::*;

declare_id!("4pYKM8QS4SV7ffF8bMjjPhZtQfb8R8Q5M72GbnN5oKFs");

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod states;
pub mod utils;

use instructions::*;

#[program]
pub mod divi {
    use super::*;

    /// Initialize a payment vault that hosts payment infos and primary payer
    ///
    /// ### Parameters
    /// - `payment_id` - Unique payment ID to find vault PDA
    /// - `total_amount` - This is the total amount that the primary payer must pay (alone or with other participants)
    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        payment_id: u32,
        total_amount: u64,
    ) -> Result<()> {
        return instructions::initialize_vault::handler(ctx, payment_id, total_amount);
    }

    /// Participant pay his share
    ///
    /// ### Parameters
    /// - `payment_id` - Unique payment ID to find vault PDA
    /// - `amount` - Amount to transfert
    pub fn pay(ctx: Context<Pay>, payment_id: u32, amount: u64) -> Result<()> {
        return instructions::pay::handler(ctx, payment_id, amount);
    }

    /// Close the vault and transfert the lamports to the issuer
    ///
    /// ### Parameters
    /// - `payment_id` - Unique payment ID to find vault PDA
    pub fn close_vault(ctx: Context<CloseVault>, payment_id: u32) -> Result<()> {
        return instructions::close_vault::handler(ctx, payment_id);
    }

    /// Participate to payment with vault
    ///
    /// ### Parameters
    /// - `payment_id` - Unique payment ID to find vault PDA
    pub fn participate(
        ctx: Context<CreateParticipantVault>,
        payment_id: u32,
        amount: u64,
    ) -> Result<()> {
        return instructions::create_participant_vault::handler(ctx, payment_id, amount);
    }

    /// Close the vault and transfert the lamports to the issuer
    ///
    /// ### Parameters
    /// - `payment_id` - Unique payment ID to find vault PDA
    pub fn cancel_payment(ctx: Context<CancelPayment>, payment_id: u32) -> Result<()> {
        return instructions::cancel_payment::handler(ctx, payment_id);
    }

    /// Close the vault and transfert the lamports to the issuer
    ///
    /// ### Parameters
    /// - `payment_id` - Unique payment ID to find vault PDA
    pub fn close_payment_vault(ctx: Context<ClosePaymentVault>, payment_id: u32) -> Result<()> {
        return instructions::close_payment_vault::handler(ctx, payment_id);
    }

    /// Refund a payer participant
    ///
    /// ### Parameters
    /// - `payment_id` - Unique payment ID to find vault PDA
    pub fn refund_participant(ctx: Context<RefundParticipant>, payment_id: u32) -> Result<()> {
        return instructions::refund_participant::handler(ctx, payment_id);
    }
}
