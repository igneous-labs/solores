#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};
    use solana_program::{
        pubkey::Pubkey,
        stake::{self, instruction::LockupArgs},
        sysvar,
    };
    use stake_program_interface::{
        authorize_checked_ix, authorize_checked_with_seed_ix, authorize_ix, authorize_with_seed_ix,
        deactivate_delinquent_ix, deactivate_ix, delegate_stake_ix, get_minimum_delegation_ix,
        initialize_checked_ix, initialize_ix, merge_ix, redelegate_ix, set_lockup_checked_ix,
        set_lockup_ix, split_ix, withdraw_ix, AuthorizeCheckedIxArgs, AuthorizeCheckedKeys,
        AuthorizeCheckedWithSeedIxArgs, AuthorizeCheckedWithSeedKeys, AuthorizeIxArgs,
        AuthorizeKeys, AuthorizeWithSeedIxArgs, AuthorizeWithSeedKeys, Authorized,
        DeactivateDelinquentKeys, DeactivateKeys, DelegateStakeKeys, InitializeCheckedKeys,
        InitializeIxArgs, InitializeKeys, Lockup, MergeKeys, RedelegateKeys,
        SetLockupCheckedIxArgs, SetLockupCheckedKeys, SetLockupIxArgs, SetLockupKeys, SplitIxArgs,
        SplitKeys, StakeAuthorize, WithdrawIxArgs, WithdrawKeys,
    };

    fn conv_stake_authorize(s: &StakeAuthorize) -> stake::state::StakeAuthorize {
        match s {
            StakeAuthorize::Staker => stake::state::StakeAuthorize::Staker,
            StakeAuthorize::Withdrawer => stake::state::StakeAuthorize::Withdrawer,
        }
    }

    #[test]
    fn stake_program_check_ix_serde() {
        let pk0 = Pubkey::new_unique();
        let pk1 = Pubkey::new_unique();
        let pk2 = Pubkey::new_unique();
        let pk3 = Pubkey::new_unique();

        let mut rng = thread_rng();
        let u640: u64 = rng.gen();
        let i640: i64 = rng.gen();
        let stake_authorize = match rng.gen::<bool>() {
            true => StakeAuthorize::Staker,
            false => StakeAuthorize::Withdrawer,
        };
        let seed = rng.gen::<u64>().to_string();

        for (actual, expected) in [
            (
                &initialize_ix(
                    InitializeKeys {
                        stake: pk0,
                        rent: sysvar::rent::ID,
                    },
                    InitializeIxArgs {
                        authorized: Authorized {
                            staker: pk1,
                            withdrawer: pk2,
                        },
                        lockup: Lockup {
                            unix_timestamp: i640,
                            epoch: u640,
                            custodian: pk3,
                        },
                    },
                ),
                &stake::instruction::initialize(
                    &pk0,
                    &stake::state::Authorized {
                        staker: pk1,
                        withdrawer: pk2,
                    },
                    &stake::state::Lockup {
                        unix_timestamp: i640,
                        epoch: u640,
                        custodian: pk3,
                    },
                ),
            ),
            (
                &authorize_ix(
                    AuthorizeKeys {
                        stake: pk0,
                        clock: sysvar::clock::ID,
                        authority: pk1,
                    },
                    AuthorizeIxArgs {
                        new_authority: pk2,
                        stake_authorize: stake_authorize.clone(),
                    },
                ),
                &stake::instruction::authorize(
                    &pk0,
                    &pk1,
                    &pk2,
                    conv_stake_authorize(&stake_authorize),
                    None,
                ),
            ),
            (
                &delegate_stake_ix(DelegateStakeKeys {
                    stake: pk0,
                    vote: pk1,
                    clock: sysvar::clock::ID,
                    stake_history: sysvar::stake_history::ID,
                    stake_config: stake::config::ID,
                    stake_authority: pk2,
                }),
                &stake::instruction::delegate_stake(&pk0, &pk2, &pk1),
            ),
            (
                &split_ix(
                    SplitKeys {
                        from: pk0,
                        to: pk1,
                        stake_authority: pk2,
                    },
                    SplitIxArgs { lamports: u640 },
                ),
                &stake::instruction::split(&pk0, &pk2, u640, &pk1)
                    .last()
                    .unwrap(),
            ),
            (
                &withdraw_ix(
                    WithdrawKeys {
                        from: pk0,
                        to: pk1,
                        clock: sysvar::clock::ID,
                        stake_history: sysvar::stake_history::ID,
                        withdraw_authority: pk2,
                    },
                    WithdrawIxArgs { lamports: u640 },
                ),
                &stake::instruction::withdraw(&pk0, &pk2, &pk1, u640, None),
            ),
            (
                &deactivate_ix(DeactivateKeys {
                    stake: pk0,
                    clock: sysvar::clock::ID,
                    stake_authority: pk1,
                }),
                &stake::instruction::deactivate_stake(&pk0, &pk1),
            ),
            (
                &set_lockup_ix(
                    SetLockupKeys {
                        stake: pk0,
                        authority: pk1,
                    },
                    SetLockupIxArgs {
                        unix_timestamp: Some(i640),
                        epoch: Some(u640),
                        custodian: Some(pk2),
                    },
                ),
                &stake::instruction::set_lockup(
                    &pk0,
                    &LockupArgs {
                        unix_timestamp: Some(i640),
                        epoch: Some(u640),
                        custodian: Some(pk2),
                    },
                    &pk1,
                ),
            ),
            (
                &set_lockup_ix(
                    SetLockupKeys {
                        stake: pk0,
                        authority: pk1,
                    },
                    SetLockupIxArgs {
                        unix_timestamp: Some(i640),
                        epoch: None,
                        custodian: Some(pk2),
                    },
                ),
                &stake::instruction::set_lockup(
                    &pk0,
                    &LockupArgs {
                        unix_timestamp: Some(i640),
                        epoch: None,
                        custodian: Some(pk2),
                    },
                    &pk1,
                ),
            ),
            (
                &merge_ix(MergeKeys {
                    to: pk0,
                    from: pk1,
                    clock: sysvar::clock::ID,
                    stake_history: sysvar::stake_history::ID,
                    stake_authority: pk2,
                }),
                &stake::instruction::merge(&pk0, &pk1, &pk2).last().unwrap(),
            ),
            (
                &authorize_with_seed_ix(
                    AuthorizeWithSeedKeys {
                        stake: pk0,
                        authority_base: pk1,
                        clock: sysvar::clock::ID,
                    },
                    AuthorizeWithSeedIxArgs {
                        new_authority: pk2,
                        stake_authorize: stake_authorize.clone(),
                        authority_seed: seed.clone(),
                        authority_owner: pk3,
                    },
                ),
                &stake::instruction::authorize_with_seed(
                    &pk0,
                    &pk1,
                    seed.clone(),
                    &pk3,
                    &pk2,
                    conv_stake_authorize(&stake_authorize),
                    None,
                ),
            ),
            (
                &initialize_checked_ix(InitializeCheckedKeys {
                    stake: pk0,
                    rent: sysvar::rent::ID,
                    stake_authority: pk1,
                    withdraw_authority: pk2,
                }),
                &stake::instruction::initialize_checked(
                    &pk0,
                    &stake::state::Authorized {
                        staker: pk1,
                        withdrawer: pk2,
                    },
                ),
            ),
            (
                &authorize_checked_ix(
                    AuthorizeCheckedKeys {
                        stake: pk0,
                        clock: sysvar::clock::ID,
                        authority: pk1,
                        new_authority: pk2,
                    },
                    AuthorizeCheckedIxArgs {
                        stake_authorize: stake_authorize.clone(),
                    },
                ),
                &stake::instruction::authorize_checked(
                    &pk0,
                    &pk1,
                    &pk2,
                    conv_stake_authorize(&stake_authorize),
                    None,
                ),
            ),
            (
                &authorize_checked_with_seed_ix(
                    AuthorizeCheckedWithSeedKeys {
                        stake: pk0,
                        authority_base: pk1,
                        clock: sysvar::clock::ID,
                        new_authority: pk2,
                    },
                    AuthorizeCheckedWithSeedIxArgs {
                        stake_authorize: stake_authorize.clone(),
                        authority_seed: seed.clone(),
                        authority_owner: pk3,
                    },
                ),
                &stake::instruction::authorize_checked_with_seed(
                    &pk0,
                    &pk1,
                    seed.clone(),
                    &pk3,
                    &pk2,
                    conv_stake_authorize(&stake_authorize),
                    None,
                ),
            ),
            (
                &set_lockup_checked_ix(
                    SetLockupCheckedKeys {
                        stake: pk0,
                        authority: pk1,
                    },
                    SetLockupCheckedIxArgs {
                        unix_timestamp: Some(i640),
                        epoch: Some(u640),
                    },
                ),
                &stake::instruction::set_lockup_checked(
                    &pk0,
                    &stake::instruction::LockupArgs {
                        unix_timestamp: Some(i640),
                        epoch: Some(u640),
                        custodian: None,
                    },
                    &pk1,
                ),
            ),
            (
                &get_minimum_delegation_ix(),
                &stake::instruction::get_minimum_delegation(),
            ),
            (
                &deactivate_delinquent_ix(DeactivateDelinquentKeys {
                    stake: pk0,
                    vote: pk1,
                    reference_vote: pk2,
                }),
                &stake::instruction::deactivate_delinquent_stake(&pk0, &pk1, &pk2),
            ),
            (
                &redelegate_ix(RedelegateKeys {
                    stake: pk0,
                    uninitialized_stake: pk1,
                    vote: pk2,
                    stake_config: stake::config::ID,
                    stake_authority: pk3,
                }),
                &stake::instruction::redelegate(&pk0, &pk3, &pk2, &pk1)
                    .last()
                    .unwrap(),
            ),
        ] {
            assert_eq!(actual, expected);
        }
    }
}
