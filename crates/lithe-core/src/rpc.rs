use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcRequest {
    pub function: String,
    pub args: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcResponse {
    pub result: serde_json::Value,
}
