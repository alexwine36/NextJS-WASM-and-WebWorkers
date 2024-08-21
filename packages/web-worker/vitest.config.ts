import react from '@vitejs/plugin-react';
// import wasm from 'vite-plugin-wasm';

import { defineConfig } from 'vitest/config';

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [
		react(),
		// wasm()
	],
	test: {
		environment: 'jsdom',
		globals: true,
	},
});
