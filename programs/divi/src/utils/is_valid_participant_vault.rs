use anchor_lang::prelude::*;

use crate::states::ParticipantVault;

pub fn is_valid_participant_vault(
    account_info: &AccountInfo,
    payment_id: u32,
    program_id: &Pubkey,
) -> Result<bool> {
    // Vérifier d'abord le propriétaire du compte
    if account_info.owner != program_id {
        return Ok(false);
    }

    // Essayer de désérialiser le compte (incluant le discriminant)
    let data = account_info.try_borrow_data()?;
    if data.len() <= 8 {
        return Ok(false);
    }

    // Désérialiser les données brutes
    let mut data_slice = &data[8..]; // Sauter le discriminant
    let vault = match ParticipantVault::deserialize(&mut data_slice) {
        Ok(v) => v,
        Err(_) => return Ok(false),
    };

    // Vérifier le payment_id
    Ok(vault.payment_id == payment_id)
}
