use std::{str::FromStr, fmt::Error};

use log::{warn, debug};

pub struct PushRef {
    pub src: String,
    pub dst: String,
    pub force: bool,
}

impl FromStr for PushRef {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let refspec = s.split_whitespace().nth(1).ok_or_else(|| {
            format!("Could not read in refspec from push line: {:?}", &s)
        }).unwrap();

        let mut refspec_iter = refspec.split(':');

        let first_half = refspec_iter.next().ok_or_else(|| {
            format!("Could not read source ref from refspec: {:?}", refspec)
        }).unwrap();

        let force = first_half.starts_with('+');

        let src = if force {
            warn!("THIS PUSH WILL BE FORCED");
            &first_half[1..]
        } else {
            first_half
        };
        debug!("Parsed src: {}", src);

        let dst = refspec_iter.next().ok_or_else(|| {
            format!("Could not read destination ref from refspec: {:?}", refspec)
        }).unwrap();
        debug!("Parsed dst: {}", dst);

        Ok(Self {src: src.to_string(),dst: dst.to_string(), force})
    }
}

pub struct FetchRef {
    pub hash: String,
    pub ref_name: String,
}

impl FromStr for FetchRef {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hash = s.split_whitespace().nth(1).ok_or_else(|| {
            format!("Could not read in hash from fetch line: {:?}", &s)
        }).unwrap();

        debug!("Parsed hash: {}", hash);

        let ref_name = s.split_whitespace().nth(2).ok_or_else(|| {
            format!("Could not read in refspec from fetch line: {:?}", &s)
        }).unwrap();
        debug!("Parsed ref: {}", ref_name);

        Ok(Self {hash: hash.to_string(),ref_name: ref_name.to_string()})
    }
}