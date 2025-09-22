
use libp2p::{
    gossipsub::{Gossipsub, GossipsubEvent, IdentTopic as Topic, MessageAuthenticity, ValidationMode},
    kad::{Kademlia, KademliaEvent, store::MemoryStore},
    swarm::{NetworkBehaviour, Swarm, SwarmEvent},
    Multiaddr,
    PeerId,
    futures::StreamExt,
};
use std::error::Error;
use tokio::time::{interval, Duration};
use std::sync::Arc;
use parking_lot::Mutex;
use rand::Rng;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviourEvent")]
pub struct MyBehaviour {
    pub kademlia: Kademlia<MemoryStore>,
    pub gossipsub: Gossipsub,
}

impl MyBehaviour {
    pub fn bootstrap(&mut self) -> Result<(), String> {
        self.kademlia.bootstrap().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn publish_message(&mut self, topic: &Topic, message: Vec<u8>) -> Result<(), String> {
        self.gossipsub.publish(topic.clone(), message).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn publish_dummy_message(&mut self) -> Result<(), String> {
        let topic = Topic::new("connexa-cover-traffic");
        let dummy_message = (0..32).map(|_| rand::thread_rng().gen::<u8>()).collect::<Vec<u8>>();
        self.gossipsub.publish(topic.clone(), dummy_message).map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub enum MyBehaviourEvent {
    Kademlia(KademliaEvent),
    Gossipsub(GossipsubEvent),
}

impl From<KademliaEvent> for MyBehaviourEvent {
    fn from(event: KademliaEvent) -> Self {
        MyBehaviourEvent::Kademlia(event)
    }
}

impl From<GossipsubEvent> for MyBehaviourEvent {
    fn from(event: GossipsubEvent) -> Self {
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
                swarm.lock().behaviour_mut().bootstrap()?;
            }
            _ = cover_traffic_timer.tick() => {
                swarm.lock().behaviour_mut().publish_dummy_message()?;
            }
            event = swarm.lock().select_next_some() => match event {
                SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(GossipsubEvent::Message { 
                    propagation_source: peer_id, 
                    message_id: id, 
                    message 
                })) => {
                    println!(
                        "Got message: {} with id: {} from peer: {:?}",
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