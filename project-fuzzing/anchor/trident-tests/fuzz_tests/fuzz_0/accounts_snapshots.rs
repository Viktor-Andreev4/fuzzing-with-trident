use anchor_lang::prelude::*;
use trident_client::fuzzing::{anchor_lang, FuzzingError};
pub struct TransferSolWithCpiSnapshot<'info> {
    pub payer: Signer<'info>,
    pub recipient: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
pub struct TransferSolWithProgramSnapshot<'info> {
    pub payer: UncheckedAccount<'info>,
    pub recipient: SystemAccount<'info>,
}
impl<'info> TransferSolWithCpiSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let payer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("payer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("payer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("payer".to_string()))?;
        let recipient: SystemAccount<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("recipient".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::system_account::SystemAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("recipient".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("recipient".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            payer,
            recipient,
            system_program,
        })
    }
}
impl<'info> TransferSolWithProgramSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let payer = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("payer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::unchecked_account::UncheckedAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("payer".to_string()))?;
        let recipient: SystemAccount<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("recipient".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::system_account::SystemAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("recipient".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("recipient".to_string()))?;
        Ok(Self { payer, recipient })
    }
}
