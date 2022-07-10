import React, { useEffect, useState } from 'react';
import CodeMirror from '@uiw/react-codemirror';
import { javascript } from '@codemirror/lang-javascript';
import { rust } from '@codemirror/lang-rust';
import Layout from '../components/Layout'
import { useContractRead } from 'wagmi'
import useIpfsFactory from '../hooks/use-ipfs-factory';
import { IPFS } from 'ipfs-core'
import { CID } from 'multiformats/cid'
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
    const KEY = 'QmW1fpwjve61tDSDni96FaMvFqNDPc43eusabahk1qdkxN'
    const readFile = async (ipfs: IPFS, cid: CID): Promise<string> => {
        const decoder = new TextDecoder()
        let content = ''
        for await (const chunk of ipfs.cat(cid)) {
            content += decoder.decode(chunk)
        }

        return content
    }

    const { ipfs, ipfsInitError } = useIpfsFactory()
    // // const id = ipfs && await ipfs.id();
    const [code, setCode] = useState(null)

    useEffect(() => {
        if (!ipfs) return;

        const getVersion = async () => {
            const nodeId = await ipfs.version();
            // setVersion(nodeId as any);
            console.info(data);
            console.info(nodeId);
            // const file = await ipfs.add({
            //     path: 'hello.txt',
            //     content: new TextEncoder().encode('Hello World bugu ipfs....')
            // })

            // console.log('Added file:', file.path, file.cid.toString())
            const cid = CID.parse(KEY);
            console.info(cid);
            const json = await readFile(ipfs, cid);
            console.info('json '+json);
        }

        getVersion();
    }, [data, ipfs])

    const onChange = React.useCallback((value: any, viewUpdate: any) => {
        console.log('value:', value);
    }, []);
    return (
        <Layout title="Home | Next.js + TypeScript Example">
            {/* <div>{JSON.stringify(version)}</div> */}
            {/* <div>{data}</div> */}
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