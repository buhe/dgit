use std::collections::BTreeSet;

#[derive(Clone, Debug, Deserialize, Serialize)]
/// representation of a git object
pub struct Object {
    /// The git hash of the underlying git object
    pub git_hash: String,
    /// A link to the raw form of the object
    pub raw_data_ipfs_hash: String,
    /// Object-type-specific metadata
    pub metadata: ObjectMetadata,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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