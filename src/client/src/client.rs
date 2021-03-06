use exonum_crypto::Hash;
use libp2p::{identity::PublicKey, PeerId};
use reqwest::{Client, Error};
use schema::block::SignedBlock;
use schema::signed_transaction::SignedTransaction;
use schema::state::State;
use schema::transaction_pool::{TxnPool, TxnPoolKeyType, POOL};
use std::collections::HashMap;
use std::net::IpAddr;
use utils::global_peer_data::{PeerData, GLOBALDATA};
use utils::serializer::{deserialize, serialize};

fn get_peer_url() -> Option<String> {
    let locked_peer_map = GLOBALDATA.lock().unwrap();
    for (_, peer_data) in locked_peer_map.peers.iter() {
        let peer: PeerData = peer_data.clone();
        match peer.get_network_addr() {
            Ok(ip_address) => {
                let ip_string: String = match ip_address {
                    IpAddr::V4(ip) => ip.to_string(),
                    IpAddr::V6(ip) => ip.to_string(),
                };
                let mut url: String = String::from("http://");
                url.extend(ip_string.chars());
                url.extend(":8089/".chars());
                return Some(url);
            }
            Err(_) => {}
        }
    }
    None
}

fn get_peer_url_using_pk(pk: &String) -> Option<String> {
    let public_key: PublicKey = match utils::keypair::PublicKey::from_string(pk) {
        Some(public_key) => PublicKey::Ed25519(public_key),
        None => return None,
    };
    let peer_id_string: String = PeerId::from_public_key(public_key).to_string();
    let locked_peer_map = GLOBALDATA.lock().unwrap();
    for (peer_id, peer_data) in locked_peer_map.peers.iter() {
        let peer_id: &String = peer_id;
        if peer_id.eq(&peer_id_string) {
            let peer: PeerData = peer_data.clone();
            match peer.get_network_addr() {
                Ok(ip_address) => {
                    let ip_string: String = match ip_address {
                        IpAddr::V4(ip) => ip.to_string(),
                        IpAddr::V6(ip) => ip.to_string(),
                    };
                    let mut url: String = String::from("http://");
                    url.extend(ip_string.chars());
                    url.extend(":8089/".chars());
                    return Some(url);
                }
                Err(_) => {}
            }
        }
    }
    None
}

pub struct ClientObj {
    client: Client,
}

#[derive(Debug)]
pub struct SyncState {
    pub index: u64,
    pub block_map: HashMap<u64, SignedBlock>,
    pub txn_map: HashMap<Hash, SignedTransaction>,
}

impl SyncState {
    pub fn new() -> SyncState {
        SyncState {
            index: 0,
            block_map: HashMap::new(),
            txn_map: HashMap::new(),
        }
    }

    pub fn new_from(
        index: u64,
        block_map: HashMap<u64, SignedBlock>,
        txn_map: HashMap<Hash, SignedTransaction>,
    ) -> SyncState {
        SyncState {
            index,
            block_map,
            txn_map,
        }
    }
}

impl ClientObj {
    pub fn new() -> ClientObj {
        std::env::set_var("RUST_BACKTRACE", "1");
        ClientObj {
            client: Client::new(),
        }
    }

    pub fn submit_transaction(&self, txn: Vec<u8>) -> Result<bool, Error> {
        let mut url: String = match get_peer_url() {
            Some(url) => url,
            None => return Ok(false),
        };
        url.extend("client/submit_transaction".chars());
        let response = self
            .client
            .post(&url) // <- Create request builder
            .header("User-Agent", "Actix-web")
            //.send() // <- Send http request
            .body(txn)
            .send()?;
        match response.error_for_status() {
            Ok(mut body) => {
                let mut buf: Vec<u8> = vec![];
                body.copy_to(&mut buf)?;
                match deserialize::<String>(buf.as_slice()) {
                    Result::Ok(_) => return Ok(true),
                    Result::Err(_) => return Ok(false),
                }
            }
            Err(err) => return Result::Err(err),
        }
    }

    // request to peer to fetch pending transaction
    pub fn fetch_pending_transaction(
        &self,
        txn_hash: &Hash,
    ) -> Result<Option<SignedTransaction>, Error> {
        let mut url: String = match get_peer_url() {
            Some(url) => url,
            None => return Ok(None),
        };
        url.extend("client/fetch_pending_transaction".chars());
        let serialized_body: Vec<u8> = match serialize(txn_hash) {
            Result::Ok(value) => value,
            Result::Err(_) => vec![0],
        };
        let response = self
            .client
            .get(&url) // <- Create request builder
            .header("User-Agent", "Actix-web")
            //.send() // <- Send http request
            .body(serialized_body)
            .send()?;
        match response.error_for_status() {
            Ok(mut body) => {
                let mut buf: Vec<u8> = vec![];
                body.copy_to(&mut buf)?;
                let signed_transaction: SignedTransaction = match deserialize(buf.as_slice()) {
                    Result::Ok(value) => value,
                    Result::Err(_) => return Ok(None),
                };
                Ok(Some(signed_transaction))
            }
            Err(err) => {
                return Result::Err(err);
            }
        }
    }

