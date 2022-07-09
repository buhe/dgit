import { useAccount, useConnect, useDisconnect } from 'wagmi'

import { useIsMounted } from '../hooks/useIsMounted'
import { Button } from 'antd'

function Connect() {
    const isMounted = useIsMounted()
    const { connector, isConnected } = useAccount()
    const { connect, connectors, error, isLoading, pendingConnector } =
        useConnect()
    const { disconnect } = useDisconnect()

    return (
        <div style={{ display: 'flex', justifyContent: 'end', paddingRight: 44, paddingTop: 12 }}>
            <div>
                {isConnected && (
                    <Button onClick={() => disconnect()}>
                        Disconnect from {connector?.name}
                    </Button>
                )}

                {connectors
                    .filter((x) => isMounted && x.ready && x.id !== connector?.id)
                    .map((x) => (
                        <Button key={x.id} onClick={() => connect({ connector: x })}>
                            {x.name}
                            {isLoading && x.id === pendingConnector?.id && ' (connecting)'}
                        </Button>
                    ))}
            </div>

            {error && <div>{error.message}</div>}
        </div>
    )
}

export default Connect;