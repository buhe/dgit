import React, { useEffect, useState } from 'react';
import CodeMirror from '@uiw/react-codemirror';
import { javascript } from '@codemirror/lang-javascript';
import { rust } from '@codemirror/lang-rust';
import Layout from '../components/Layout'
import { useContractRead } from 'wagmi'
import useIpfsFactory from '../hooks/use-ipfs-factory';
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
    const { data, isError, isLoading } = useContractRead({
        addressOrName: ADDRESS,
        contractInterface: ABI,
        functionName: 'greet',
    })

    const { ipfs, ipfsInitError } = useIpfsFactory()
    // // const id = ipfs && await ipfs.id();
    const [version, setVersion] = useState(null)

    useEffect(() => {
        if (!ipfs) return;

        const getVersion = async () => {
            const nodeId = await ipfs.version();
            setVersion(nodeId as any);
        }

        getVersion();
    }, [ipfs])

    const onChange = React.useCallback((value: any, viewUpdate: any) => {
        console.log('value:', value);
    }, []);
    return (
        <Layout title="Home | Next.js + TypeScript Example">
            <div>{JSON.stringify(version)}</div>
            <div>{data}</div>
            <CodeMirror
                value="console.log('hello world!');"
                height="600px"
                extensions={[javascript({ jsx: true }), rust()]}
                onChange={onChange}
            />
        </Layout>
    );
}
export default App;