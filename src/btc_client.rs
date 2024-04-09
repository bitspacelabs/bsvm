use reqwest::Client;
use std::env;
use std::io::{self, ErrorKind};

struct BtcRpcEnv {
    client: Client,
    network: String,
    rpc_url: String,
    rpc_user: String,
    rpc_password: String,
}

fn btc_vm_space_client() -> Result<BtcRpcEnv, io::Error> {
    dotenv::dotenv().ok();
    
    let network = env::var("BTC_NETWORK").unwrap_or_default();
    // Bitcoin Core RPC URL
    let rpc_url = match network.as_str() {
        "MAINNET" => env::var("BTC_MAINNET_RPC_0").unwrap_or_default(),
        "TESTNET" => env::var("BTC_TESTNET_RPC_0").unwrap_or_default(),
        _ => return Err(io::Error::new(ErrorKind::InvalidInput, "rpc_url")),
    };
    let rpc_user = match network.as_str() {
        "MAINNET" => env::var("BTC_MAINNET_USER").unwrap_or_default(),
        "TESTNET" => env::var("BTC_TESTNET_USER").unwrap_or_default(),
        _ => return Err(io::Error::new(ErrorKind::InvalidInput, "rpc_user")),
    };
    let rpc_password = match network.as_str() {
        "MAINNET" => env::var("BTC_MAINNET_PASSWORD").unwrap_or_default(),
        "TESTNET" => env::var("BTC_TESTNET_PASSWORD").unwrap_or_default(),
        _ => return Err(io::Error::new(ErrorKind::InvalidInput, "rpc_password")),
    };
    
    Ok(BtcRpcEnv{
        client: Client::new(),
        network,
        rpc_user,
        rpc_password,
        rpc_url,
    })
}


pub async fn btc_client_run() -> Result<reqwest::RequestBuilder, ()> {
    let btc_client_env = match btc_vm_space_client() {
        Err(e) => {
            println!("btc_client_env::{:#?}", e);
            return Err(())
        },
        Ok(c) => c,
    };
    println!("network::{},rpc_url::{}", btc_client_env.network, btc_client_env.rpc_url);
    // println!("rpc_user::{},rpc_password::{}", btc_client_env.rpc_user, btc_client_env.rpc_password);

    let request_builder: reqwest::RequestBuilder = btc_client_env.client
    .post(btc_client_env.rpc_url)
    .basic_auth(btc_client_env.rpc_user, Some(btc_client_env.rpc_password));

    Ok(request_builder)   
}
