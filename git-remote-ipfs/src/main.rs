use std::{ io::{self, BufRead, BufReader, Write}, process, env};

use env_logger::Builder;
use git2::Repository;
use log::{LevelFilter, trace, info, error, debug};

use crate::{wallet_connect::connect, ref_parse::Ref, repo::Repo};

mod wallet_connect;
mod repo;
mod object;
mod ref_parse;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_logging(LevelFilter::Trace);
    let mut args = env::args();
    trace!("Hello, world! {} {} {}", args.next().unwrap(), args.next().unwrap(), args.next().unwrap());
    let mut input_handle = BufReader::new(io::stdin());
    let mut output_handle = io::stdout();

    handle_capabilities(&mut input_handle, &mut output_handle)?;
    handle_list(&mut input_handle, &mut output_handle)?;
    // connect().await.unwrap();

    handle_fetches_and_pushes(&mut input_handle, &mut output_handle)?;
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
            let response = &mut ["push"].join("\n");
            response.push_str("\n\n");
            output_handle.write_all(response.as_bytes())?;
        }
        other => {
            println!("Received unexpected command {:?}", other);
        }
    }
    Ok(())
}

fn handle_list(
    input_handle: &mut dyn BufRead,
    output_handle: &mut dyn Write,
) -> std::io::Result<()> {
    let mut line_buf = String::new();
    input_handle.read_line(&mut line_buf)?;

    // Consume the command line
    match line_buf.as_str() {
        list if list.starts_with("list") => {
            // trace!("Consumed the \"list*\" command");
        }
        // Sometimes git needs to finish early, e.g. when the local ref doesn't even exist locally
        "\n" => {
            // debug!("Git finished early, exiting...");
            process::exit(0);
        }
        other => {
            let msg = format!("Expected a \"list*\" command, got {:?}", other);
            println!("{}", msg);
        }
    }

    output_handle.write_all(b"\n")?;

    
    Ok(())
}

fn handle_fetches_and_pushes(
    input_handle: &mut dyn BufRead,
    output_handle: &mut dyn Write) -> std::io::Result<()> {
         for line in input_handle.lines() {
        let line_buf = line?;
        match line_buf.as_str() {
            // fetch <sha> <ref_name>
            fetch_line if fetch_line.starts_with("fetch") => {
                trace!("Raw fetch line {:?}", fetch_line);
            }
            // push <refspec>
            push_line if push_line.starts_with("push") => {
                trace!("Raw push line {:?}", push_line);
                // Tell git we're done with this ref
                let r: Ref = push_line.parse().unwrap();
                let mut repo = Repo::default();
                let mut git_repo = Repository::open_from_env().unwrap();
                repo.push(&r.src, &r.dst, r.force, &mut git_repo).unwrap();

                debug!("repo:{:#?}", &repo);
                writeln!(output_handle, "ok {}", &r.dst)?;
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