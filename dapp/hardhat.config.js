require("@nomiclabs/hardhat-waffle");
require('hardhat-abi-exporter');

// This is a sample Hardhat task. To learn how to create your own go to
// https://hardhat.org/guides/create-task.html
task("accounts", "Prints the list of accounts", async (taskArgs, hre) => {
  const accounts = await hre.ethers.getSigners();

  for (const account of accounts) {
    console.log(account.address);
  }
});

// You need to export an object to set up your config
// Go to https://hardhat.org/config/ to learn more

/**
 * @type import('hardhat/config').HardhatUserConfig
 */

// Go to https://www.alchemyapi.io, sign up, create
// a new App in its dashboard, and replace "KEY" with its key
const ALCHEMY_API_KEY = "bjJEnGWhmiAGmUm7fMp9BE9oyJq4OSMO";

// Replace this private key with your Ropsten account private key
// To export your private key from Metamask, open Metamask and
// go to Account Details > Export Private Key
// Be aware of NEVER putting real Ether into testing accounts
const ROPSTEN_PRIVATE_KEY = "1a4240d88f6f90f5ca8f73345a825b09154f2ddcdf4269a703350e507454f7ac";

module.exports = {
  solidity: "0.8.4",
  networks: {
    ropsten: {
      url: `https://eth-ropsten.alchemyapi.io/v2/${ALCHEMY_API_KEY}`,
      accounts: [`0x${ROPSTEN_PRIVATE_KEY}`]
    }
  },
  abiExporter: [
    {
      path: '../dgit-cli/src/abi',
      runOnCompile: true,
    },
    {
      path: '../git-remote-ipfs/src/abi',
      runOnCompile: true,
    },
    {
      path: '../webapp2/abi',
      runOnCompile: true,
    }
  ]
};
