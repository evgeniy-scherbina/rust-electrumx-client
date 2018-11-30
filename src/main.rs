extern crate clap;
extern crate electrumx_client;

use clap::{App, Arg, SubCommand};

use electrumx_client::{
    electrumx_client::ElectrumxClient,
    interface::Electrumx,
    tools,
};
fn main() {
    let matches = App::new("electrumx_client")
        .version("1.0")
        .subcommand(SubCommand::with_name("getblockheader")
            .arg(Arg::with_name("height")
                .long("height")
                .takes_value(true)
                .required(true)
                .help("The height of the block, a non-negative integer."))
            .arg(Arg::with_name("decode")
                .long("decode")
                .help("Decode hex-encoded block header and return in human-readable format."))
            .about("Return the block header at the given height."))
        .subcommand(SubCommand::with_name("getblockheaders")
            .arg(Arg::with_name("start_height")
                .long("start_height")
                .takes_value(true)
                .required(true)
                .help("The height of the first header requested, a non-negative integer."))
            .arg(Arg::with_name("count")
                .long("count")
                .takes_value(true)
                .required(true)
                .help("The number of headers requested, a non-negative integer."))
            .arg(Arg::with_name("decode")
                .long("decode")
                .help("Decode hex-encoded block headers and return in human-readable format."))
            .about("Return a concatenated chunk of block headers from the main chain."))
        .subcommand(SubCommand::with_name("estimatefee")
            .arg(Arg::with_name("number")
                .long("number")
                .takes_value(true)
                .required(true)
                .help("The number of blocks to target for confirmation."))
            .about("Return the estimated transaction fee per kilobyte for a transaction to be \
            confirmed within a certain number of blocks."))
        .subcommand(SubCommand::with_name("relayfee")
            .about("Return the minimum fee a low-priority transaction must pay in order to be \
            accepted to the daemon’s memory pool."))
        .subcommand(SubCommand::with_name("toscripthash")
            .arg(Arg::with_name("addr")
                .long("addr")
                .takes_value(true)
                .required(true)))
        .subcommand(SubCommand::with_name("getbalance")
            .arg(Arg::with_name("addr")
                .long("addr")
                .takes_value(true)
                .required(true)
                .help("Ordinary bitcoin address. It will be automatically converted in suitable \
                 for electrumx protocol format"))
            .about("Return the confirmed and unconfirmed balances of a bitcoin address."))
        .subcommand(SubCommand::with_name("gethistory")
            .arg(Arg::with_name("addr")
                .long("addr")
                .takes_value(true)
                .required(true)
                .help("Ordinary bitcoin address. It will be automatically converted in suitable \
                 for electrumx protocol format"))
            .about("Return the confirmed and unconfirmed history of a bitcoin address."))
        .subcommand(SubCommand::with_name("getmempool")
            .arg(Arg::with_name("addr")
                .long("addr")
                .takes_value(true)
                .required(true)
                .help("Ordinary bitcoin address. It will be automatically converted in suitable \
                 for electrumx protocol format"))
            .about("Return the unconfirmed transactions of a bitcoin address."))
        .subcommand(SubCommand::with_name("history")
            .arg(Arg::with_name("addr")
                .long("addr")
                .takes_value(true)
                .required(true)
                .help("Ordinary bitcoin address. It will be automatically converted in suitable \
                 for electrumx protocol format"))
            .about("Return part of the confirmed history of a bitcoin address."))
        .subcommand(SubCommand::with_name("getlistunspent")
            .arg(Arg::with_name("addr")
                .long("addr")
                .takes_value(true)
                .required(true)
                .help("Ordinary bitcoin address. It will be automatically converted in suitable \
                 for electrumx protocol format"))
            .about("Return an ordered list of UTXOs sent to a bitcoin address."))
        .subcommand(SubCommand::with_name("getutxos")
            .arg(Arg::with_name("addr")
                .long("addr")
                .takes_value(true)
                .required(true)
                .help("Ordinary bitcoin address. It will be automatically converted in suitable \
                 for electrumx protocol format"))
            .arg(Arg::with_name("start_height")
                .long("start_height")
                .takes_value(true)
                .required(true)
                .help("UTXOs will be returned starting from this height, a non-negative integer. \
                If there are several UTXOs in one block, the server will return all of them – \
                partial results from a block are not permitted. The client can start subsequent \
                requests at one above the greatest returned height and avoid repeats."))
            .about("Return some confirmed UTXOs sent to a bitcoin address."))
        .subcommand(SubCommand::with_name("broadcasttx")
            .arg(Arg::with_name("raw_tx")
                .long("raw_tx")
                .takes_value(true)
                .required(true)
                .help("The raw transaction as a hexadecimal string."))
            .about("Broadcast a transaction to the network."))
        .subcommand(SubCommand::with_name("gettx")
            .arg(Arg::with_name("tx_hash")
                .long("tx_hash")
                .takes_value(true)
                .required(true)
                .help("The transaction hash as a hexadecimal string."))
            .arg(Arg::with_name("verbose")
                .long("verbose")
                .help("Whether a verbose coin-specific response is required."))
            .arg(Arg::with_name("merkle")
                .long("merkle")
                .help("Whether a merkle branch proof should be returned as well."))
            .about("Return a raw transaction."))
        .subcommand(SubCommand::with_name("getmerkletx")
            .arg(Arg::with_name("tx_hash")
                .long("tx_hash")
                .takes_value(true)
                .required(true)
                .help("The transaction hash as a hexadecimal string."))
            .arg(Arg::with_name("height")
                .long("height")
                .takes_value(true)
                .required(true)
                .help("The height at which it was confirmed, an integer."))
            .about("Return the merkle branch to a confirmed transaction given its hash and height."))
        .subcommand(SubCommand::with_name("txidfrompos")
            .arg(Arg::with_name("height")
                .long("height")
                .takes_value(true)
                .required(true)
                .help("The main chain block height, a non-negative integer."))
            .arg(Arg::with_name("tx_pos")
                .long("tx_pos")
                .takes_value(true)
                .required(true)
                .help("A zero-based index of the transaction in the given block, an integer."))
            .arg(Arg::with_name("merkle")
                .long("merkle")
                .help("Whether a merkle proof should also be returned, a boolean.")))
        .subcommand(SubCommand::with_name("getfeehistogram"))
        .get_matches();

    let mut client = ElectrumxClient::new("127.0.0.1:60401").unwrap();

    if let Some(matches) = matches.subcommand_matches("getblockheader") {
        let height = matches.value_of("height").unwrap().parse().unwrap();
        let raw_block_header = client.get_block_header(height).unwrap();

        if matches.is_present("decode") {
            let block_header = tools::decode_block_header(raw_block_header);
            println!("{:?}", block_header);
        } else {
            println!("{}", raw_block_header);
        }
    }

    if let Some(matches) = matches.subcommand_matches("getblockheaders") {
        let start_height = matches.value_of("start_height").unwrap().parse().unwrap();
        let count = matches.value_of("count").unwrap().parse().unwrap();
        let raw_block_headers = client.get_block_headers(start_height, count).unwrap();

        if matches.is_present("decode") {
            let block_headers = tools::decode_block_header_chain(raw_block_headers.hex);
            println!("{:?}", block_headers);
        } else {
            println!("{:?}", raw_block_headers);
        }
    }

    if let Some(matches) = matches.subcommand_matches("estimatefee") {
        let number = matches.value_of("number").unwrap().parse().unwrap();
        let fee = client.estimate_fee(number).unwrap();
        println!("{}", fee);
    }

    if let Some(_matches) = matches.subcommand_matches("relayfee") {
        let fee = client.relay_fee().unwrap();
        println!("{}", fee);
    }

    if let Some(matches) = matches.subcommand_matches("getbalance") {
        let addr = matches.value_of("addr").unwrap();
        let resp = client.get_balance(addr).unwrap();
        println!("{:?}", resp);
    }

    if let Some(matches) = matches.subcommand_matches("gethistory") {
        let addr = matches.value_of("addr").unwrap();
        let resp = client.get_history(addr).unwrap();
        println!("{:?}", &resp);
    }

    // TODO(evg): DOES NOT SUPPORT ??
    if let Some(matches) = matches.subcommand_matches("getmempool") {
        let addr = matches.value_of("addr").unwrap();
        let resp = client.get_mempool(addr).unwrap();
        println!("{:?}", &resp);
    }

    // TODO(evg): DOES NOT SUPPORT ??
    if let Some(matches) = matches.subcommand_matches("history") {
        let addr = matches.value_of("addr").unwrap();
        let resp = client.history(addr).unwrap();
        println!("{:?}", &resp);
    }

    if let Some(matches) = matches.subcommand_matches("getlistunspent") {
        let addr = matches.value_of("addr").unwrap();
        let resp = client.get_list_unspent(addr).unwrap();
        println!("{:?}", &resp);
    }

    // TODO(evg): DOES NOT SUPPORT ??
    if let Some(matches) = matches.subcommand_matches("getutxos") {
        let addr = matches.value_of("addr").unwrap();
        let resp = client.get_utxos(addr).unwrap();
        println!("{:?}", &resp);
    }

    if let Some(matches) = matches.subcommand_matches("broadcasttx") {
        let raw_tx = matches.value_of("raw_tx").unwrap();
        let resp = client.broadcast_transaction(raw_tx.to_string()).unwrap();
        println!("{:?}", &resp);
    }

    if let Some(matches) = matches.subcommand_matches("gettx") {
        let tx_hash = matches.value_of("tx_hash").unwrap();
        let verbose = matches.is_present("verbose");
        let merkle = matches.is_present("merkle");
        let tx = client.get_transaction(tx_hash.to_string(), verbose, merkle).unwrap();
        println!("{}", &tx);
    }

    // TODO(evg): DOES IT WORK ??
    if let Some(matches) = matches.subcommand_matches("getmerkletx") {
        let tx_hash = matches.value_of("tx_hash").unwrap();
        let height = matches.value_of("height").unwrap().parse().unwrap();
        let resp = client.get_merkle_transaction(tx_hash.to_string(), height).unwrap();
        println!("{:?}", std::str::from_utf8(&resp).unwrap());
    }

    // TODO(evg): DOES NOT SUPPORT ??
    if let Some(matches) = matches.subcommand_matches("txidfrompos") {
        let height = matches.value_of("height").unwrap().parse().unwrap();
        let tx_pos = matches.value_of("tx_pos").unwrap().parse().unwrap();
        let merkle = matches.is_present("merkle");
        let resp = client.transaction_id_from_pos(height, tx_pos, merkle).unwrap();
        println!("{:?}", std::str::from_utf8(&resp).unwrap());
    }

    if let Some(_matches) = matches.subcommand_matches("getfeehistogram") {
        let resp = client.get_fee_histogram_mempool().unwrap();
        println!("{:?}", std::str::from_utf8(&resp).unwrap());
    }
}