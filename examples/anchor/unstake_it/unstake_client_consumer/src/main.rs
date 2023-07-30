use solana_cli_config::{Config, CONFIG_FILE};
use solana_sdk::{
    pubkey::Pubkey, signature::read_keypair_file, signer::Signer, system_program, sysvar,
};
use unstake_interface::*;

fn main() {
    let config_file = CONFIG_FILE.as_ref().unwrap();
    let cli_config = Config::load(&config_file).unwrap();
    let kp = read_keypair_file(cli_config.keypair_path).unwrap();

    let accounts = SetFeeKeys {
        fee_authority: kp.pubkey(),
        pool_account: Pubkey::new_unique(),
        fee_account: Pubkey::new_unique(),
        system_program: system_program::ID,
        rent: sysvar::rent::ID,
    };
    let args = SetFeeIxArgs {
        fee: Fee {
            fee: FeeEnum::Flat {
                ratio: Rational {
                    num: 1u64,
                    denom: 10_000u64,
                },
            },
        },
    };
    let remove_creator_ver_ix = set_fee_ix(accounts, args).unwrap();
    println!("{:?}", remove_creator_ver_ix);
}
