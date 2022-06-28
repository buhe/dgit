use std::collections::BTreeMap;

use git2::{Repository, Error, ObjectType};
use ipfs_api_backend_hyper::IpfsClient;
use log::debug;

#[derive(Debug)]
pub struct Repo {
    /// All refs this repository knows; a {name -> sha1} mapping
    pub refs: BTreeMap<String, String>,
}

impl Repo {
    pub fn find_all_objects(&mut self, top: String){

    }

    pub fn push(&mut self,
        ref_src: &str,
        ref_dst: &str,
        force: bool,
        repo: &mut Repository,
        // ipfs: &mut IpfsClient,
    ) -> Result<(), Error> {

        let reference = repo.find_reference(ref_src)?.resolve()?;

        // Differentiate between annotated tags and their commit representation
        let obj = reference
            .peel(ObjectType::Tag)
            .unwrap_or(reference.peel(ObjectType::Commit)?);

        debug!(
            "{:?} dereferenced to {:?} {}",
            reference.shorthand(),
            obj.kind(),
            obj.id()
        );

        Ok(())
    }
}

impl Default for Repo{
    fn default() -> Self {
        Self { refs: Default::default() }
    }
}