use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockInfoResult {
    pub blocks: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeInfo {
    pub result: BlockInfoResult
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcJsonBlockhash {
    pub result: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcJsonBlockResult {
    pub height: i64,
    pub hash: String,
    pub merkleroot: String,
    pub nextblockhash: String,
    pub previousblockhash: String,
    pub nonce: i64,
    pub size: i32,
    pub time: i32,
    pub version: i32,
    pub tx: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcJsonBlock {
    pub result: RpcJsonBlockResult
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionScriptSig {
    asm: String,
    hex: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionVin {
    sequence: u32,
    /// The raw scriptSig in case of a coinbase tx.
    coinbase: Option<String>,
    txid: Option<String>,
    vout: Option<u32>,
    /// The scriptSig in case of a non-coinbase tx.
    #[serde(rename = "scriptSig")]
    script_sig: Option<TransactionScriptSig>,
    txinwitness: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinbaseTransactionScriptPubKey {
    asm: String,
    desc: String,
    hex: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferTransactionScriptPubKey {
    asm: String,
    desc: String,
    hex: String,
    #[serde(rename = "type")]
    pub_type: String,
    txid: String,
    vout: i16,
    #[serde(rename = "reqSigs")]
    req_sigs: i64,
    addresses: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawTransactionResultVoutScriptPubKey {
    asm: String,
    desc: String,
    hex: String,
    #[serde(rename = "type")]
    pub_type: Option<String>,
    // txid: String,
    // vout: i16,
    #[serde(rename = "reqSigs")]
    req_sigs: Option<i64>,
    addresses: Option<Vec<String>>,
} 
#[derive(Serialize, Deserialize, Debug)]
pub struct  TransactionVout {
    value: f64,
    n: i16,
    // #[serde(rename = "scriptPubKey")]
    // script_pub_key: CoinbaseTransactionScriptPubKey,
    // script_pub_key: TransferTransactionScriptPubKey,
    #[serde(rename = "scriptPubKey")]
    script_pub_key: RawTransactionResultVoutScriptPubKey,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RpcJsonRawTransactionResult {
    in_active_chain: bool,
    hex: String,
    txid: String,
    hash: String,
    size: i32,
    weight: i32,
    vsize: i32,
    version: i8,
    locktime: i32,
    vin: Vec<TransactionVin>,
    vout: Vec<TransactionVout>,
    blockhash: String,
    confirmations: i32,
    blocktime: i32,
    time: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcJsonRawTransaction {
    // result: String,
    result: RpcJsonRawTransactionResult,
}
