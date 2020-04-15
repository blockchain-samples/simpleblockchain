use super::messages::*;
use futures::prelude::*;
use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent},
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour, PeerId,
};
use std::time::SystemTime;
use utils::globaldata::{PeerData, GLOBALDATA};

/// Network behavior defined combining, floodsub and mdns (for discovery)
///
#[derive(NetworkBehaviour)]
pub struct P2PBehaviour<TSubstream: AsyncRead + AsyncWrite> {
    pub floodsub: Floodsub<TSubstream>,
    pub mdns: Mdns<TSubstream>,
}

impl<TSubstream: AsyncRead + AsyncWrite> P2PBehaviour<TSubstream> {
    pub fn new(peer_id: PeerId) -> Self {
        let mdns = Mdns::new().unwrap();
        let behaviour = P2PBehaviour {
            floodsub: Floodsub::new(peer_id.clone()),
            mdns,
        };
        behaviour
    }
    pub fn subscribe(&mut self, topic_str: &String) {
        let floodsub_topic = floodsub::TopicBuilder::new(topic_str).build();
        self.floodsub.subscribe(floodsub_topic);
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<MdnsEvent>
    for P2PBehaviour<TSubstream>
{
    fn inject_event(&mut self, mdns_event: MdnsEvent) {
        match mdns_event {
            MdnsEvent::Discovered(discovered_nodes) => {
                debug!("Discovered address {:?}", discovered_nodes);
                for (peer_id, multi_address) in discovered_nodes {
                    let peerid_str = peer_id.to_string();
                    info!("peer discovered {} with address {}", peer_id, multi_address);
                    self.floodsub.add_node_to_partial_view(peer_id.clone());
                    let time_stamp = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_micros();
                    GLOBALDATA.lock().unwrap().peers.insert(
                        peerid_str,
                        PeerData::new(peer_id, time_stamp, multi_address),
                    );
                }
            }
            MdnsEvent::Expired(expired_nodes) => {
                debug!("Expired address {:?}", expired_nodes);
                for (peer_id, _) in expired_nodes {
                    let peerid_str = peer_id.to_string();
                    self.floodsub.remove_node_from_partial_view(&peer_id);
                    GLOBALDATA.lock().unwrap().peers.remove(&peerid_str);
                }
            }
        }
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<FloodsubEvent>
    for P2PBehaviour<TSubstream>
{
    fn inject_event(&mut self, pubsub_event: FloodsubEvent) {
        match pubsub_event {
            FloodsubEvent::Message(msg) => {
                debug!(
                    "Message received from {:?}, msg topic {:?}",
                    msg.source, msg.topics
                );
                msg.process(&msg.topics, &msg.data);
            }
            FloodsubEvent::Subscribed {
                peer_id: _,
                topic: _,
            } => {
                // info!("subscribed by peer {:?} topic {:?}", peer_id, topic);
            }
            FloodsubEvent::Unsubscribed {
                peer_id: _,
                topic: _,
            } => {
                // info!("unsubscribed by peer {:?} topic {:?}", peer_id, topic);
            }
        }
    }
}
