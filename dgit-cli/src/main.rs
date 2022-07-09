use failure::Error;

mod issue;
mod meta;

use clap::{Args, Parser, Subcommand};

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
    Issue(Issue),
}

#[derive(Args)]
struct Issue {
    #[clap(value_parser)]
    title: Option<String>,
    #[clap(value_parser)]
    content: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error>{
    println!("Hello, world!");
    let cli = Cli::parse();

    match &cli.command {
        Commands::Issue(issue) => {
            println!("'issue' was used, name is: {:?}", issue.title)
        }
    }
    Ok(())
}
