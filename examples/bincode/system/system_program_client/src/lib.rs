#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};
    use solana_program::{pubkey::Pubkey, system_instruction, sysvar};
    use system_program_interface::{
        advance_nonce_account_ix, allocate_ix, allocate_with_seed_ix, assign_ix,
        assign_with_seed_ix, authorize_nonce_account_ix, create_account_ix,
        create_account_with_seed_ix, initialize_nonce_account_ix, transfer_ix,
        transfer_with_seed_ix, upgrade_nonce_account_ix, withdraw_nonce_account_ix,
        AdvanceNonceAccountKeys, AllocateIxArgs, AllocateKeys, AllocateWithSeedIxArgs,
        AllocateWithSeedKeys, AssignIxArgs, AssignKeys, AssignWithSeedIxArgs, AssignWithSeedKeys,
        AuthorizeNonceAccountIxArgs, AuthorizeNonceAccountKeys, CreateAccountIxArgs,
        CreateAccountKeys, CreateAccountWithSeedIxArgs, CreateAccountWithSeedKeys,
        InitializeNonceAccountIxArgs, InitializeNonceAccountKeys, TransferIxArgs, TransferKeys,
        TransferWithSeedIxArgs, TransferWithSeedKeys, UpgradeNonceAccountKeys,
        WithdrawNonceAccountIxArgs, WithdrawNonceAccountKeys,
    };

    #[test]
    fn system_program_check_ix_serde() {
        let pk0 = Pubkey::new_unique();
        let pk1 = Pubkey::new_unique();
        let pk2 = Pubkey::new_unique();
        let pk3 = Pubkey::new_unique();

        let mut rng = thread_rng();
        let u640: u64 = rng.gen();
        let u641: u64 = rng.gen();
        let seed = rng.gen::<u64>().to_string();

        for (actual, expected) in [
            (
                &create_account_ix(
                    CreateAccountKeys { from: pk0, to: pk1 },
                    CreateAccountIxArgs {
                        lamports: u640,
                        space: u641,
                        owner: pk2,
                    },
                ),
                &system_instruction::create_account(&pk0, &pk1, u640, u641, &pk2),
            ),
            (
                &assign_ix(AssignKeys { assign: pk0 }, AssignIxArgs { owner: pk1 }),
                &system_instruction::assign(&pk0, &pk1),
            ),
            (
                &transfer_ix(
                    TransferKeys { from: pk0, to: pk1 },
                    TransferIxArgs { lamports: u640 },
                ),
                &system_instruction::transfer(&pk0, &pk1, u640),
            ),
            (
                &create_account_with_seed_ix(
                    CreateAccountWithSeedKeys {
                        from: pk0,
                        to: pk1,
                        base: pk2,
                    },
                    CreateAccountWithSeedIxArgs {
                        base: pk2,
                        seed: seed.clone(),
                        lamports: u640,
                        space: u641,
                        owner: pk3,
                    },
                ),
                &system_instruction::create_account_with_seed(
                    &pk0, &pk1, &pk2, &seed, u640, u641, &pk3,
                ),
            ),
            (
                &advance_nonce_account_ix(AdvanceNonceAccountKeys {
                    nonce: pk0,
                    recent_blockhashes: sysvar::recent_blockhashes::ID,
                    authority: pk1,
                }),
                &system_instruction::advance_nonce_account(&pk0, &pk1),
            ),
            (
                &withdraw_nonce_account_ix(
                    WithdrawNonceAccountKeys {
                        nonce: pk0,
                        to: pk2,
                        recent_blockhashes: sysvar::recent_blockhashes::ID,
                        rent: sysvar::rent::ID,
                        authority: pk1,
                    },
                    WithdrawNonceAccountIxArgs { lamports: u640 },
                ),
                &system_instruction::withdraw_nonce_account(&pk0, &pk1, &pk2, u640),
            ),
            (
                &initialize_nonce_account_ix(
                    InitializeNonceAccountKeys {
                        nonce: pk0,
                        recent_blockhashes: sysvar::recent_blockhashes::ID,
                        rent: sysvar::rent::ID,
                    },
                    InitializeNonceAccountIxArgs { authority: pk1 },
                ),
                &system_instruction::create_nonce_account(&pk2, &pk0, &pk1, u640)[1],
            ),
            (
                &authorize_nonce_account_ix(
                    AuthorizeNonceAccountKeys {
                        nonce: pk0,
                        authority: pk1,
                    },
                    AuthorizeNonceAccountIxArgs { new_authority: pk2 },
                ),
                &system_instruction::authorize_nonce_account(&pk0, &pk1, &pk2),
            ),
            (
                &allocate_ix(
                    AllocateKeys { allocate: pk0 },
                    AllocateIxArgs { space: u640 },
                ),
                &system_instruction::allocate(&pk0, u640),
            ),
            (
                &allocate_with_seed_ix(
                    AllocateWithSeedKeys {
                        allocate: pk0,
                        base: pk1,
                    },
                    AllocateWithSeedIxArgs {
                        base: pk1,
                        seed: seed.clone(),
                        space: u640,
                        owner: pk2,
                    },
                ),
                &system_instruction::allocate_with_seed(&pk0, &pk1, &seed, u640, &pk2),
            ),
            (
                &assign_with_seed_ix(
                    AssignWithSeedKeys {
                        assign: pk0,
                        base: pk1,
                    },
                    AssignWithSeedIxArgs {
                        base: pk1,
                        seed: seed.clone(),
                        owner: pk2,
                    },
                ),
                &system_instruction::assign_with_seed(&pk0, &pk1, &seed, &pk2),
            ),
            (
                &transfer_with_seed_ix(
                    TransferWithSeedKeys {
                        from: pk0,
                        base: pk1,
                        to: pk2,
                    },
                    TransferWithSeedIxArgs {
                        lamports: u640,
                        from_seed: seed.clone(),
                        from_owner: pk3,
                    },
                ),
                &system_instruction::transfer_with_seed(&pk0, &pk1, seed.clone(), &pk3, &pk2, u640),
            ),
            (
                &upgrade_nonce_account_ix(UpgradeNonceAccountKeys { nonce: pk0 }),
                &system_instruction::upgrade_nonce_account(pk0),
            ),
        ] {
            assert_eq!(actual, expected);
        }
    }
}
