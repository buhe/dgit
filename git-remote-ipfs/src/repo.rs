use std::str;
use std::{collections::{BTreeMap, HashSet}, io::Cursor};
use failure::Error;
use futures::TryStreamExt;
use git2::{Repository, ObjectType, Object, Oid, Odb};
use ipfs_api_backend_hyper::{IpfsClient, IpfsApi};
use log::{debug, error, trace, warn};

use crate::object::{GitObject, ObjectMetadata};
// serialize to json
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Repo {
    /// All refs this repository knows; a {name -> sha1} mapping
    pub refs: BTreeMap<String, String>,
    /// All objects this repository contains; a {sha1 -> {type,IPFS hash}} map
    pub objects: BTreeMap<String, GitObject>,
}

impl Repo {
    pub fn build(ipfs_hash: String, ipfs: &mut IpfsClient) -> Self {
        let req = ipfs
            .cat(ipfs_hash.as_str())
            .map_ok(|chunk| chunk.to_vec())
            .try_concat();
        let content = futures::executor::block_on(req).map_err(|e| {
            error!("Could not cat ipfs file");
            debug!("Raw error: {}", e);
            // process::exit(1);
        })
        .unwrap();

        let content = String::from_utf8(content).unwrap();

        debug!("build {}", content);

        serde_json::from_str(content.as_str()).unwrap()
    }

    pub fn find_all_objects   (&self,
        obj: &Object,
        push_todo: &mut HashSet<Oid>,
        repo: &Repository,
    ) -> Result<(), Error> {
        // Object tree traversal state
        let mut stack = vec![obj.clone()];
        let mut obj_cnt = 1;
        while let Some(obj) = stack.pop() {
            // if self.objects.contains_key(&obj.id().to_string()) {
            //     trace!("Object {} already in nip index", obj.id());
            //     continue;
            // }

            // if push_todo.contains(&obj.id()) {
            //     trace!("Object {} already in state", obj.id());
            //     continue;
            // }

            let obj_type = obj.kind().ok_or_else(|| {
                let msg = format!("Cannot determine type of object {}", obj.id());
                error!("{}", msg);
                format!("{}", msg)
            }).unwrap();

            push_todo.insert(obj.id());

            match obj_type {
                ObjectType::Commit => {
                    let commit = obj
                        .as_commit()
                        .ok_or_else(|| format!("Could not view {:?} as a commit", obj)).unwrap();
                    debug!("[{}] Counting commit {:?}", obj_cnt, commit);

                    let tree_obj = obj.peel(ObjectType::Tree)?;
                    trace!("Commit {}: Handling tree {}", commit.id(), tree_obj.id());

                    stack.push(tree_obj);

                    for parent in commit.parents() {
                        trace!(
                            "Commit {}: Pushing parent commit {}",
                            commit.id(),
                            parent.id()
                        );
                        stack.push(parent.into_object());
                    }
                }
                ObjectType::Tree => {
                    let tree = obj
                        .as_tree()
                        .ok_or_else(|| format!("Could not view {:?} as a tree", obj)).unwrap();
                    debug!("[{}] Counting tree {:?}", obj_cnt, tree);

                    for entry in tree.into_iter() {

                        trace!(
                            "Tree {}: Pushing tree entry {} ({:?})",
                            tree.id(),
                            entry.id(),
                            entry.kind()
                        );

                        stack.push(entry.to_object(&repo)?);
                    }
                }
                ObjectType::Blob => {
                    let blob = obj
                        .as_blob()
                        .ok_or_else(|| format!("Could not view {:?} as a blob", obj)).unwrap();
                    debug!("[{}] Counting blob {:?}", obj_cnt, blob);
                }
                ObjectType::Tag => {
                    let tag = obj
                        .as_tag()
                        .ok_or_else(|| format!("Could not view {:?} as a tag", obj)).unwrap();
                    debug!("[{}] Counting tag {:?}", obj_cnt, tag);

                    stack.push(tag.target()?);
                }
                other => {
                    error!("unknow git object type {}", other)
                }
            }

            obj_cnt += 1;
        }
        Ok(())
    }

     pub fn push(&mut self,
        ref_src: &str,
        ref_dst: &str,
        _force: bool,
        repo: &mut Repository,
        ipfs: &mut IpfsClient,
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

        let mut objs_for_push = HashSet::new();
        self.find_all_objects(&obj.clone(), &mut objs_for_push, repo)?;
        debug!("git object is {:#?}", objs_for_push);

        self.push_git_objects(&objs_for_push, repo, ipfs)?;
        self.refs
            .insert(ref_dst.to_owned(), format!("{}", obj.id()));
        Ok(())
    }

