
import React from 'react';
import GlobalLayout from './pages/_layout'

const DynamicIndex = React.lazy(() => import('./pages/index'));
const DynamicApp = React.lazy(() => import('./pages/App'));


export const routes = [
  {
    path: '/',
    element: <GlobalLayout />,
    children: [
      { path: '/', element: <DynamicIndex />, index: true},
      { path: '/App', element: <DynamicApp />, },
    ]
  }
]

export const pages = [
  { route: '/' },
  { route: '/App' },
]
