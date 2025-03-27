import starlightPlugin from '@astrojs/starlight-tailwind';

/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{astro,html,js,jsx,md,mdx,ts,tsx}'],
    theme: {
        extend: {
            fontFamily: {
                sans: ['"Inter Tight", "sans-serif'],
                mono: ['"JetBrains Mono"', 'monospace'],
            },
            colors: {
                accent: '#e8c547',
                // Your preferred gray scale. Zinc is closest to Starlight’s defaults.
                gray: '#9a9a9a',
            },
        }
    },
    plugins: [starlightPlugin()],
};