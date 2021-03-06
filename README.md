![Build](https://github.com/Talentica/simpleblockchain/workflows/Build/badge.svg)
[![Gitter](https://badges.gitter.im/simpleblockchain-framework/community.svg)](https://gitter.im/simpleblockchain-framework/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)
[![Dependabot](https://badgen.net/badge/Dependabot/enabled/green?icon=dependabot)](https://dependabot.com/)
[![codecov](https://codecov.io/gh/Talentica/simpleblockchain/branch/master/graph/badge.svg)](https://codecov.io/gh/Talentica/simpleblockchain)



# Simpleblockchain
A framework to build blockchain applications.
(Please refer to Wiki for details)

## Build
To build the docker image, use these below steps
  * Clone the repository
  * Run from shell ./build.sh
  * The build will generate 3 docker images simplebc_buildbase, simplebc_build and simplebc
  * The build will also generate a wallet_app_client binary which can be used to test the wallet functionality of the blockchain.


## Setting up the test network
We will deploy a three node network where one node is in validator configuration with POA consensus and other 2 in observation mode. 
After the docker images are generated using build script,
  * Go to tests/docker folder and verify the configuration settings. 
    - Make sure the flag `genesis_block` is true for the validator node.
    - Make sure the user developed apps are copied to ./config*/app folder properly and path for the same is correctly mentioned using `client_apps`.
    - Ports exposed by all the docker nodes are correct.
  * Set RUST_BACKTRACE=1 in the docker environment for debugging.
  * Run `docker-compose up` to bring up all the nodes.
  * use the `wallet_app_client` to mint the coins, query the blockchain state and submit a transaction.
  
## Deployment
Validators and fullnodes can be deployed either using docker or directly over bare metal server. To deploy docker based setup, please follow the above [section](#setting-up-the-test-network).

To deploy on the bare metal server follow these steps:

  * clone the code base
  * install rust and cargo
  * run `cargo build --release`
  * `node` application should be generated in `target/release` directory
  * build the blockchain application
  * update `config.toml` file and run the `node` application

## Basic Transaction Flow

![Alt text](./misc/BlockchainTxnFlowDiagram.jpg?raw=true "Transaction Flow in a blockchain")


## License
Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

