import { Route, Link, useParams } from 'react-router-dom';
// import React from 'react';

// const component: React.FC = () => {
//     return (
//         <div>
//             <ul>
//                 <li>
//                     <Link to="/App">App</Link>
//                 </li>
//             </ul>
//         </div>
//     );
// };

// export default component;

import { AppstoreOutlined, MailOutlined, SettingOutlined } from '@ant-design/icons';
import type { MenuProps } from 'antd';
import { Menu } from 'antd';
import React, { useState } from 'react';

const items: MenuProps['items'] = [
    {
        label: 'Code',
        key: 'code',
        icon: <MailOutlined />,
    },
    {
        label: (<Link to="/App">Issue</Link>),
        key: 'issue',
        icon: <AppstoreOutlined />,
    },
    {
        label: 'Setting',
        key: 'setting',
        icon: <SettingOutlined />,
    },
];

const App: React.FC = () => {
    const [current, setCurrent] = useState('code');

    const onClick: MenuProps['onClick'] = e => {
        console.log('click ', e);
        setCurrent(e.key);
    };

    return <Menu onClick={onClick} selectedKeys={[current]} mode="horizontal" items={items} />;
};

export default App;