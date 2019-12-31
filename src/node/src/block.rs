extern crate utils;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use utils::keypair::{CryptoKeypair, Keypair, KeypairType, PublicKey, Verify};
use utils::serializer::{serialize, serialize_hash256, Deserialize, Serialize};

mod transaction;
use transaction::{SignedTransaction, TransactionPool, Txn, TxnPool};

const PEER_ID: &str = "static Id";

pub trait BlockTraits<T> {
    fn validate(&self, publickey: &String, signature: &[u8]) -> bool;
    fn sign(&self, keypair: &T) -> Vec<u8>;
}

pub trait BlockchainTraits {
    fn new() -> Self;
    fn block_chain_length(&self) -> usize;
    fn get_root_hash(&self) -> String;
    fn get_root_block(&self) -> Block;
    fn get_block(&self, block_hash: String) -> Block;
    fn add_block(&mut self, block: Block);
    fn generate_block(&mut self);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    id: u32,
    peer_id: String,
    prev_hash: String,
    txn_pool: Vec<SignedTransaction>,
}

pub struct BlockChain {
    keypair: KeypairType,
    blocks: HashMap<String, Block>,
    root_hash: String,
    txn_pool: TransactionPool,
}

impl BlockTraits<KeypairType> for Block {
    fn validate(&self, publickey: &String, signature: &[u8]) -> bool {
        // unimplemented!();
        let ser_block = serialize(&self);
        PublicKey::verify_from_encoded_pk(&publickey, &ser_block, &signature)
        // keypair::PublicKey::verify_from_encoded_pk(&self.txn.party_a, signing_string.as_bytes(), &self.signature.as_ref())
    }

    fn sign(&self, keypair: &KeypairType) -> Vec<u8> {
        // unimplemented!();
        let ser_block = serialize(&self);
        let sign = Keypair::sign(&keypair, &ser_block);
        sign
    }
}

impl BlockchainTraits for BlockChain {
    // create genesis block and set its hash as root_hash of blockchain
    fn new() -> Self {
        let genesis_block = Block {
            id: 0,
            peer_id: PEER_ID.to_string(),
            prev_hash: String::from(""),
            txn_pool: vec![],
        };
        let genesis_block_hash = serialize_hash256(&genesis_block);
        let mut genesis_block_hash_to_str = String::new();
        genesis_block_hash_to_str.push_str(&String::from_utf8_lossy(&genesis_block_hash));
        let mut blocks = HashMap::new();
        blocks.insert(genesis_block_hash_to_str.clone(), genesis_block);
        let keypair = Keypair::generate();
        Self {
            keypair,
            blocks,
            root_hash: genesis_block_hash_to_str,
            txn_pool: TransactionPool::new(),
        }
    }

    fn block_chain_length(&self) -> usize {
        self.blocks.len()
    }

    fn get_root_hash(&self) -> String {
        // unimplemented!();
        self.root_hash.clone()
    }

    fn get_root_block(&self) -> Block {
        self.blocks.get(&self.root_hash).unwrap().clone()
        // unimplemented!();
    }

    fn get_block(&self, _block_hash: String) -> Block {
        self.blocks.get(&_block_hash).unwrap().clone()
        // unimplemented!();
    }

    fn add_block(&mut self, block: Block) {
        // unimplemented!();
        let block_hash = serialize_hash256(&block);
        let mut block_hash_to_str = String::new();
        block_hash_to_str.push_str(&String::from_utf8_lossy(&block_hash));
        self.blocks.insert(block_hash_to_str.clone(), block);
        self.root_hash = block_hash_to_str;
    }

    fn generate_block(&mut self) {
        let prev_block = self.get_root_block();
        let id: u32 = prev_block.id + 1;
        let prev_hash = self.get_root_hash();
        let executed_txns = self.txn_pool.execute();
        // unimplemented!();
        let new_block = Block {
            id,
            peer_id: PEER_ID.to_string(),
            prev_hash,
            txn_pool: executed_txns,
        };
        self.add_block(new_block);
    }
}

pub fn main_block() {
    let mut block_chain = BlockChain::new();
    let block: Block = block_chain.get_root_block();
    let sign = block.sign(&block_chain.keypair);
    let validate = block.validate(&hex::encode(block_chain.keypair.public().encode()), &sign);
    println!("{}", validate);

    // add txn into txnpool
    for _i in 0..100 {
        let signed_txn = SignedTransaction::generate();
        block_chain.txn_pool.add(signed_txn);
    }
    while block_chain.txn_pool.length() > 0 {
        thread::sleep(Duration::from_millis(1000));
        block_chain.generate_block();
        println!("{}", block_chain.txn_pool.length());
        println!("{}", block_chain.block_chain_length());
    }
    println!("{}", block_chain.get_root_hash());
    println!("{:?}", block_chain.get_root_block());
}