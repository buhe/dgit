use failure::Error;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Issue {
    title: String,
    content: String,
    comments_hash: Vec<String>,
}

impl Issue {
    async fn add(&self, title: String, content: String) -> Result<(), Error> {
        Ok(())
    }

    async fn comment(&self, title: String, content: String) -> Result<(), Error> {
        Ok(())
    }

    async fn load(&self, issue_hash: String) -> Result<String, Error> {
        Ok("".to_string())
    }
}