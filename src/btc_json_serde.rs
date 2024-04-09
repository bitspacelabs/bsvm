use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockInfoResult {
    pub blocks: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeInfo {
    pub result: BlockInfoResult
}
