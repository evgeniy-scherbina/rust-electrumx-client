# rust-electrumx-client
straightforward implementation of https://electrumx.readthedocs.io

```
USAGE:
    electrumx_client [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    broadcasttx        Broadcast a transaction to the network.
    estimatefee        Return the estimated transaction fee per kilobyte for a transaction to be confirmed within a
                       certain number of blocks.
    getbalance         Return the confirmed and unconfirmed balances of a bitcoin address.
    getblockheader     Return the block header at the given height.
    getblockheaders    Return a concatenated chunk of block headers from the main chain.
    getfeehistogram    
    gethistory         Return the confirmed and unconfirmed history of a bitcoin address.
    getlistunspent     Return an ordered list of UTXOs sent to a bitcoin address.
    getmempool         Return the unconfirmed transactions of a bitcoin address.
    getmerkletx        Return the merkle branch to a confirmed transaction given its hash and height.
    gettx              Return a raw transaction.
    getutxos           Return some confirmed UTXOs sent to a bitcoin address.
    help               Prints this message or the help of the given subcommand(s)
    history            Return part of the confirmed history of a bitcoin address.
    relayfee           Return the minimum fee a low-priority transaction must pay in order to be accepted to the
                       daemon’s memory pool.
    toscripthash       
    txidfrompos        
```
