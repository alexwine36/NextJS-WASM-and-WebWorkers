// @ts-check

import pluginJs from '@eslint/js';
import pluginReact from 'eslint-plugin-react';
import globals from 'globals';
import tseslint from 'typescript-eslint';

import { FlatCompat } from '@eslint/eslintrc';
import path from 'path';
import { fileURLToPath } from 'url';

// mimic CommonJS variables -- not needed if using CommonJS
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const compat = new FlatCompat({
	baseDirectory: __dirname,
});

export default tseslint.config(
	{ files: ['**/*.{js,mjs,cjs,ts,jsx,tsx}', '!**/.next/*'] },
	{ languageOptions: { globals: { ...globals.browser, ...globals.node } } },
	pluginJs.configs.recommended,
	...tseslint.configs.recommended,
	pluginReact.configs.flat.recommended,
	// ...compat.extends('next'),
	{
		rules: {
			'react/react-in-jsx-scope': 'off',
		},
	},
	{
		ignores: ['**/.next/', '**/dist/'],
	},
);
// [

// ];
