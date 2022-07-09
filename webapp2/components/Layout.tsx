import React, { ReactNode } from 'react'
import Link from 'next/link'
import { Suspense, useState } from 'react';
// import { Outlet } from 'react-router-dom';

import { Breadcrumb } from 'antd';
// import { Route, Link, useParams } from 'react-router-dom';
import { SettingOutlined, CodeOutlined, BugOutlined, TagOutlined } from '@ant-design/icons';
import type { MenuProps } from 'antd';
import { Menu } from 'antd';

// import Footer from '../components/Footer';
import Header from './Header';
const items: MenuProps['items'] = [
    {
        label: (<Link href="/Code">Code</Link>),
        // label: (<Link to="/Code">Code</Link>),
        key: 'code',
        icon: <CodeOutlined />,
    },
    {
        label: 'Issue',
        key: 'issue',
        icon: <BugOutlined />,
    },
    {
        label: 'NFT',
        key: 'nft',
        icon: <TagOutlined />,
    },
    {
        label: 'Setting',
        key: 'setting',
        icon: <SettingOutlined />,
    },
];


type Props = {
    children?: ReactNode
    title?: string
}

const Layout = ({ children, title = 'This is the default title' }: Props) => {
    const [current, setCurrent] = useState('code');

    const onClick: MenuProps['onClick'] = e => {
        console.log('click ', e);
        setCurrent(e.key);
    };

    return (
        <div>
            <Header />
            <Breadcrumb style={{
                paddingLeft: 20, paddingBottom: 12,paddingTop: 12
            }}>
                <Breadcrumb.Item>buhe</Breadcrumb.Item>
                <Breadcrumb.Item>test-repo</Breadcrumb.Item>
            </Breadcrumb>
            <Menu onClick={onClick} selectedKeys={[current]} mode="horizontal" items={items} />
            {/* <Suspense fallback={'loading...'}> */}
                {children}
            {/* </Suspense> */}
            {/* <Footer /> */}
        </div>
    );
};

export default Layout;