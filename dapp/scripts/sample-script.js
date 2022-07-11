// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// When running the script with `npx hardhat run <script>` you'll find the Hardhat
// Runtime Environment's members available in the global scope.
const hre = require("hardhat");
const fs = require('fs');
const fsPromises = fs.promises;

async function main() {
  // Hardhat always runs the compile task when running scripts with its command
  // line interface.
  //
  // If this script is run directly using `node` you may want to call compile
  // manually to make sure everything is compiled
  // await hre.run('compile');

  // We get the contract to deploy
  const Greeter = await hre.ethers.getContractFactory("Greeter");
  const greeter = await Greeter.deploy();

  await greeter.deployed();

  console.log("Greeter deployed to:", greeter.address); // todo: write json file
  await writeRust(greeter.address, ['../dgit-cli/src/address.rs', '../git-remote-ipfs/src/address.rs']);
  await writeTypeScript(greeter.address, ['../webapp2/address.ts']);
  await greeter.setGreeting('hi bugu');
  console.log("call:", await greeter.greet());

  await greeter.addString('issue 1 hash');
  console.log("call:", await greeter.getStrings());
  console.log("call:", await greeter.getLength());
}

async function writeRust(address, files) {
  console.log('write ' + address + " to rust " + files);
  for(file of files) {
    await fsPromises.writeFile(file, 'pub const ADDRESS: &str = "' + address + '";\n');
  }
}

async function writeTypeScript(address, files) {
  console.log('write ' + address + " to typescriprt " + files);
  for (file of files) {
    await fsPromises.writeFile(file, 'export const ADDRESS = "' + address + '";\n');
  }
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
