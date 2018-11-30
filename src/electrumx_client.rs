use serde_json;

use std::{
    io::{self, Write, BufRead, BufReader},
    net::{TcpStream, ToSocketAddrs},
    error::Error,
};

use interface::Electrumx;
use raw_response::{GetBlockHeaderRawResponse, GetBlockHeadersRawResponse, EstimateFeeRawResponse,
                   RelayFeeRawResponse, GetBalanceRawResponse, GetHistoryRawResponse,
                   GetListUnspentRawResponse, BroadcastTransactionRawResponse, GetTransactionRawResponse};
use response::{GetBlockHeadersResponse, GetBalanceResponse, GetHistoryResponse, GetListUnspentResponse};
use tools;

pub struct ElectrumxClient<A: ToSocketAddrs> {
    #[allow(dead_code)]
    socket_addr: A,
    stream: TcpStream,
}

impl<A: ToSocketAddrs + Clone> Electrumx for ElectrumxClient<A> {
    fn get_block_header(&mut self, height: usize) -> Result<String, Box<Error>> {
        let req = Request::new(0, "blockchain.block.header", vec![Param::Usize(height)]);
        self.call(req)?;
        let raw = self.recv()?;
        let resp: GetBlockHeaderRawResponse = serde_json::from_slice(&raw)?;
        Ok(resp.result)
    }

    fn get_block_headers(&mut self, start_height: usize, count: usize) -> Result<GetBlockHeadersResponse, Box<Error>> {
        let params = vec![
            Param::Usize(start_height),
            Param::Usize(count),
        ];
        let req = Request::new(0, "blockchain.block.headers", params);
        self.call(req)?;
        let raw = self.recv()?;
        let resp: GetBlockHeadersRawResponse = serde_json::from_slice(&raw)?;
        Ok(resp.result)
    }

    fn estimate_fee(&mut self, number: usize) -> Result<f64, Box<Error>> {
        let req = Request::new(0, "blockchain.estimatefee", vec![Param::Usize(number)]);
        self.call(req)?;
        let raw = self.recv()?;
        let resp: EstimateFeeRawResponse = serde_json::from_slice(&raw)?;
        Ok(resp.result)
    }

    fn relay_fee(&mut self) -> Result<f64, Box<Error>> {
        let req = Request::new(0, "blockchain.relayfee", Vec::new());
        self.call(req)?;
        let raw = self.recv()?;
        let resp: RelayFeeRawResponse = serde_json::from_slice(&raw)?;
        Ok(resp.result)
    }

    fn get_balance(&mut self, addr: &str) -> Result<GetBalanceResponse, Box<Error>> {
        let reversed = tools::decode_address_helper(addr);
        let params = vec![Param::String(reversed)];
        let req = Request::new(0, "blockchain.scripthash.get_balance", params);
        self.call(req)?;
        let raw = self.recv()?;
        let resp: GetBalanceRawResponse = serde_json::from_slice(&raw)?;
        Ok(resp.result)
    }

    fn get_history(&mut self, addr: &str) -> Result<Vec<GetHistoryResponse>, Box<Error>> {
        let reversed = tools::decode_address_helper(&addr);
        let params = vec![Param::String(reversed)];
        let req = Request::new(0, "blockchain.scripthash.get_history", params);
        self.call(req)?;
        let raw = self.recv()?;
        let resp: GetHistoryRawResponse = serde_json::from_slice(&raw)?;
        Ok(resp.result)
    }

    // TODO(evg): DOES NOT SUPPORT ??
    fn get_mempool(&mut self, addr: &str) -> Result<Vec<u8>, Box<Error>> {
        let reversed = tools::decode_address_helper(&addr);
        let params = vec![Param::String(reversed)];
        let req = Request::new(0, "blockchain.scripthash.get_mempool", params);
        self.call(req)?;
        let raw = self.recv()?;
        Ok(raw)
    }

    // TODO(evg): DOES NOT SUPPORT ??
    fn history(&mut self, addr: &str) -> Result<Vec<u8>, Box<Error>> {
        let reversed = tools::decode_address_helper(&addr);
        let params = vec![Param::String(reversed)];
        let req = Request::new(0, "blockchain.scripthash.history", params);
        self.call(req)?;
        let raw = self.recv()?;
        Ok(raw)
    }

