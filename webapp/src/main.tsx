import React from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter, useRoutes } from 'react-router-dom';
import { routes } from './routes'; // or use Vite's alias to simplify import path for nested components
import 'antd/dist/antd.css';
import { WagmiConfig, createClient } from 'wagmi'
import { getDefaultProvider } from 'ethers'

const client = createClient({
  autoConnect: true,
  provider: getDefaultProvider(),
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