'use client';

import dynamic from 'next/dynamic';
import { useEffect, useState } from 'react';

import { pipeline } from '@huggingface/transformers';

export default dynamic(
    async function Page() {
        const { default: init, greet } = await import('../chatlock-rs/pkg');
        await init();

        const extractor = await pipeline(
            'feature-extraction',
            'Xenova/all-MiniLM-L6-v2'
        );

        const response = await extractor(
            [
                'A robot may not injure a human being or, through inaction, allow a human being to come to harm.',
            ],
            { pooling: 'mean', normalize: true }
        );

        console.log(Array.from(response.data));

        return function PageComponentLoaded() {
            const [greeting, setGreeting] = useState('Hello, world!');
            useEffect(() => {
                setGreeting(greet('User'));
            }, []);

            return <p>{greeting}</p>;
        };
    },
    {
        ssr: false,
        loading: () => <p>Loading WASM...</p>,
    }
);
