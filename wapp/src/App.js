import { useEffect, useState } from 'react';
import Web3 from 'web3';

function App() {
  const [account, setAccount] = useState(); // state variable to set account.
  const [call, setCall] = useState(); // state variable to set account.

  useEffect(() => {
    async function load() {
      const web3 = new Web3(Web3.givenProvider || 'https://ropsten.infura.io/v3/3f433221d3db475db058b3875a617fdd');
      const accounts = await web3.eth.requestAccounts();

      setAccount(accounts[0]);
    }

    load();
  }, []);

  return (
    <div>
      Your account is: {account}
      Call: {call}
    </div>
  );
}

export default App;