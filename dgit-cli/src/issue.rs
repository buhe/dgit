use std::io::Cursor;

use failure::Error;
use ipfs_api_backend_hyper::{IpfsClient, IpfsApi};
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Issue {
    pub title: String,
    pub content: String,
    pub comments_hash: Vec<String>,
}

impl Issue {
    pub async fn add(&self) -> Result<(), Error> {
        let issue = serde_json::to_string(self)?;
        debug!("issue json is {}", issue);
        self.save_raw(Cursor::new(issue)).await?;
        Ok(())
    }

    // async fn comment(&self, title: String, content: String) -> Result<(), Error> {
    //     Ok(())
    // }

    // async fn load(&self, issue_hash: String) -> Result<String, Error> {
    //     Ok("".to_string())
    // }

    async fn save_raw(&self, cursor: Cursor<String>) -> Result<(), Error> {
        let ipfs = IpfsClient::default();
        let raw_data_req = ipfs.add(cursor);
        let ipfs_hash = futures::executor::block_on(raw_data_req)?.hash;
        debug!("save raw hash is {}", ipfs_hash);
        Ok(())
    }
}