    pub fn push_git_objects(
        &mut self,
        oids: &HashSet<Oid>,
        repo: &Repository,
        ipfs: &mut IpfsClient,
    ) -> Result<(), Error> {
        let oid_count = oids.len();
        for (i, oid) in oids.iter().enumerate() {
            let obj = repo.find_object(*oid, None)?;
            trace!("Current object: {:?} at {}", obj.kind(), obj.id());

            // if self.objects.contains_key(&obj.id().to_string()) {
            //     warn!("push_objects: Object {} already in nip index", obj.id());
            //     continue;
            // }

            let obj_type = obj.kind().ok_or_else(|| {
                let msg = format!("Cannot determine type of object {}", obj.id());
                error!("{}", msg);
                format!("{}", msg)
            }).unwrap();

            match obj_type {
                ObjectType::Commit => {
                    let commit = obj
                        .as_commit()
                        .ok_or_else(|| format!("Could not view {:?} as a commit", obj)).unwrap();
                    trace!("Pushing commit {:?}", commit);

                    let object =
                        GitObject::from_git_commit(&commit, &repo.odb()?, ipfs)?;

                    self.objects
                        .insert(format!("{}", obj.id()), object.clone());

                    debug!(
                        "[{}/{}] Commit {} uploaded to",
                        i + 1,
                        oid_count,
                        obj.id(),
                        // nip_object_hash
                    );
                }
                ObjectType::Tree => {
                    let tree = obj
                        .as_tree()
                        .ok_or_else(|| format!("Could not view {:?} as a tree", obj)).unwrap();
                    trace!("Pushing tree {:?}", tree);
                    let object =
                        GitObject::from_git_tree(&tree, &repo.odb()?, ipfs)?;

                    self.objects
                        .insert(format!("{}", obj.id()), object.clone());

                    debug!(
                        "[{}/{}] Tree {} uploaded to",
                        i + 1,
                        oid_count,
                        obj.id(),
                        // nip_object_hash
                    );
                }
                ObjectType::Blob => {
                    let blob = obj
                        .as_blob()
                        .ok_or_else(|| format!("Could not view {:?} as a blob", obj)).unwrap();
                    trace!("Pushing blob {:?}", blob);

                    let object =
                        GitObject::from_git_blob(&blob, &repo.odb()?, ipfs)?;

                    self.objects
                        .insert(format!("{}", obj.id()), object.clone());


                    debug!(
                        "[{}/{}] Blob {} uploaded to",
                        i + 1,
                        oid_count,
                        obj.id(),
                        // nip_object_hash
                    );
                }
                ObjectType::Tag => {
                    let tag = obj
                        .as_tag()
                        .ok_or_else(|| format!("Could not view {:?} as a tag", obj)).unwrap();
                    trace!("Pushing tag {:?}", tag);

                    let object =
                        GitObject::from_git_tag(&tag, &repo.odb()?, ipfs)?;

                    self.objects
                        .insert(format!("{}", obj.id()), object.clone());


                    debug!(
                        "[{}/{}] Tag {} uploaded to",
                        i + 1,
                        oid_count,
                        obj.id(),
                        // nip_object_hash
                    );
                }
                other => {
                    error!(  "Don't know how to traverse a {}",
                        other);
                }
            }
        }
        Ok(())
    }

    pub fn save(&mut self,ipfs: &mut IpfsClient) -> Result<String, Error> {
        let self_buf = serde_json::to_string(self).unwrap();
        // Upload
        let add_req = ipfs.add(Cursor::new(self_buf));
        let new_hash = format!("/ipfs/{}", futures::executor::block_on(add_req)?.hash);
        debug!("hash is {}", new_hash);

        Ok(new_hash)
    }

    pub fn fetch(&self, hash: &str, ref_name: &str, git_repo: &mut Repository, ipfs: &mut IpfsClient) -> Result<(), Error> {
        debug!("Fetching {} for {}", hash, ref_name);

        let git_hash_oid = Oid::from_str(hash)?;
        let mut oids_for_fetch = HashSet::new();

        self.enumerate_for_fetch(git_hash_oid, &mut oids_for_fetch, git_repo)?;
    
        debug!(
            "Counted {} object(s) for fetch:\n{:#?}",
            oids_for_fetch.len(),
            oids_for_fetch
        );

        self.fetch_nip_objects(&oids_for_fetch, git_repo, ipfs)?;

        match git_repo.odb()?.read_header(git_hash_oid)?.1 {
            ObjectType::Commit if ref_name.starts_with("refs/tags") => {
                debug!("Not setting ref for lightweight tag {}", ref_name);
            }
            ObjectType::Commit => {
                git_repo.reference(ref_name, git_hash_oid, true, "ipfs fetch")?;
            }
            // Somehow git is upset when we set tag refs for it
            ObjectType::Tag => {
                debug!("Not setting ref for tag {}", ref_name);
            }
            other_type => {
                let msg = format!("New tip turned out to be a {} after fetch", other_type);
                error!("{}", msg);
            }
        }

        debug!("Fetched {} for {} OK.", hash, ref_name);
        Ok(())
    }

