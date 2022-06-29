use std::collections::{BTreeMap, HashSet};

use git2::{Repository, Error, ObjectType, Object, Oid};
use ipfs_api_backend_hyper::IpfsClient;
use log::{debug, error, trace};

#[derive(Debug)]
pub struct Repo {
    /// All refs this repository knows; a {name -> sha1} mapping
    pub refs: BTreeMap<String, String>,
}

impl Repo {
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
                    error!("unknow git object type")
                }
            }

            obj_cnt += 1;
        }
        Ok(())
    }

     pub fn push_git_objects(
        &mut self,
        oids: &HashSet<Oid>,
        repo: &Repository,
        ipfs: &mut IpfsClient,
    ) -> Result<(), Error> {
        Ok(())
    }

    pub fn push(&mut self,
        ref_src: &str,
        ref_dst: &str,
        force: bool,
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

        Ok(())
    }
}

impl Default for Repo{
    fn default() -> Self {
        Self { refs: Default::default() }
    }
}