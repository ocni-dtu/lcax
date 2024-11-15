// @ts-check
import {defineConfig} from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
    integrations: [
        starlight({
            title: 'LCAx Docs',
            social: {
                github: 'https://github.com/ocni-dtu/lcax',
            },
            sidebar: [
                {
                    label: 'Concept',
                    items: [
                        {label: 'Introduction', slug: 'concept/introduction'},
                        {label: 'Why Rust', slug: 'concept/rust'},
                        {label: 'LCAx\'s Data Structure', slug: 'concept/data-structure'},
                        {label: 'LCAx\'s 4 Pillars', slug: 'concept/pillars'},
                    ]
                },
                {
                    label: 'Guides',
                    items: [
                        // Each item here is one entry in the navigation menu.
                        {label: 'Install LCAx', slug: 'guides/installation'},
                        {
                            label: 'Conversion', items: [
                                {label: 'Python', slug: 'guides/conversion/python'},
                                {label: 'TypeScript', slug: 'guides/conversion/javascript'},
                                {label: 'Rust', slug: 'guides/conversion/rust'},
                            ]
                        },
                        {
                            label: 'Verification', items: [
                                {label: 'Python', slug: 'guides/verification/python'},
                                {label: 'TypeScript', slug: 'guides/verification/javascript'},
                                {label: 'Rust', slug: 'guides/verification/rust'},
                            ]
                        },

                    ],
                },
                {
                    label: 'Reference',
                    items: [
                        {label: 'Changelog', slug: 'reference/changelog'},
                        {
                            label: 'Schemas', items: [
                                {label: 'C#', slug: 'reference/schemas/csharp'},
                                {label: 'Python', slug: 'reference/schemas/python'},
                                {label: 'TypeScript', slug: 'reference/schemas/javascript'},
                                {label: 'Rust', slug: 'reference/schemas/rust'},
                            ]
                        },
                    ]
                },
            ],
        }),
    ],
});
