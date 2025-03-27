// @ts-check
import {defineConfig} from 'astro/config';
import starlight from '@astrojs/starlight';
import tailwindcss from '@tailwindcss/vite';
import tailwind from '@astrojs/tailwind';
import tsconfigPaths from 'vite-tsconfig-paths'

import icon from 'astro-icon';

// https://astro.build/config
export default defineConfig({
    integrations: [starlight({
        title: 'LCAx Docs',
        favicon: '/favicon.svg',
        customCss: [
            './src/styles/global.css'
        ],
        social: {
            github: 'https://github.com/ocni-dtu/lcax',
        },
        components: {
            Header: './src/components/Header.astro',
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
                    {label: 'Data Structure', slug: 'guides/validation/validate'},
                    {
                        label: 'Conversion', items: [
                            {label: 'ILCD', slug: 'guides/conversion/python'},
                            {label: 'LCAByg', slug: 'guides/conversion/javascript'},
                            {label: 'Real-Time LCA', slug: 'guides/conversion/rust'},
                            {label: 'Custom Converter', slug: 'guides/conversion/rust'},
                        ]
                    },
                    {
                        label: 'Validation', items: [
                            {label: 'Validate', slug: 'guides/validation/validate'},
                            {label: 'Advanced', slug: 'guides/validation/advanced'},
                        ]
                    },
                    {
                        label: 'Calculation', items: [
                            {label: 'Calculate', slug: 'guides/validation/validate'},
                            {label: 'Results', slug: 'guides/validation/advanced'},
                        ]
                    },

                ],
            },
            {
                label: 'Reference',
                items: [
                    {label: 'Changelog', slug: 'reference/changelog'},
                    {
                        label: 'API', items: [
                            {label: 'JSON Schema', slug: 'reference/schemas/json'},
                            {label: 'Python', slug: 'reference/schemas/python'},
                            {label: 'TypeScript', slug: 'reference/schemas/javascript'},
                            {label: 'Rust', slug: 'reference/schemas/rust'},
                        ]
                    },
                ]
            },
        ],
    }), tailwind({
        applyBaseStyles: false,
        configFile: './tailwind.config.js',
    }), icon({
        iconDir: "node_modules/@tabler/icons/icons",
    })],

    vite: {
        plugins: [tailwindcss(), tsconfigPaths()],
    },
});