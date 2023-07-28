use borsh::BorshDeserialize;
use lazy_static::lazy_static;
use marinade_finance_interface::{State, STATE_ACCOUNT_DISCM};
use solana_client::nonblocking::rpc_client::RpcClient;

lazy_static! {
    static ref RPC: RpcClient =
        RpcClient::new("https://solana-mainnet.rpc.extrnode.com".to_owned());
}

mod marinade_state {
    solana_sdk::declare_id!("8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC");
}

#[tokio::test]
async fn test_read_serde_marinade_state() {
    let acc = RPC.get_account_data(&marinade_state::ID).await.unwrap();
    let mut reader = acc.as_slice();

    let discm = <[u8; 8]>::deserialize(&mut reader).unwrap();
    assert_eq!(discm, STATE_ACCOUNT_DISCM);
    let s = State::deserialize(&mut reader).unwrap();
    serde_json::to_string(&s).unwrap();
}
