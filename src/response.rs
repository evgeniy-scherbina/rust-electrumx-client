#[derive(Debug, Deserialize)]
pub struct GetBlockHeadersResponse {
    count: usize,
    pub hex:   String,
    max:   usize,
}

#[derive(Debug, Deserialize)]
pub struct GetBalanceResponse {
    confirmed:   u64,
    unconfirmed: u64,
}

#[derive(Debug, Deserialize)]
pub struct GetHistoryResponse {
    height:  usize,
    pub tx_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct GetListUnspentResponse {
    height:  usize,
    tx_hash: String,
    tx_pos:  usize,
    value:   usize,
}