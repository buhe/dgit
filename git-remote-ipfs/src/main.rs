use std::{ io::{self, BufRead, BufReader, Write}, process, env};

use env_logger::Builder;
use git2::Repository;
use ipfs_api_backend_hyper::{IpfsClient, IpfsApi};
use log::{LevelFilter, trace, error, debug};

use crate::{ref_parse::{PushRef, FetchRef}, repo::Repo};
use crate::wallet_connect::Wallet;

#[macro_use]
extern crate serde_derive;

mod wallet_connect;
mod repo;
mod object;
mod ref_parse;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    init_logging(LevelFilter::Trace);
    let mut args = env::args();
    trace!("Hello, world! {} {} {}", args.next().unwrap(), args.next().unwrap(), args.next().unwrap());

    let mut ipfs = IpfsClient::default();

// Execute the future, blocking the current thread until completion
    let stats = futures::executor::block_on(ipfs.stats_repo())
        .map_err(|e| {
            error!("Could not connect to IPFS, are you sure `ipfs daemon` is running?");
            debug!("Raw error: {}", e);
            process::exit(1);
        })
        .unwrap();

    debug!("IPFS connectivity OK. Datastore stats:\n{:#?}", stats);

    let mut wallet = Wallet::default();
    wallet.connect().await.unwrap();

    let ipfs_hash = wallet.load().await.unwrap();// todo: when repo not push, hash is none
    debug!("repo ipfs hash is {}", ipfs_hash);
    let mut repo = Repo::build(ipfs_hash, &mut ipfs);
    
    let mut input_handle = BufReader::new(io::stdin());
    let mut output_handle = io::stdout();

    handle_capabilities(&mut input_handle, &mut output_handle)?;
    handle_list(&mut input_handle, &mut output_handle, &repo)?;
    // connect().await.unwrap();

    handle_fetches_and_pushes(&mut input_handle, &mut output_handle, &mut ipfs, &mut repo, &mut wallet).await?;
    // Ok(for line in input_handle.lines() {
    //     let line_buf = line?;
    //     match line_buf.as_str() {
    //         _ => println!("{}", line_buf),
    //     }
    // })
    Ok(())
}

fn handle_capabilities(input_handle: &mut dyn BufRead, output_handle: &mut dyn Write) -> std::io::Result<()> {
    let mut line_buf = String::new();
    input_handle.read_line(&mut line_buf)?;
    match line_buf.as_str() {
        "capabilities\n" => {
            let response = &mut ["push", "fetch"].join("\n");
            response.push_str("\n\n");
            output_handle.write_all(response.as_bytes())?;
            debug!("call capabilities");
        }
        other => {
            debug!("Received unexpected command {:?}", other);
        }
    }
    Ok(())
}

fn handle_list(
    input_handle: &mut dyn BufRead,
    output_handle: &mut dyn Write,
    repo: &Repo,
) -> std::io::Result<()> {
    let mut line_buf = String::new();
    input_handle.read_line(&mut line_buf)?;

    // Consume the command line
    match line_buf.as_str() {
        list if list.starts_with("list") => {
            trace!("Consumed the \"list*\" command {}", list);
        }
        // Sometimes git needs to finish early, e.g. when the local ref doesn't even exist locally
        "\n" => {
            debug!("Git finished early, exiting...");
            process::exit(0);
        }
        other => {
            let msg = format!("Expected a \"list*\" command, got {:?}", other);
            error!("{}", msg);
        }
    }

    // output_handle.write_all(b"\n")?;

    for (name, git_hash) in &repo.refs {
        let output = format!("{} {}", git_hash, name);
        debug!("fetch from {}", output);
        writeln!(output_handle, "{}", output)?;
    }
    // output_handle.write_all(b"\n")?;

    // // Indicate that we're done listing
    // // writeln!(output_handle,"refs/heads/master HEAD")?;
    writeln!(output_handle)?;

    
    Ok(())
}

async fn handle_fetches_and_pushes(
    input_handle: &mut dyn BufRead,
    output_handle: &mut dyn Write,
    ipfs: &mut IpfsClient,
    repo: &mut Repo,
    wallet: &mut Wallet,
) -> std::io::Result<()> {
    
   
    for line in input_handle.lines() {
        let line_buf = line?;
        match line_buf.as_str() {
            // fetch <sha> <ref_name>
            fetch_line if fetch_line.starts_with("fetch") => {
                trace!("Raw fetch line {:?}", fetch_line);
                let r: FetchRef = fetch_line.parse().unwrap();
                
                let mut git_repo = Repository::open_from_env().unwrap();

                repo.fetch(&r.hash, &r.ref_name, &mut git_repo, ipfs).unwrap();

                debug!("fetch repo:{:#?} done.", &repo);
            }
            // push <refspec>
            push_line if push_line.starts_with("push") => {
                trace!("Raw push line {:?}", push_line);
                // Tell git we're done with this ref
                let r: PushRef = push_line.parse().unwrap();
                
                let mut git_repo = Repository::open_from_env().unwrap();

                repo.push(&r.src, &r.dst, r.force, &mut git_repo, ipfs).unwrap();

                debug!("push repo:{:#?} done.", &repo);
                writeln!(output_handle, "ok {}", &r.dst)?;
                let repo_ipfs_hash = repo.save(ipfs).unwrap();
                wallet.save(repo_ipfs_hash).await.unwrap();
            }
            clone_line if clone_line.starts_with("clone") => {
                trace!("Raw clone line {:?}", clone_line);
            }
            // The lines() iterator clips the newline by default, so the last line match is ""
            "" => {
                trace!("Consumed all fetch/push commands");
                break;
            }
            other => {
                let msg = format!(
                    "Git unexpectedly said {:?} during push/fetch parsing.",
                    other
                );
                error!("{}", msg);
            }
        }
    }



    // Upload current_idx to IPFS if it differs from the original idx
    // Tell git that we're done
    writeln!(output_handle)?;
    Ok(())
}

pub fn init_logging(default_lvl: LevelFilter) {
    match env::var("RUST_LOG") {
        Ok(_) => env_logger::init(),
        Err(_) => Builder::new().filter_level(default_lvl).init(),
    }
}