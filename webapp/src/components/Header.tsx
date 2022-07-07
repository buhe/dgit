import { useState } from 'react';
import Web3 from 'web3';
import { Button } from 'antd';

function App() {
    const [account, setAccount] = useState(''); // state variable to set account.
    const [connected, setConnected] = useState(false); // state variable to set account.

    async function connect() {
        try {
            const web3 = new Web3(Web3.givenProvider || 'https://ropsten.infura.io/v3/3f433221d3db475db058b3875a617fdd');
            setConnected(true);
            const accounts = await web3.eth.requestAccounts();

            setAccount(accounts[0]);
        } catch (e) {
            setConnected(false);
        }
    }

    return (
        <div>
            {connected ? <Button>Disconnect {account} </Button> : <Button onClick={() => connect()}>Connect</Button>}
        </div>
    );
}

export default App;