    // request to peer to fetch public_address state
    pub fn fetch_state(&self, public_address: &String) -> Result<Option<State>, Error> {
        let mut url: String = match get_peer_url() {
            Some(url) => url,
            None => return Ok(None),
        };
        url.extend("client/fetch_state".chars());
        let serialized_body: Vec<u8> = match serialize(public_address) {
            Result::Ok(value) => value,
            Result::Err(_) => vec![0],
        };
        let response = self
            .client
            .get(&url) // <- Create request builder
            .header("User-Agent", "Actix-web")
            .body(serialized_body)
            .send()?;
        match response.error_for_status() {
            Ok(mut body) => {
                let mut buf: Vec<u8> = vec![];
                body.copy_to(&mut buf)?;
                let state: State = match deserialize(buf.as_slice()) {
                    Result::Ok(value) => value,
                    Result::Err(_) => return Ok(None),
                };
                Ok(Some(state))
            }
            Err(err) => {
                return Result::Err(err);
            }
        }
    }

    // request to peer to fetch block
    pub fn fetch_block(&self, block_index: &u64) -> Result<Option<SignedBlock>, Error> {
        let mut url: String = match get_peer_url() {
            Some(url) => url,
            None => return Ok(None),
        };
        url.extend("peer/fetch_block".chars());
        let serialized_body: Vec<u8> = match serialize(block_index) {
            Result::Ok(value) => value,
            Result::Err(_) => vec![0],
        };
        let response = self
            .client
            .get(&url) // <- Create request builder
            .header("User-Agent", "Actix-web")
            .body(serialized_body)
            .send()?;
        match response.error_for_status() {
            Ok(mut body) => {
                let mut buf: Vec<u8> = vec![];
                body.copy_to(&mut buf)?;
                let signed_block: SignedBlock = match deserialize(buf.as_slice()) {
                    Result::Ok(value) => value,
                    Result::Err(_) => return Ok(None),
                };
                Ok(Some(signed_block))
            }
            Err(err) => {
                return Result::Err(err);
            }
        }
    }

    // request to peer to fetch confirmed transaction
    pub fn fetch_confirm_transaction(
        &self,
        txn_hash: &Hash,
    ) -> Result<Option<SignedTransaction>, Error> {
        let mut url: String = match get_peer_url() {
            Some(url) => url,
            None => return Ok(None),
        };
        url.extend("client/fetch_confirm_transaction".chars());
        let serialized_body: Vec<u8> = match serialize(txn_hash) {
            Result::Ok(value) => value,
            Result::Err(_) => vec![0],
        };
        let response = self
            .client
            .get(&url) // <- Create request builder
            .header("User-Agent", "Actix-web")
            .body(serialized_body) // <- Send http request
            .send()?;
        match response.error_for_status() {
            Ok(mut body) => {
                let mut buf: Vec<u8> = vec![];
                body.copy_to(&mut buf)?;
                let signed_transaction: SignedTransaction = match deserialize(buf.as_slice()) {
                    Result::Ok(value) => value,
                    Result::Err(_) => return Ok(None),
                };
                Ok(Some(signed_transaction))
            }
            Err(err) => {
                return Result::Err(err);
            }
        }
    }

    // request for fetching latest block
    pub fn fetch_latest_block(&self) -> Result<Option<SignedBlock>, Error> {
        let mut url: String = match get_peer_url() {
            Some(url) => url,
            None => return Ok(None),
        };
        url.extend("peer/fetch_latest_block".chars());
        let response = self
            .client
            .get(&url) // <- Create request builder
            .header("User-Agent", "Actix-web")
            //.send() // <- Send http request
            .send()?;
        match response.error_for_status() {
            Ok(mut body) => {
                let mut buf: Vec<u8> = vec![];
                body.copy_to(&mut buf)?;
                let signed_block: SignedBlock = match deserialize(buf.as_slice()) {
                    Result::Ok(value) => value,
                    Result::Err(_) => return Ok(None),
                };
                Ok(Some(signed_block))
            }
            Err(err) => {
                return Result::Err(err);
            }
        }
    }

