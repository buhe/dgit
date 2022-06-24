// use std::env;
use std::error::Error;
use log::info;
// use walletconnect::transport::WalletConnect;
use walletconnect::{qr, Client, Metadata};

use crate::test_url;
// use web3::types::TransactionRequest;
// use web3::Web3;


pub async fn connect() -> Result<(), Box<dyn Error>> {
    // env_logger::init();

    let client = Client::new(
        "examples-web3",
        Metadata {
            description: "WalletConnect-rs web3 transport example.".into(),
            url: "https://github.com/nlordell/walletconnect-rs".parse()?,
            icons: vec!["https://avatars0.githubusercontent.com/u/4210206".parse()?],
            name: "WalletConnect-rs Web3 Example".into(),
        },
    )?;

    let (accounts, _) = client.ensure_session(test_url::print_with_url).await?;

    info!("Connected accounts:");
    for account in &accounts {
        info!(" - {:?}", account);
    }

    // let tx = web3
    //     .eth()
    //     .send_transaction(TransactionRequest {
    //         from: accounts[0],
    //         to: Some("000102030405060708090a0b0c0d0e0f10111213".parse()?),
    //         value: Some(1_000_000_000_000_000u128.into()),
    //         ..TransactionRequest::default()
    //     })
    //     .await?;

    println!("Transaction sent:\n  https://ropsten.etherscan.io/address/{}", accounts[0]);
    // println!("Transaction sent:\n  https://etherscan.io/tx/{:?}", tx);

    Ok(())
}