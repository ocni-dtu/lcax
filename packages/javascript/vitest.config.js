/// <reference types="vitest" />
/// <reference types="vite/client" />
import {defineConfig} from 'vitest/config'
import wasm from "vite-plugin-wasm";

export default defineConfig({
    plugins: [
        wasm(),
    ]
});