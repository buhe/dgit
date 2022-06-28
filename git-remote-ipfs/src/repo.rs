use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Repo {
    /// All refs this repository knows; a {name -> sha1} mapping
    pub refs: BTreeMap<String, String>,
}

impl Repo {
    pub fn find_all_objects(&mut self, top: String){

    }
}

impl Default for Repo{
    fn default() -> Self {
        Self { refs: Default::default() }
    }
}