use std::error::Error;
use response::{GetBlockHeadersResponse, GetBalanceResponse, GetHistoryResponse, GetListUnspentResponse};

pub trait Electrumx {
    // Return the block header at the given height.
    // * height - the height of the block, a non-negative integer.
    // * result - the raw block header as a hexadecimal string.
    fn get_block_header(&mut self, height: usize) -> Result<String, Box<Error>>;

    // Return a concatenated chunk of block headers from the main chain.
    // * start_height - the height of the first header requested, a non-negative integer.
    // * count - the number of headers requested, a non-negative integer.
    // * result - A dictionary with the following members:
    // - count - The number of headers returned, between zero and the number requested.
    // If the chain has not extended sufficiently far, only the available headers will be returned.
    // If more headers than max were requested at most max will be returned.
    // - hex - the binary block headers concatenated together in-order as a hexadecimal string.
    // - max - The maximum number of headers the server will return in a single request.
    fn get_block_headers(&mut self, start_height: usize, count: usize) -> Result<GetBlockHeadersResponse, Box<Error>>;

    // Return the estimated transaction fee per kilobyte for a transaction to be confirmed
    // within a certain number of blocks.
    // * result - the estimated transaction fee in coin units per kilobyte, as a floating point number.
    // If the daemon does not have enough information to make an estimate, the integer -1 is returned.
    fn estimate_fee(&mut self, number: usize) -> Result<f64, Box<Error>>;

    // Return the minimum fee a low-priority transaction must pay in order to be accepted
    // to the daemon’s memory pool.
    // * result - the fee in whole coin units (BTC, not satoshis for Bitcoin) as a
    // floating point number.
    fn relay_fee(&mut self) -> Result<f64, Box<Error>>;

    // Return the confirmed and unconfirmed balances of a bitcoin address.
    // * result - a dictionary with keys confirmed and unconfirmed. The value of each is the
    // appropriate balance in coin units as a string.
    fn get_balance(&mut self, addr: &str) -> Result<GetBalanceResponse, Box<Error>>;

    // Return the confirmed and unconfirmed history of a bitcoin address.
    // * result - a list of confirmed transactions in blockchain order, with the output of
    // blockchain.scripthash.get_mempool() appended to the list. Each confirmed transaction is a
    // dictionary with the following keys:
    // - height - the integer height of the block the transaction was confirmed in.
    // - tx_hash - the transaction hash in hexadecimal.
    fn get_history(&mut self, addr: &str) -> Result<Vec<GetHistoryResponse>, Box<Error>>;
    fn get_mempool(&mut self, addr: &str) -> Result<Vec<u8>, Box<Error>>;
    fn history(&mut self, addr: &str) -> Result<Vec<u8>, Box<Error>>;
    fn get_list_unspent(&mut self, addr: &str) -> Result<Vec<GetListUnspentResponse>, Box<Error>>;
    fn get_utxos(&mut self, addr: &str) -> Result<Vec<u8>, Box<Error>>;
    fn broadcast_transaction(&mut self, raw_tx: String) -> Result<String, Box<Error>>;
    fn get_transaction(&mut self, tx_hash: String, verbose: bool, merkle: bool) -> Result<String, Box<Error>>;
    fn get_merkle_transaction(&mut self, tx_hash: String, height: usize) -> Result<Vec<u8>, Box<Error>>;
    fn transaction_id_from_pos(&mut self, height: usize, tx_pos: usize, merkle: bool) -> Result<Vec<u8>, Box<Error>>;
    fn get_fee_histogram_mempool(&mut self) -> Result<Vec<u8>, Box<Error>>;
}