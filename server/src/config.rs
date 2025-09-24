use std::net::SocketAddr;

/// Centralized configuration loaded from environment variables with sensible defaults.
///
/// Environment variables:
/// - DATABASE_URL: Database connection string (default: "sqlite:connexa.db")
/// - HTTP_ADDR: HTTP bind address (default: "127.0.0.1:3000")
/// - P2P_LISTEN: libp2p multiaddr to listen on (default: "/ip4/0.0.0.0/tcp/0")
/// - GOSSIP_TOPICS: Comma-separated list of gossipsub topics (default: "connexa-cover-traffic")
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub http_addr: SocketAddr,
    pub p2p_listen: String,
    pub gossip_topics: Vec<String>,
}

impl Config {
    pub fn from_env() -> Self {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:connexa.db".to_string());

        let http_addr_str = std::env::var("HTTP_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
        let http_addr: SocketAddr = http_addr_str
            .parse()
            .unwrap_or_else(|_| panic!("Invalid HTTP_ADDR: {}", http_addr_str));

        let p2p_listen = std::env::var("P2P_LISTEN").unwrap_or_else(|_| "/ip4/0.0.0.0/tcp/0".to_string());

        let topics_csv = std::env::var("GOSSIP_TOPICS").unwrap_or_else(|_| "connexa-cover-traffic".to_string());
        let gossip_topics = topics_csv
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Self {
            database_url,
            http_addr,
            p2p_listen,
            gossip_topics,
        }
    }
}
