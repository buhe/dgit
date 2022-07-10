import React, { ReactNode } from 'react'
import Link from 'next/link'
import { useRouter } from 'next/router'
import { Breadcrumb } from 'antd';
import { SettingOutlined, CodeOutlined, BugOutlined, TagOutlined } from '@ant-design/icons';
import type { MenuProps } from 'antd';
import { Menu } from 'antd';
import Header from './Header';
const items: MenuProps['items'] = [
    {
        label: (<Link href="/Code">Code</Link>),
        key: '/Code',
        icon: <CodeOutlined />,
    },
    {
        label: (<Link href="/Issue">Issue</Link>),
        key: '/Issue',
        icon: <BugOutlined />,
    },
    {
        label: 'NFT',
        key: '/NFT',
        icon: <TagOutlined />,
    },
    {
        label: 'Setting',
        key: '/Setting',
        icon: <SettingOutlined />,
    },
];


type Props = {
    children?: ReactNode
    title?: string
}

const Layout = ({ children, title = 'This is the default title' }: Props) => {
    const router = useRouter()
    console.debug('Current router is ' + JSON.stringify(router))

    return (
        <div>
            <Header />
            <Breadcrumb style={{
                paddingLeft: 20, paddingBottom: 12,paddingTop: 12
            }}>
                <Breadcrumb.Item>buhe</Breadcrumb.Item>
                <Breadcrumb.Item>test-repo</Breadcrumb.Item>
            </Breadcrumb>
            <Menu selectedKeys={[router.pathname]} mode="horizontal" items={items} />
                {children}
        </div>
    );
};

export default Layout;