    // request for fetching latest block
    pub fn fetch_blockchain_length(&self) -> Result<u64, Error> {
        let mut url: String = match get_peer_url() {
            Some(url) => url,
            None => return Ok(0),
        };
        url.extend("peer/fetch_blockchain_length".chars());
        let response = self
            .client
            .get(&url) // <- Create request builder
            .header("User-Agent", "Actix-web")
            //.send() // <- Send http request
            .send()?;
        match response.error_for_status() {
            Ok(mut body) => {
                let mut buf: Vec<u8> = vec![];
                body.copy_to(&mut buf)?;
                let length: u64 = match deserialize(buf.as_slice()) {
                    Result::Ok(value) => value,
                    Result::Err(_) => return Ok(0),
                };
                Ok(length)
            }
            Err(err) => {
                // asserting a 400 as an example
                // it could be any status between 400...599
                return Result::Err(err);
            }
        }
    }

    /// this function will sync blockchain state with other peers
    pub fn fetch_sync_state(&self, current_length: u64) -> SyncState {
        let mut block_pool: HashMap<u64, SignedBlock> = HashMap::new();
        let mut txn_map: HashMap<Hash, SignedTransaction> = HashMap::new();
        let mut own_chain_length = current_length;
        // let block_threads_vec = vec![];
        info!("sync-state function called");
        let mut fetch_flag: bool = true;
        let is_blockchain_length: Result<u64, Error> = self.fetch_blockchain_length();
        let blockchain_length: u64 = match is_blockchain_length {
            Ok(length) => length,
            Err(_) => return SyncState::new(),
        };
        if blockchain_length == 0 {
            return SyncState::new();
        }
        while own_chain_length < blockchain_length {
            let block: Result<Option<SignedBlock>, Error> = self.fetch_block(&own_chain_length);
            match block {
                Ok(is_signed_block) => {
                    match is_signed_block {
                        Some(signed_block) => {
                            block_pool.insert(own_chain_length.clone(), signed_block);
                            own_chain_length = own_chain_length + 1;
                        }
                        None => own_chain_length = blockchain_length,
                    };
                }
                // no point in fetching higher block since lower is missing.
                Err(_) => own_chain_length = blockchain_length,
            }
        }
        info!("Block fetched -> {:#?}", block_pool.len());
        while fetch_flag {
            for (_key, value) in block_pool.iter() {
                for each in value.block.txn_pool.iter() {
                    let fetch_txn_output: Result<Option<SignedTransaction>, Error> =
                        self.fetch_confirm_transaction(each);
                    match fetch_txn_output {
                        Ok(is_txn) => {
                            match is_txn {
                                Some(txn) => {
                                    txn_map.insert(each.clone(), txn);
                                }
                                None => fetch_flag = false,
                            };
                        }
                        Err(_) => {
                            fetch_flag = false;
                            break;
                        }
                    }
                }
                if !fetch_flag {
                    break;
                }
            }
            fetch_flag = false;
        }
        info!("Sync_State --All data fetched");
        return SyncState::new_from(blockchain_length, block_pool, txn_map);
    }

    pub fn fetch_transaction(
        &self,
        url: &String,
        transaction_hash: &Hash,
    ) -> Result<Option<SignedTransaction>, Error> {
        let mut url: String = url.clone();
        url.extend("peer/fetch_transaction".chars());
        let serialized_body: Vec<u8> = match serialize(transaction_hash) {
            Result::Ok(value) => value,
            Result::Err(_) => return Ok(None),
        };
        let response = self
            .client
            .get(&url) // <- Create request builder
            .header("User-Agent", "Actix-web")
            //.send() // <- Send http request
            .body(serialized_body)
            .send()?;
        match response.error_for_status() {
            Ok(mut body) => {
                let mut buf: Vec<u8> = vec![];
                body.copy_to(&mut buf)?;
                let signed_transaction: SignedTransaction = match deserialize(buf.as_slice()) {
                    Result::Ok(value) => value,
                    Result::Err(_) => return Ok(None),
                };
                Ok(Some(signed_transaction))
            }
            Err(_) => {
                return Ok(None);
            }
        }
    }

    pub fn sync_txn_pool(&self, pk: String, transaction_hash_vec: &Vec<Hash>) -> bool {
        let peer_url: String = match get_peer_url_using_pk(&pk) {
            Some(url) => url,
            None => return true,
        };
        for each in transaction_hash_vec.iter() {
            if None == POOL.get(each) {
                match self.fetch_transaction(&peer_url, each) {
                    Ok(is_txn) => match is_txn {
                        Some(txn) => match txn.header.get(&String::from("timestamp")) {
                            Some(string) => match string.parse::<TxnPoolKeyType>() {
                                Ok(timestamp) => POOL.insert_op(&timestamp, &txn),
                                Err(_) => return false,
                            },
                            None => return false,
                        },
                        None => return false,
                    },
                    Err(_) => return false,
                }
            }
        }
        true
    }
}
