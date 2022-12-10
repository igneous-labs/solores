use solana_program::{account_info::AccountInfo, instruction::AccountMeta, pubkey::Pubkey};

#[derive(Copy, Clone, Debug)]
pub struct TransferAccounts<'me, 'a1: 'me, 'a2: 'me> {
    pub src: &'me AccountInfo<'a1>,
    pub dest: &'me AccountInfo<'a2>,
}

#[derive(Copy, Clone, Debug)]
pub struct TransferKeys<'me> {
    pub src: &'me Pubkey,
    pub dest: &'me Pubkey,
}

impl<'me> From<&TransferAccounts<'me, '_, '_>> for TransferKeys<'me> {
    fn from(transfer_accounts: &TransferAccounts<'me, '_, '_>) -> Self {
        Self {
            src: transfer_accounts.src.key,
            dest: transfer_accounts.dest.key,
        }
    }
}

impl From<&TransferKeys<'_>> for [AccountMeta; 2] {
    fn from(transfer_keys: &TransferKeys<'_>) -> Self {
        [
            AccountMeta::new(*transfer_keys.src, true),
            AccountMeta::new(*transfer_keys.dest, false),
        ]
    }
}

impl<'a> From<&TransferAccounts<'_, 'a, 'a>> for [AccountInfo<'a>; 2] {
    fn from(transfer_accounts: &TransferAccounts<'_, 'a, 'a>) -> Self {
        [
            transfer_accounts.src.clone(),
            transfer_accounts.dest.clone(),
        ]
    }
}
