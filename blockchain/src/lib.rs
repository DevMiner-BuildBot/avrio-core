use config::config;
use std::sync::mutex;
use std::sync::once;

assert_eq!(true as i32, 1);
assert_eq!(false as i32, 0);

pub struct Transaction {
    amount: u64,
    sender_key: String,
    gas_price: u64,
    max_gas: u64,
    gas: u64, // gas used
    nonce: u8,
    signature: String,
}

pub struct TxStore { // remove data not needed to be stored
    amount: u64,
    sender_key: String,
    fee: u64, // fee in AIO
    nonce: u8,
    signature: String,
}    

pub struct Header {
    version_major: u8,
    version_minor: u8,
    chain_key: String,
    prev_hash: String,
    timestamp: u64,
}

pub struct Block {
    header: Header,
    txns: Vec<Transaction>,
    txnc: u64;
    hash: String,
    signature: String,
    node_signatures:Vec<String>, // a block must be signed by at least (c / 2) + 1 nodes to be valid (ensures at least ne honest node has singed it)
}


fn check_block(blk: Block) -> bool {
    if blk.header.version_major > config.version_major {
        return false;
    } else if blk.header.prev_hash != get_last_block(blk.header.chain_key) {
        return false;
    } else if !check_signature(blk.signature,blk.header.chain_key) {
        return false;
    }

    for (txn in blk.txns) {
        if !validate_transaction(txn) {
            return false;
        }
    }

    // Todo continue blk validation
}
