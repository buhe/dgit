import React, { Suspense } from 'react';
import { Outlet } from 'react-router-dom';

import { Breadcrumb } from 'antd';

// import Footer from '../components/Footer';
import Header from '../components/Header';

const Layout: React.FC = () => {
    return (
        <div>
            <Header />
            <Breadcrumb style={{
                paddingLeft: 20, paddingBottom: 12,paddingTop: 12
            }}>
                <Breadcrumb.Item>buhe</Breadcrumb.Item>
                <Breadcrumb.Item>test-repo</Breadcrumb.Item>
            </Breadcrumb>
            <Suspense fallback={'loading...'}>
                <Outlet />
            </Suspense>
            {/* <Footer /> */}
        </div>
    );
};

export default Layout;