    fn get_list_unspent(&mut self, addr: &str) -> Result<Vec<GetListUnspentResponse>, Box<Error>> {
        let reversed = tools::decode_address_helper(&addr);
        let params = vec![Param::String(reversed)];
        let req = Request::new(0, "blockchain.scripthash.listunspent", params);
        self.call(req)?;
        let raw = self.recv()?;
        let resp: GetListUnspentRawResponse = serde_json::from_slice(&raw)?;
        Ok(resp.result)
    }

    // TODO(evg): DOES NOT SUPPORT ??
    fn get_utxos(&mut self, addr: &str) -> Result<Vec<u8>, Box<Error>> {
        let reversed = tools::decode_address_helper(&addr);
        let params = vec![Param::String(reversed)];
        let req = Request::new(0, "blockchain.scripthash.utxos", params);
        self.call(req)?;
        let raw = self.recv()?;
        Ok(raw)
    }

    fn broadcast_transaction(&mut self, raw_tx: String) -> Result<String, Box<Error>> {
        let params = vec![Param::String(raw_tx)];
        let req = Request::new(0, "blockchain.transaction.broadcast", params);
        self.call(req)?;
        let raw = self.recv()?;
        let resp: BroadcastTransactionRawResponse = serde_json::from_slice(&raw)?;
        Ok(resp.result)
    }

    fn get_transaction(&mut self, tx_hash: String, verbose: bool, merkle: bool) -> Result<String, Box<Error>> {
        let params = vec![
            Param::String(tx_hash),
            Param::Bool(verbose),
            Param::Bool(merkle),
        ];
        let req = Request::new(0, "blockchain.transaction.get", params);
        self.call(req)?;
        let raw = self.recv()?;
        let resp: GetTransactionRawResponse = serde_json::from_slice(&raw)?;
        Ok(resp.result)
    }

    // TODO(evg): DOES IT WORK ??
    fn get_merkle_transaction(&mut self, tx_hash: String, height: usize) -> Result<Vec<u8>, Box<Error>> {
        let params = vec![
            Param::String(tx_hash),
            Param::Usize(height),
        ];
        let req = Request::new(0, "blockchain.transaction.get_merkle", params);
        self.call(req)?;
        let raw = self.recv()?;
        Ok(raw)
    }

    // TODO(evg): DOES NOT SUPPORT ??
    fn transaction_id_from_pos(&mut self, height: usize, tx_pos: usize, merkle: bool) -> Result<Vec<u8>, Box<Error>> {
        let params = vec![
            Param::Usize(height),
            Param::Usize(tx_pos),
            Param::Bool(merkle),
        ];
        let req = Request::new(0, "blockchain.transaction.id_from_pos", params);
        self.call(req)?;
        let raw = self.recv()?;
        Ok(raw)
    }

    fn get_fee_histogram_mempool(&mut self) -> Result<Vec<u8>, Box<Error>> {
        let req = Request::new(0, "mempool.get_fee_histogram", vec![]);
        self.call(req)?;
        let raw = self.recv()?;
        Ok(raw)
    }
}

impl<A: ToSocketAddrs + Clone> ElectrumxClient<A> {
    pub fn new(socket_addr: A) -> io::Result<Self> {
        let stream = TcpStream::connect(socket_addr.clone())?;
        Ok(Self {
            socket_addr,
            stream,
        })
    }

    fn call(&mut self, req: Request) -> Result<(), Box<Error>> {
        let raw = serde_json::to_vec(&req)?;
        self.stream.write_all(&raw)?;
        self.stream.write_all(&[10])?;
        self.stream.flush()?;
        Ok(())
    }

    fn recv(&self) -> io::Result<Vec<u8>> {
        let mut buff_stream = BufReader::new(&self.stream);
        let mut resp = Vec::new();
        buff_stream.read_until(10, &mut resp)?;
        Ok(resp)
    }
}

#[derive(Serialize)]
#[serde(untagged)]
enum Param {
    Usize(usize),
    String(String),
    Bool(bool),
}

#[derive(Serialize)]
struct Request<'a> {
    id: usize,
    method: &'a str,
    params: Vec<Param>,
}

impl<'a> Request<'a> {
    fn new(id: usize, method: &'a str, params: Vec<Param>) -> Self {
        Self {
            id,
            method,
            params,
        }
    }
}