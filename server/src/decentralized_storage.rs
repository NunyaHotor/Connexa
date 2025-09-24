use ipfs_api::{IpfsClient, TryFromUri};


use std::io::Cursor;
use futures::TryStreamExt;


pub struct Storage {
    pub client: IpfsClient,
}

impl Storage {
    pub fn new() -> Self {
        let client = IpfsClient::from_host_and_port("127.0.0.1".parse().unwrap(), 5001).unwrap();
        Self { client }
    }

    pub async fn add_file(&self, data: Vec<u8>) -> Result<String, String> {
        let cursor = Cursor::new(data);
        let response = self.client.add(cursor).await.map_err(|e| e.to_string())?;
        Ok(response.hash)
    }

    pub async fn get_file(&self, hash: &str) -> Result<Vec<u8>, String> {
        let bytes = self.client.cat(hash).map_ok(|chunk| chunk.to_vec()).try_concat().await.map_err(|e| e.to_string())?;
        Ok(bytes)
    }
}
