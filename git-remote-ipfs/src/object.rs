use std::{collections::BTreeSet, io::Cursor};

use failure::Error;
use futures::TryStreamExt;
use git2::{Blob, Odb, Commit, Tag, Tree, OdbObject};
use ipfs_api_backend_hyper::{IpfsClient, IpfsApi};
use log::{error, debug};
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
/// representation of a git object
pub struct GitObject {
    /// The git hash of the underlying git object
    pub git_hash: String,
    /// A link to the raw form of the object
    pub raw_data_ipfs_hash: String,
    /// Object-type-specific metadata
    pub metadata: ObjectMetadata,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
/// A helper type
pub enum ObjectMetadata {
    #[allow(missing_docs)]
    Commit {
        parent_git_hashes: BTreeSet<String>,
        tree_git_hash: String,
    },
    #[allow(missing_docs)]
    Tag { target_git_hash: String },
    #[allow(missing_docs)]
    Tree { entry_git_hashes: BTreeSet<String> },
    #[allow(missing_docs)]
    Blob,
}

impl GitObject {
    /// Instantiate a `Object` from a blob object.
    pub fn from_git_blob(blob: &Blob, odb: &Odb, ipfs: &mut IpfsClient) -> Result<Self, Error> {
        let odb_obj = odb.read(blob.id())?;
        let raw_data_ipfs_hash = Self::upload_odb_obj(&odb_obj, ipfs)?;

        Ok(Self {
            git_hash: blob.id().to_string(),
            raw_data_ipfs_hash,
            metadata: ObjectMetadata::Blob,
        })
    }

    /// Instantiate a `Object` from a commit object.
    pub fn from_git_commit(
        commit: &Commit,
        odb: &Odb,
        ipfs: &mut IpfsClient,
    ) -> Result<Self, Error> {
        let odb_obj = odb.read(commit.id())?;
        let raw_data_ipfs_hash = Self::upload_odb_obj(&odb_obj, ipfs)?;
        let parent_git_hashes: BTreeSet<String> = commit
            .parent_ids()
            .map(|parent_id| format!("{}", parent_id))
            .collect();

        let tree_git_hash = format!("{}", commit.tree()?.id());

        Ok(Self {
            git_hash: commit.id().to_string(),
            raw_data_ipfs_hash,
            metadata: ObjectMetadata::Commit {
                parent_git_hashes,
                tree_git_hash,
            },
        })
    }

    /// Instantiate a `Object` from an annotated/signed tag object.
    pub fn from_git_tag(tag: &Tag, odb: &Odb, ipfs: &mut IpfsClient) -> Result<Self, Error> {
        let odb_obj = odb.read(tag.id())?;
        let raw_data_ipfs_hash = Self::upload_odb_obj(&odb_obj, ipfs)?;

        Ok(Self {
            git_hash: tag.id().to_string(),
            raw_data_ipfs_hash,
            metadata: ObjectMetadata::Tag {
                target_git_hash: format!("{}", tag.target_id()),
            },
        })
    }

    /// Instantiate a `Object` from a tree object.
    pub fn from_git_tree(tree: &Tree, odb: &Odb, ipfs: &mut IpfsClient) -> Result<Self, Error> {
        let odb_obj = odb.read(tree.id())?;
        let raw_data_ipfs_hash = Self::upload_odb_obj(&odb_obj, ipfs)?;

        let entry_git_hashes: BTreeSet<String> =
            tree.iter().map(|entry| format!("{}", entry.id())).collect();

        Ok(Self {
            git_hash: tree.id().to_string(),
            raw_data_ipfs_hash,
            metadata: ObjectMetadata::Tree { entry_git_hashes },
        })
    }

      /// Download from IPFS and instantiate a `Object`.
    pub fn ipfs_get(hash: String, ipfs: &mut IpfsClient) -> Result<Vec<u8>, Error> {
       let req = ipfs
            .cat(hash.as_str())
            .map_ok(|chunk| chunk.to_vec())
            .try_concat();
        let content = futures::executor::block_on(req).map_err(|e| {
            error!("Could not cat ipfs file");
            debug!("Raw error: {}", e);
            // process::exit(1);
        })
        .unwrap();

        Ok(content)
    }

        /// Upload `odb_obj` to IPFS and return the link.
    fn upload_odb_obj(odb_obj: &OdbObject, ipfs: &mut IpfsClient) -> Result<String, Error> {
        let obj_buf = odb_obj.data().to_vec();

        let raw_data_req = ipfs.add(Cursor::new(obj_buf));
        let ipfs_hash = futures::executor::block_on(raw_data_req)?.hash;
        Ok(format!("/ipfs/{}", ipfs_hash))
    }

}