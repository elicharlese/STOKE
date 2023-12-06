import React from 'react';
import DEXComponent1 from './DEXComponent1';
import DEXComponent2 from './DEXComponent2';
import DEXComponent3 from './DEXComponent3';

const DEX: React.FC = () => {
    return (
        <div>
            <h1>DEX Page</h1>
            <div>
                <DEXComponent1 />
            </div>
            <div>
                <DEXComponent2 />
            </div>
            <div>
                <DEXComponent3 />
            </div>
        </div>
    );
};

export default DEX;
