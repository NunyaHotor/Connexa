use libp2p::{
    gossipsub,
    identity,
    kad,
    swarm::{NetworkBehaviour, Swarm, SwarmEvent},
    PeerId,
    Multiaddr,
};
use std::error::Error;
use tokio::time::{interval, Duration};
use std::sync::Arc;
use tokio::sync::Mutex;
use rand::Rng;
use futures::StreamExt;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviourEvent")]
pub struct MyBehaviour {
    pub kademlia: kad::Kademlia<kad::store::MemoryStore>,
    pub gossipsub: gossipsub::Gossipsub,
}

impl MyBehaviour {
    pub fn bootstrap(&mut self) -> Result<(), String> {
        self.kademlia.bootstrap().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn publish_message(&mut self, topic: &gossipsub::IdentTopic, message: Vec<u8>) -> Result<(), String> {
        self.gossipsub.publish(topic.clone(), message).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn publish_dummy_message(&mut self) -> Result<(), String> {
        let topic = gossipsub::IdentTopic::new("connexa-cover-traffic");
        let dummy_message = (0..32).map(|_| rand::thread_rng().gen::<u8>()).collect::<Vec<u8>>();
        self.gossipsub.publish(topic.clone(), dummy_message).map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub enum MyBehaviourEvent {
    Kademlia(kad::KademliaEvent),
    Gossipsub(gossipsub::GossipsubEvent),
}

impl From<kad::KademliaEvent> for MyBehaviourEvent {
    fn from(event: kad::KademliaEvent) -> Self {
        MyBehaviourEvent::Kademlia(event)
    }
}

impl From<gossipsub::GossipsubEvent> for MyBehaviourEvent {
    fn from(event: gossipsub::GossipsubEvent) -> Self {
        MyBehaviourEvent::Gossipsub(event)
    }
}

pub fn add_peer(swarm: &mut Swarm<MyBehaviour>, peer_id: PeerId, addr: Multiaddr) {
    swarm.behaviour_mut().kademlia.add_address(&peer_id, addr);
}

pub async fn new(swarm: Arc<Mutex<Swarm<MyBehaviour>>>) -> Result<(), Box<dyn Error>> {
    let mut bootstrap_timer = interval(Duration::from_secs(300));
    let mut cover_traffic_timer = interval(Duration::from_secs(10));

    // Kick it off
    loop {
        tokio::select! {
            _ = bootstrap_timer.tick() => {
                swarm.lock().await.behaviour_mut().bootstrap()?;
            }
            _ = cover_traffic_timer.tick() => {
                swarm.lock().await.behaviour_mut().publish_dummy_message()?;
            }
            event = swarm.lock().await.select_next_some() => match event {
                SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::GossipsubEvent::Message {
                    propagation_source: peer_id,
                    message_id: id,
                    message
                })) => {
                    println!(
                        "Got message: {} with id: {} from peer: {}",
                        String::from_utf8_lossy(&message.data),
                        id,
                        peer_id
                    );
                }
                SwarmEvent::Behaviour(MyBehaviourEvent::Kademlia(event)) => {
                    println!("Kademlia event: {:?}", event);
                }
                _ => {}
            }
        }
    }
}

pub fn get_peer_id(swarm: &Swarm<MyBehaviour>) -> PeerId {
    *swarm.local_peer_id()
}

pub fn get_all_peers(swarm: &Swarm<MyBehaviour>) -> Vec<PeerId> {
    swarm
        .behaviour()
        .kademlia
        .kbuckets()
        .flat_map(|kbucket| kbucket.iter().map(|entry| *entry.node.key.preimage()))
        .collect()
}

pub fn get_listening_addresses(swarm: &Swarm<MyBehaviour>) -> Vec<Multiaddr> {
    swarm.listeners().cloned().collect()
}

pub fn get_external_addresses(swarm: &Swarm<MyBehaviour>) -> Vec<Multiaddr> {
    swarm.external_addresses().cloned().collect()
}

pub fn get_kad_dht_info(swarm: &Swarm<MyBehaviour>) -> String {
    format!("{:?}", swarm.behaviour().kademlia)
}

pub fn get_gossipsub_info(swarm: &Swarm<MyBehaviour>) -> String {
    format!("{:?}", swarm.behaviour().gossipsub)
}

pub fn get_swarm_info(swarm: &Swarm<MyBehaviour>) -> String {
    format!("{:?}", swarm)
}

pub fn get_network_info(swarm: &Swarm<MyBehaviour>) -> String {
    let mut info = String::new();
    info.push_str(&format!("Local Peer ID: {:?}\n", get_peer_id(swarm)));
    info.push_str(&format!("Listening Addresses: {:?}\n", get_listening_addresses(swarm)));
    info.push_str(&format!("External Addresses: {:?}\n", get_external_addresses(swarm)));
    info.push_str(&format!("Peers: {:?}\n", get_all_peers(swarm)));
    info.push_str(&format!("Kademlia DHT: {:?}\n", get_kad_dht_info(swarm)));
    info.push_str(&format!("Gossipsub: {:?}\n", get_gossipsub_info(swarm)));
    info
}
