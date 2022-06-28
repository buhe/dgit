use std::collections::BTreeMap;

pub struct Repo {
    /// All refs this repository knows; a {name -> sha1} mapping
    pub refs: BTreeMap<String, String>,
}