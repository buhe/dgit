// use std::env;
use std::error::Error;
use std::str::FromStr;
use log::info;
use walletconnect::transport::WalletConnect;
use walletconnect::{qr, Client, Metadata, H160};
use web3::contract::{Contract, Options};
use web3::transports::Http;
use web3::types::Address;
use web3::Web3;

pub struct Wallet {
    contract: Option<Contract<WalletConnect<Http>>>,
    account: Option<H160>,
}

impl Wallet {
    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {

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

        self.account = Some(accounts[0]);

            // Get current balance
        let balance = web3.eth().balance(accounts[0], None).await?;

        info!("Balance: {}", balance);

        info!("Transaction sent:\n  https://ropsten.etherscan.io/address/{}", accounts[0]);
        let addr = Address::from_str("0x22fCB380773027B246b0EAfafC1f996938f2eF14").unwrap();
        self.contract =
            Some(Contract::from_json(web3.eth(), addr, include_bytes!("./abi/contracts/Greeter.sol/Greeter.json")).unwrap());

        Ok(())
    }

    pub async fn save(&self, hash: String) -> Result<(), Box<dyn Error>> {

        let tx = self.contract.as_ref().unwrap()
            .call("setGreeting", hash, self.account.unwrap(), Options::default())
            .await
            .unwrap();

        info!("tx is {}", tx);

        Ok(())
    }


    pub async fn load(&self) -> Result<String, Box<dyn Error>> {
        let greet: String = self.contract.as_ref().unwrap()
            .query("greet", (), None, Options::default(), None)
            .await
            .unwrap();

        info!("greet {}", greet);
        Ok(greet)
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self{ contract: None, account: None }
    }
}
