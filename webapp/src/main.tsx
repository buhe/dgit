import React from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter, useRoutes } from 'react-router-dom';
import { routes } from './routes'; // or use Vite's alias to simplify import path for nested components
import 'antd/dist/antd.css';
import { WagmiConfig, createClient } from 'wagmi'
import { getDefaultProvider } from 'ethers'
import { chain, configureChains } from 'wagmi'
import { infuraProvider } from 'wagmi/providers/infura'
import { CoinbaseWalletConnector } from 'wagmi/connectors/coinbaseWallet'
import { InjectedConnector } from 'wagmi/connectors/injected'
import { MetaMaskConnector } from 'wagmi/connectors/metaMask'
import { WalletConnectConnector } from 'wagmi/connectors/walletConnect'

const infuraId = '3f433221d3db475db058b3875a617fdd'
const { chains, provider } = configureChains(
  [chain.ropsten],
  [infuraProvider({ infuraId })],
)
const client = createClient({
  autoConnect: true,
  connectors: [
    new MetaMaskConnector({ chains }),
    new CoinbaseWalletConnector({
      chains,
      options: {
        appName: 'dgit',
      },
    }),
    new WalletConnectConnector({
      chains,
      options: {
        qrcode: true,
      },
    }),
    new InjectedConnector({
      chains,
      options: {
        name: 'Injected',
        shimDisconnect: true,
      },
    }),
  ],
  provider,
})

function App() {
  const element = useRoutes(routes);
  return element;
}

const root = ReactDOM.createRoot(
  document.getElementById("root")!
);
root.render(
  <BrowserRouter>
    <WagmiConfig client={client}>
    <App />
    </WagmiConfig>
  </BrowserRouter>
);