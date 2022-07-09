import { useAccount, useConnect, useDisconnect } from 'wagmi'
import { InjectedConnector } from 'wagmi/connectors/injected'

import { Button } from 'antd';

function Profile() {
    const { address } = useAccount()
    const { connect } = useConnect({
        connector: new InjectedConnector(),
    })
    const { disconnect } = useDisconnect()

    if (address)
        return (
            <div style={{display: 'flex', justifyContent: 'end',paddingRight: 44, paddingTop: 12}}>
                <Button onClick={() => disconnect()}>Disconnect</Button>
            </div>
        )
    return (
        <div style={{ display: 'flex', justifyContent: 'end', paddingRight: 44, paddingTop: 12 }}>
            <Button onClick={() => connect()}>Connect Wallet</Button>
        </div>
    )
}

export default Profile;