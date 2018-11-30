use response::{GetBlockHeadersResponse, GetBalanceResponse, GetHistoryResponse, GetListUnspentResponse};

#[derive(Debug, Deserialize)]
pub struct GetBlockHeaderRawResponse {
    id:      usize,
    jsonrpc: String,
    pub result:  String,
}

#[derive(Debug, Deserialize)]
pub struct GetBlockHeadersRawResponse {
    id:      usize,
    jsonrpc: String,
    pub result:  GetBlockHeadersResponse,
}

#[derive(Debug, Deserialize)]
pub struct EstimateFeeRawResponse {
    id:      usize,
    jsonrpc: String,
    pub result:  f64,
}

#[derive(Debug, Deserialize)]
pub struct RelayFeeRawResponse {
    id:      usize,
    jsonrpc: String,
    pub result:  f64,
}

#[derive(Debug, Deserialize)]
pub struct GetBalanceRawResponse {
    id:      usize,
    jsonrpc: String,
    pub result:  GetBalanceResponse,
}

#[derive(Debug, Deserialize)]
pub struct GetHistoryRawResponse {
    id:      usize,
    jsonrpc: String,
    pub result:  Vec<GetHistoryResponse>,
}

#[derive(Debug, Deserialize)]
pub struct GetListUnspentRawResponse {
    id:      usize,
    jsonrpc: String,
    pub result:  Vec<GetListUnspentResponse>,
}

#[derive(Debug, Deserialize)]
pub struct BroadcastTransactionRawResponse {
    id:      usize,
    jsonrpc: String,
    pub result:  String,
}

#[derive(Debug, Deserialize)]
pub struct GetTransactionRawResponse {
    id:      usize,
    jsonrpc: String,
    pub result:  String,
}