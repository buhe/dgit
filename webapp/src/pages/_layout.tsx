import React, { Suspense } from 'react';
import { Outlet } from 'react-router-dom';
// import Footer from '../components/Footer';
import Header from '../components/Header';

const Layout: React.FC = () => {
    return (
        <div>
            <Header />
            <Suspense fallback={'loading...'}>
                <Outlet />
            </Suspense>
            {/* <Footer /> */}
        </div>
    );
};

export default Layout;