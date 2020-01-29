use super::*;
use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use toml;

use crypto::keypair::CryptoKeypair;
use crypto::keypair::Keypair;

use serde::{Deserialize, Serialize};

use toml::Value;

#[derive(Debug, Serialize, Deserialize)]
pub enum NODETYPE {
    FullNode,
    Validator,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TomlReaderConfig {
    pub public: String,
    pub secret: String,
    node_type: String,
    genesis_block: bool,
    //db config
    dbpath: String,
}

#[derive(Debug)]
pub struct Configuration {
    //Node
    pub node: Node,
    //P2P config
    //peers list

    //Database config
    pub db: Database,
}

impl Configuration {
    pub fn new() -> Self {
        let tomlreader: TomlReaderConfig = Configuration::init_config();
        let mut secret = hex::decode(tomlreader.secret).expect("invalid secret");
        let keypair = Keypair::generate_from(secret.as_mut_slice());
        if hex::encode(keypair.public().encode()) != tomlreader.public {
            panic!("Secret and public key pair is invalid");
        }
        let mut node_type: NODETYPE;
        println!("{}", tomlreader.node_type.to_ascii_lowercase());
        if tomlreader.node_type.to_ascii_lowercase() == "fullnode" {
            node_type = NODETYPE::FullNode
        } else if tomlreader.node_type.to_ascii_lowercase() == "validator" {
            node_type = NODETYPE::Validator
        } else {
            panic!("node type not defined properly");
        }
        let node_obj: Node = Node {
            public: Keypair::public(&keypair),
            hex_public: tomlreader.public,
            keypair: keypair,
            node_type,
            genesis_block: tomlreader.genesis_block,
        };
        let db_path: Database = Database {
            dbpath: tomlreader.dbpath,
        };
        let conf_obj = Configuration {
            node: node_obj,
            db: db_path,
        };
        conf_obj
    }
    pub fn init_config() -> TomlReaderConfig {
        // get Current Directory
        let cwd: String = match env::current_dir() {
            Ok(c) => c.display().to_string(),
            Err(e) => panic!(
                "Error processing envirnment variable of current_exe dir - Err: {}!",
                e
            ),
        };
        println!(">> Current Working Directory: {}", cwd);
        let config_file_path: String = cwd + &String::from("/config.toml");
        println!("path = {}", config_file_path);
        let mut config_file = match File::open(config_file_path) {
            Ok(f) => f,
            Err(e) => panic!("Error occurred opening config file:  Err: {}", e),
        };
        let mut config_file_str = String::new();
        config_file
            .read_to_string(&mut config_file_str)
            .expect("Error reading config");
        let conf_data: configreader::TomlReaderConfig = toml::from_str(&config_file_str).unwrap();
        conf_data
    }
}

#[derive(Debug)]
pub struct Node {
    //Node config
    pub public: crypto::keypair::PublicKeyType,
    pub hex_public: String,
    pub keypair: crypto::keypair::KeypairType,
    pub node_type: NODETYPE,
    pub genesis_block: bool,
}

#[derive(Debug)]
pub struct Database {
    pub dbpath: String,
}

lazy_static! {
    pub static ref GLOBAL_CONFIG: Configuration = Configuration::new();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_config() {
        use super::*;
        println!("conf data = {:?}", configreader::GLOBAL_CONFIG.node);
        assert_eq!(
            hex::encode(configreader::GLOBAL_CONFIG.node.keypair.public().encode()),
            configreader::GLOBAL_CONFIG.node.hex_public
        );
    }
}
