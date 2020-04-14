extern crate futures;

use crate::cli_config::Configuration;
use crate::wallet_app_types::{CryptoState, CryptoTransaction, SignedTransaction};
use awc::Client;
use bytes::Bytes;
use exonum_crypto::Hash;
use generic_traits::state::State;
use utils::crypto::keypair::{CryptoKeypair, Keypair, KeypairType};
use utils::serializer::{deserialize, serialize};

pub struct ClientObj {
    client: Client,
    url: String,
    keypair: KeypairType,
}

impl ClientObj {
    pub fn new(config: &Configuration) -> ClientObj {
        std::env::set_var("RUST_BACKTRACE", "1");
        // let public_address: String = hex::encode(kp.public().encode());
        let mut secret = hex::decode(config.secret.clone()).expect("invalid secret");
        let keypair = Keypair::generate_from(secret.as_mut_slice());
        ClientObj {
            client: Client::default(),
            url: config.url.clone(),
            keypair,
        }
    }

    pub fn get_keypair(&self) -> &KeypairType {
        &self.keypair
    }
    // request for transaction submission to validator
    pub async fn submit_transaction(&self, txn: &SignedTransaction) {
        let mut url: String = self.url.clone();
        url.extend("client/submit_transaction".chars());
        let response = self
            .client
            .post(url) // <- Create request builder
            .header("User-Agent", "Actix-web")
            .send_body(Bytes::from(serialize(&txn))) // <- Send http request
            .await;
        match response {
            Ok(response) => info!("submit_transaction Status: {:?}", response.status()),
            Err(e) => error!("Error response: {:?}", e),
        }
    }

    // request to peer to fetch pending transaction
    pub async fn fetch_pending_transaction(&self, txn_hash: &Hash) {
        let mut url: String = self.url.clone();
        url.extend("client/fetch_pending_transaction".chars());
        let result = self
            .client
            .get(url) // <- Create request builder
            .header("User-Agent", "Actix-web")
            .send_body(Bytes::from(serialize(txn_hash)))
            .await
            .map_err(|_| ());
        match result {
            Ok(mut response) => {
                let resp_body = response.body();
                info!("fetch_pending_transaction Status: {:?}", response.status());
                if response.status() == 200 {
                    match resp_body.await {
                        Ok(txnbody) => {
                            let signed_transaction: SignedTransaction = deserialize(&txnbody);
                            let crypto_transaction: CryptoTransaction =
                                deserialize(&signed_transaction.txn);
                            info!("{:#?}", crypto_transaction);
                        }
                        Err(e) => error!("Error body: {:?}", e),
                    }
                }
            }
            Err(e) => error!("Error response: {:?}", e),
        }
    }

    // request to peer to fetch public_address state
    pub async fn fetch_state(&self, public_address: &String) {
        let mut url: String = self.url.clone();
        url.extend("client/fetch_state".chars());
        let result = self
            .client
            .get(url) // <- Create request builder
            .header("User-Agent", "Actix-web")
            //.send() // <- Send http request
            .send_body(Bytes::from(serialize(public_address)))
            .await
            .map_err(|_| ());
        match result {
            Ok(mut response) => {
                let resp_body = response.body();
                info!("fetch_state Status: {:?}", response.status());
                if response.status() == 200 {
                    match resp_body.await {
                        Ok(state) => {
                            let state: State = deserialize(&state);
                            let crypto_state: CryptoState =
                                deserialize(state.get_data().as_slice());
                            info!("{:#?}", crypto_state);
                        }
                        Err(e) => error!("Error body: {:?}", e),
                    }
                }
            }
            Err(e) => error!("Error response: {:?}", e),
        }
    }

    // request to peer to fetch block
    pub async fn fetch_block(&self, block_index: &u64) {
        let mut url: String = self.url.clone();
        url.extend("client/fetch_block".chars());
        let result = self
            .client
            .get(url) // <- Create request builder
            .header("User-Agent", "Actix-web")
            //.send() // <- Send http request
            .send_body(Bytes::from(serialize(block_index)))
            .await
            .map_err(|_| ());
        match result {
            Ok(mut response) => {
                let resp_body = response.body();
                info!("fetch_block Status: {:?}", response.status());
                if response.status() == 200 {
                    match resp_body.await {
                        Ok(state) => {
                            let fetched_block: String = deserialize(&state);
                            info!("{:#?}", fetched_block);
                        }
                        Err(e) => error!("Error body: {:?}", e),
                    }
                }
            }
            Err(e) => error!("Error response: {:?}", e),
        }
    }

    // request to peer to fetch confirmed transaction
    pub async fn fetch_confirm_transaction(&self, txn_hash: &Hash) {
        let mut url: String = self.url.clone();
        url.extend("client/fetch_confirm_transaction".chars());
        let result = self
            .client
            .get(url) // <- Create request builder
            .header("User-Agent", "Actix-web")
            //.send() // <- Send http request
            .send_body(Bytes::from(serialize(txn_hash)))
            .await
            .map_err(|_| ());
        match result {
            Ok(mut response) => {
                let resp_body = response.body();
                info!("fetch_confirm_transaction Status: {:?}", response.status());
                if response.status() == 200 {
                    match resp_body.await {
                        Ok(txnbody) => {
                            let signed_transaction: SignedTransaction = deserialize(&txnbody);
                            let crypto_transaction: CryptoTransaction =
                                deserialize(&signed_transaction.txn);
                            info!("{:#?}", crypto_transaction);
                        }
                        Err(e) => error!("Error body: {:?}", e),
                    }
                }
            }
            Err(e) => error!("Error response: {:?}", e),
        }
    }

    // request for fetching latest block
    pub async fn fetch_latest_block(&self) {
        let mut url: String = self.url.clone();
        url.extend("client/fetch_latest_block".chars());
        let result = self
            .client
            .get(url) // <- Create request builder
            .header("User-Agent", "Actix-web")
            //.send() // <- Send http request
            .send()
            .await
            .map_err(|_| ());
        match result {
            Ok(mut response) => {
                let resp_body = response.body();
                info!("fetch_block Status: {:?}", response.status());
                if response.status() == 200 {
                    match resp_body.await {
                        Ok(state) => {
                            let fetched_block: String = deserialize(&state);
                            info!("{:#?}", fetched_block);
                        }
                        Err(e) => error!("Error body: {:?}", e),
                    }
                }
            }
            Err(e) => error!("Error response: {:?}", e),
        }
    }

    pub async fn get_nonce(&self) -> Option<u64> {
        let public_key: &String = &hex::encode(self.keypair.public().encode());
        let mut url: String = self.url.clone();
        url.extend("client/fetch_state".chars());
        let result = self
            .client
            .get(url) // <- Create request builder
            .header("User-Agent", "Actix-web")
            //.send() // <- Send http request
            .send_body(Bytes::from(serialize(public_key)))
            .await
            .map_err(|_| ());
        match result {
            Ok(mut response) => {
                let resp_body = response.body();
                info!("fetch_state Status: {:?}", response.status());
                if response.status() == 200 {
                    match resp_body.await {
                        Ok(state) => {
                            let state: State = deserialize(&state);
                            let crypto_state: CryptoState =
                                deserialize(state.get_data().as_slice());
                            return Some(crypto_state.nonce + 1);
                        }
                        Err(_) => return Some(0),
                    }
                }
                if response.status() == 400 {
                    return Some(0);
                } else {
                    return None;
                }
            }
            Err(e) => {
                error!("Error response: {:?}", e);
                return None;
            }
        }
    }
}
