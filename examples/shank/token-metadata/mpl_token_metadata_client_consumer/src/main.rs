use mpl_token_metadata_interface::*;
use solana_cli_config::{Config, CONFIG_FILE};
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file, signer::Signer};

fn main() {
    let config_file = CONFIG_FILE.as_ref().unwrap();
    let cli_config = Config::load(&config_file).unwrap();
    let kp = read_keypair_file(cli_config.keypair_path).unwrap();

    let accounts = RemoveCreatorVerificationKeys {
        creator: &kp.pubkey(),
        metadata: &Pubkey::new_unique(),
    };
    let args = RemoveCreatorVerificationIxArgs {};
    let remove_creator_ver_ix = remove_creator_verification_ix(accounts, &args).unwrap();
    println!("{:?}", remove_creator_ver_ix);
}
