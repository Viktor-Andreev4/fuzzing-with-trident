pub mod transfer_sol_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        TransferSolWithCpi(TransferSolWithCpi),
        TransferSolWithProgram(TransferSolWithProgram),
    }
    #[derive(Arbitrary, Debug)]
    pub struct TransferSolWithCpi {
        pub accounts: TransferSolWithCpiAccounts,
        pub data: TransferSolWithCpiData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct TransferSolWithCpiAccounts {
        pub payer: AccountId,
        pub recipient: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct TransferSolWithCpiData {
        pub amount: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct TransferSolWithProgram {
        pub accounts: TransferSolWithProgramAccounts,
        pub data: TransferSolWithProgramData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct TransferSolWithProgramAccounts {
        pub payer: AccountId,
        pub recipient: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct TransferSolWithProgramData {
        pub amount: u64,
    }
    impl<'info> IxOps<'info> for TransferSolWithCpi {
        type IxData = transfer_sol::instruction::TransferSolWithCpi;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = TransferSolWithCpiSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = transfer_sol::instruction::TransferSolWithCpi { amount: self.data.amount };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {

            let payer = fuzz_accounts.
                payer.get_or_create_account(self.accounts.payer, client, 10*LAMPORTS_PER_SOL);

            let recipient = fuzz_accounts.
                recipient.get_or_create_account(self.accounts.recipient, client, 10*LAMPORTS_PER_SOL);

            let signers = vec![payer.clone(), recipient.clone()];
            let acc_meta = transfer_sol::accounts::TransferSolWithCpi {
                payer: payer.pubkey(),
                recipient: recipient.pubkey(),
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for TransferSolWithProgram {
        type IxData = transfer_sol::instruction::TransferSolWithProgram;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = TransferSolWithProgramSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = transfer_sol::instruction::TransferSolWithProgram { amount: self.data.amount };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let payer = fuzz_accounts.
                payer.get_or_create_account(self.accounts.payer, client, 10*LAMPORTS_PER_SOL);

            let recipient = fuzz_accounts.
                recipient.get_or_create_account(self.accounts.recipient, client, 10*LAMPORTS_PER_SOL);

            let signers = vec![payer.clone(), recipient.clone()];
            let acc_meta = transfer_sol::accounts::TransferSolWithProgram {
                payer: payer.pubkey(),
                recipient: recipient.pubkey(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        payer: AccountsStorage<Keypair>,
        recipient: AccountsStorage<Keypair>,
        // system_program: AccountsStorage<ProgramStore>,
    }
}
