use std::{ io::{self, BufRead, BufReader, Write}, process, env};

use env_logger::Builder;
use log::{LevelFilter, trace};

use crate::wallet_connect::connect;

mod wallet_connect;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_logging(LevelFilter::Trace);
    trace!("Hello, world!");
    let mut input_handle = BufReader::new(io::stdin());
    let mut output_handle = io::stdout();

    handle_capabilities(&mut input_handle, &mut output_handle).unwrap();
    handle_list(&mut input_handle, &mut output_handle).unwrap();
    connect().await.unwrap();
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

pub fn init_logging(default_lvl: LevelFilter) {
    match env::var("RUST_LOG") {
        Ok(_) => env_logger::init(),
        Err(_) => Builder::new().filter_level(default_lvl).init(),
    }
}