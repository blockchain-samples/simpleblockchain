use futures::{channel::mpsc::*, executor::*, future, prelude::*, task::*};
use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent, Topic},
    identity,
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    Multiaddr, NetworkBehaviour, PeerId, Swarm,
};
use std::error::Error;
use std::sync::{Arc, Mutex};

use utils::configreader;
use utils::configreader::Configuration;

use utils::crypto::keypair;
use utils::serializer::*;

use super::messages::*;
use super::p2pbehaviour::P2PBehaviour;

pub struct SimpleSwarm {
    // behaviour: Option<P2PBehaviour<TSubstream>>,
    pub topic_list: Vec<String>,
    pub tx: Sender<Option<MessageTypes>>,
    pub rx: Receiver<Option<MessageTypes>>,
}

impl SimpleSwarm {
    pub fn new() -> Self {
        let (mut tx1, mut rx1) = channel::<Option<MessageTypes>>(4194304);
        SimpleSwarm {
            topic_list: Vec::new(),
            tx: tx1,
            rx: rx1,
        }
    }
    pub fn process(
        &mut self,
        peer_id: PeerId,
        config: &Configuration,
    ) -> Result<(), Box<dyn Error>> {
        // let transport = libp2p::build_tcp_ws_secio_mplex_yamux(libp2p::identity::Keypair::Ed25519(
        // config.node.keypair.clone(),
        // ))
        // .unwrap();
        let transport = libp2p::build_development_transport(libp2p::identity::Keypair::Ed25519(
            config.node.keypair.clone(),
        ))
        .unwrap();
        let mut behaviour = P2PBehaviour::new(peer_id.clone());
        for topic in &self.topic_list {
            behaviour.subscribe(&topic);
        }
        let mut swarm = Swarm::new(transport, behaviour, peer_id);
        // behaviour.unwrap().subscribe(String::from("test-msg"));

        Swarm::listen_on(
            &mut swarm,
            format!("{}{}", "/ip4/0.0.0.0/tcp/", config.node.p2p_port)
                .parse()
                .unwrap(),
        )
        .unwrap();

        let mut listening = false;
        block_on(future::poll_fn(move |cx: &mut Context| {
            loop {
                match self.rx.poll_next_unpin(cx) {
                    Poll::Ready(Some(msg)) => {
                        println!("msg received {:?}", msg);
                        match msg {
                            None => println!("empty message !"),
                            Some(msgtype) => match msgtype {
                                MessageTypes::NodeMsg(data) => {
                                    let msgdata: Vec<u8> = serialize(&data);
                                    let topics: Vec<Topic> =
                                        Vec::<Topic>::from(MessageTypes::NodeMsg(data)); //TODO Find way to get rid of clone
                                    swarm.floodsub.publish_many(topics, msgdata)
                                }
                                MessageTypes::ConsensusMsg(data) => {
                                    let msgdata: Vec<u8> = serialize(&data);
                                    let topics: Vec<Topic> =
                                        Vec::<Topic>::from(MessageTypes::ConsensusMsg(data));
                                    swarm.floodsub.publish_many(topics, msgdata)
                                }
                                _ => println!("unhandled msgs"),
                            },
                        }
                    }
                    Poll::Ready(None) => {
                        println!("channel closed !");
                        return Poll::Ready(Ok(()));
                        // Poll::Ready(());
                    }
                    Poll::Pending => break,
                }
            }

            loop {
                match swarm.poll_next_unpin(cx) {
                    Poll::Ready(Some(event)) => println!("{:?}", event),
                    Poll::Ready(None) => return Poll::Ready(Ok(())),
                    Poll::Pending => {
                        if !listening {
                            if let Some(a) = Swarm::listeners(&swarm).next() {
                                println!("Listening on {:?}", a);
                                listening = true;
                            }
                        }
                        break;
                    }
                }
            }
            Poll::Pending
        }))
    }
}