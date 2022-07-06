
import React from 'react';
import GlobalLayout from './pages/_layout'

const DynamicApp = React.lazy(() => import('./pages/App'));


export const routes = [
  {
    path: '/',
    element: <GlobalLayout />,
    children: [
      { path: '/App', element: <DynamicApp />, },
    ]
  }
]

export const pages = [
  { route: '/App' },
]
