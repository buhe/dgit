use std::{str::FromStr, fmt::Error};

use log::{warn, debug};

pub struct Ref {
    pub src: String,
    pub dst: String,
}

impl FromStr for Ref {
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

        Ok(Ref {src: src.to_string(),dst: dst.to_string()})
    }
}