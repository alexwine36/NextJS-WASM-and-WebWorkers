import metaUrlPlugin from '@chialab/esbuild-plugin-meta-url';
import { defineConfig, type Options } from 'tsup';
export default defineConfig((options: Options) => ({
	// banner: {
	// 	js: "'use client'",
	// },
	dts: true,
	// clean: true,
	format: ['cjs', 'esm'],
	entryPoints: {
		index: 'src/index.ts',
		// textSearch: 'src/lib/textSearch.ts',
		// streamWorker: 'src/lib/streamWorker.ts',
	},
	// shims: true,
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	esbuildPlugins: [metaUrlPlugin() as any],
	...options,
}));
