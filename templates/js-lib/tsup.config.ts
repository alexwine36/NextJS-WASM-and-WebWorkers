import { defineConfig, type Options } from 'tsup';

export default defineConfig((options: Options) => ({
	dts: true,
	format: ['cjs', 'esm'],
	entryPoints: {
		index: 'src/index.ts',
	},
	...options,
}));