     /// Fill a hash set with `oid`'s children that are present in `self` but missing in `repo`.
    pub fn enumerate_for_fetch(
        &self,
        oid: Oid,
        fetch_todo: &mut HashSet<Oid>,
        repo: &Repository,
    ) -> Result<(), Error> {
        let mut stack = vec![oid];
        let mut obj_cnt = 1;

        while let Some(oid) = stack.pop() {
            if repo.odb()?.read_header(oid).is_ok() {
                trace!("Object {} already present locally!", oid);
                continue;
            }

            if fetch_todo.contains(&oid) {
                trace!("Object {} already present in state!", oid);
                continue;
            }

            let git_object = self
                .objects
                .get(&format!("{}", oid))
                .ok_or_else(|| {
                    let msg = format!("Could not find object {} in the index", oid);
                    error!("{}", msg);
                    format!("{}", msg)
                }).unwrap()
                .clone();

            // if nip_obj_ipfs_hash == SUBMODULE_TIP_MARKER {
            //     debug!("Ommitting submodule {}", oid.to_string());
            //     return Ok(());
            // }

            fetch_todo.insert(oid);

            // let nip_obj = NIPObject::ipfs_get(&nip_obj_ipfs_hash, ipfs)?;

            match git_object.clone().metadata {
                ObjectMetadata::Commit {
                    parent_git_hashes,
                    tree_git_hash,
                } => {
                    debug!("[{}] Counting nip commit {}", obj_cnt, git_object.raw_data_ipfs_hash);

                    stack.push(Oid::from_str(&tree_git_hash)?);

                    for parent_git_hash in parent_git_hashes {
                        stack.push(Oid::from_str(&parent_git_hash)?);
                    }
                }
                ObjectMetadata::Tag { target_git_hash } => {
                    debug!("[{}] Counting nip tag {}", obj_cnt, git_object.raw_data_ipfs_hash);

                    stack.push(Oid::from_str(&target_git_hash)?);
                }
                ObjectMetadata::Tree { entry_git_hashes } => {
                    debug!("[{}] Counting nip tree {}", obj_cnt, git_object.raw_data_ipfs_hash);

                    for entry_git_hash in entry_git_hashes {
                        stack.push(Oid::from_str(&entry_git_hash)?);
                    }
                }
                ObjectMetadata::Blob => {
                    debug!("[{}] Counting nip blob {}", obj_cnt, git_object.raw_data_ipfs_hash);
                }
            }
            obj_cnt += 1;
        }

        Ok(())
    }

    /// Download git objects in `oids` from IPFS and instantiate them in `repo`.
    pub fn fetch_nip_objects(
        &self,
        oids: &HashSet<Oid>,
        repo: &mut Repository,
        ipfs: &mut IpfsClient,
    ) -> Result<(), Error> {
        for (i, &oid) in oids.iter().enumerate() {
            debug!("[{}/{}] Fetching object {}", i + 1, oids.len(), oid);

            let git_object = self.objects.get(&format!("{}", oid)).ok_or_else(|| {
                let msg = format!("Could not find object {} in nip index", oid);
                error!("{}", msg);
                format!("{}", msg)
            }).unwrap();

            let content = GitObject::ipfs_get(git_object.raw_data_ipfs_hash.clone(), ipfs)?;

            trace!("git object is {:#?}", git_object);

            if repo.odb()?.read_header(oid).is_ok() {
                warn!("fetch_nip_objects: Object {} already present locally!", oid);
                continue;
            }

            let written_oid = write_raw_data(&mut repo.odb()?, &content[..], &git_object.metadata)?;
            if written_oid != oid {
                let msg = format!("Object tree inconsistency detected: fetched {} from {}, but write result hashes to {}", oid, git_object.raw_data_ipfs_hash, written_oid);
                error!("{}", msg);
            }
            trace!("Fetched object {} to {}", git_object.raw_data_ipfs_hash, written_oid);
        }
        Ok(())
    }




}
    fn write_raw_data(odb: &mut Odb, content: &[u8], metadata: &ObjectMetadata) -> Result<Oid, Error> {

        let obj_type = match metadata {
            ObjectMetadata::Blob => ObjectType::Blob,
            ObjectMetadata::Commit { .. } => ObjectType::Commit,
            ObjectMetadata::Tag { .. } => ObjectType::Tag,
            ObjectMetadata::Tree { .. } => ObjectType::Tree,
        };

        Ok(odb.write(obj_type, content)?)
    }

impl Default for Repo{
    fn default() -> Self {
        Self { refs: Default::default(), objects: Default::default() }
    }
}