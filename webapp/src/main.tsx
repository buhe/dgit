import React from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter, useRoutes } from 'react-router-dom';
import { routes } from './routes'; // or use Vite's alias to simplify import path for nested components
import 'antd/dist/antd.css';

function App() {
  const element = useRoutes(routes);
  return element;
}

const root = ReactDOM.createRoot(
  document.getElementById("root")!
);
root.render(
  <BrowserRouter>
    <App />
  </BrowserRouter>
);