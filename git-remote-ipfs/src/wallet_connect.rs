// use std::env;
use std::error::Error;
use log::info;
use walletconnect::transport::WalletConnect;
use walletconnect::{qr, Client, Metadata};
use web3::types::TransactionRequest;
use web3::Web3;


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


    client.ensure_session(qr::print_with_url).await?;

    let wc = WalletConnect::new(client, "3f433221d3db475db058b3875a617fdd")?;
    let web3 = Web3::new(wc);
    
    let accounts = web3.eth().accounts().await?;
    info!("Connected accounts:");
    for account in &accounts {
        info!(" - {:?}", account);
    }

        // Get current balance
    let balance = web3.eth().balance(accounts[0], None).await?;

    info!("Balance: {}", balance);

    // let tx = web3
    //     .eth()
    //     .send_transaction(TransactionRequest {
    //         from: accounts[0],
    //         to: Some("000102030405060708090a0b0c0d0e0f10111213".parse()?),
    //         value: Some(1_000_000_000_000_000u128.into()),
    //         ..TransactionRequest::default()
    //     })
    //     .await?;

    info!("Transaction sent:\n  https://ropsten.etherscan.io/address/{}", accounts[0]);
    // web3.eth().call(req, block)
    // info!("Transaction sent:\n  https://etherscan.io/tx/{:?}", tx);

    Ok(())
}