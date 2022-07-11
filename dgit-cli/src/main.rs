use std::{env};

use env_logger::Builder;
use failure::Error;

mod issue;
mod address;

use clap::{Args, Parser, Subcommand};
use issue::Issue;
use log::{LevelFilter, debug};

use crate::address::ADDRESS;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Issue management
    Issue(IssueArg),
}

#[derive(Args)]
struct IssueArg {
    #[clap(value_parser)]
    title: Option<String>,
    #[clap(value_parser)]
    content: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // println!("Hello, world!");
    init_logging(LevelFilter::Trace);
    debug!("address is {}", ADDRESS);
    let cli = Cli::parse();

    match &cli.command {
        Commands::Issue(arg) => {
            // trace!("'issue' was used, name is: {:?}", issue.title);
            // let ipfs = IpfsClient::default();
            // let raw_data_req = ipfs.add(Cursor::new(issue.title.clone().unwrap()));
            // let ipfs_hash = futures::executor::block_on(raw_data_req)?.hash;
            // debug!("hash {}", ipfs_hash);
            let issue = Issue{title: arg.title.clone().unwrap(), content: arg.content.clone().unwrap(), comments_hash: vec![]};
            issue.add().await?;
        }
    }
    Ok(())
}

pub fn init_logging(default_lvl: LevelFilter) {
    match env::var("RUST_LOG") {
        Ok(_) => env_logger::init(),
        Err(_) => Builder::new().filter_level(default_lvl).init(),
    }
}
