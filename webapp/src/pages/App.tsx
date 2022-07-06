import { useEffect, useState } from 'react';
import { AbiItem } from 'web3-utils'
import Web3 from 'web3';
const ABI = [
  {
    "inputs": [
      {
        "internalType": "string",
        "name": "_greeting",
        "type": "string"
      }
    ],
    "stateMutability": "nonpayable",
    "type": "constructor"
  },
  {
    "inputs": [],
    "name": "greet",
    "outputs": [
      {
        "internalType": "string",
        "name": "",
        "type": "string"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "string",
        "name": "_greeting",
        "type": "string"
      }
    ],
    "name": "setGreeting",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  }
];
const ADDRESS = '0x22fCB380773027B246b0EAfafC1f996938f2eF14';
function App() {
  const [account, setAccount] = useState(''); // state variable to set account.
  const [call, setCall] = useState(); // state variable to set account.

  useEffect(() => {
    async function load() {
      const web3 = new Web3(Web3.givenProvider || 'https://ropsten.infura.io/v3/3f433221d3db475db058b3875a617fdd');
      const accounts = await web3.eth.requestAccounts();

      setAccount(accounts[0]);


      // Instantiate smart contract using ABI and address.
      const contactList = new web3.eth.Contract(ABI as AbiItem[], ADDRESS);
      // await contactList.methods.setGreeting('hi').call();
      const greet = await contactList.methods.greet().call();
      console.log(greet);
      setCall(greet);
    }

    load();
  }, []);

  return (
    <div>
      Your account is: {account}
      Call: {JSON.stringify(call)}
    </div>
  );
}

export default App;