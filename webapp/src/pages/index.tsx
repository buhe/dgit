import { Route, Link, useParams } from 'react-router-dom';
import React from 'react';

const component: React.FC = () => {
    return (
        <div>
            <ul>
                <li>
                    <Link to="/App">App</Link>
                </li>
            </ul>
        </div>
    );
};

export default component;