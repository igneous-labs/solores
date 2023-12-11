use lazy_static::lazy_static;
use marinade_finance_interface::StateAccount;
use solana_client::nonblocking::rpc_client::RpcClient;

lazy_static! {
    static ref RPC: RpcClient = RpcClient::new("https://api.mainnet-beta.solana.com".to_owned());
}

mod marinade_state {
    solana_sdk::declare_id!("8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC");
}

#[tokio::test]
async fn test_read_serde_marinade_state() {
    let acc = RPC.get_account_data(&marinade_state::ID).await.unwrap();
    let mut buf = acc.as_slice();
    let sa = StateAccount::deserialize(&mut buf).unwrap();
    serde_json::to_string(&sa.0).unwrap();
}
