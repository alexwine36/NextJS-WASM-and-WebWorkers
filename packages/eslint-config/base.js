const { resolve } = require('node:path');

const project = resolve(process.cwd(), 'tsconfig.json');

/** @type {import("eslint").Linter.Config} */
module.exports = {
	extends: ['eslint:recommended', 'prettier', 'prettier'],
	plugins: ['@typescript-eslint', 'unused-imports'],
	globals: {
		React: true,
		JSX: true,
	},
	env: {
		node: true,
		browser: true,
		jest: true,
	},
	settings: {
		'import/resolver': {
			typescript: {
				project,
			},
		},
	},
	ignorePatterns: [
		// Ignore dotfiles
		'.*.js',
		'node_modules/',
		'dist/',
	],
	overrides: [
		{
			files: ['*.js?(x)', '*.ts?(x)'],
		},
		{
			files: ['*.ts', '*.tsx'],
			rules: {
				'no-undef': 'off',
			},
		},
	],
	rules: {
		'no-redeclare': 'off',
		'no-case-declarations': 'warn',
		'no-useless-escape': 'error',
		'no-prototype-builtins': 'warn',
		'no-unsafe-finally': 'warn',
		'no-unused-vars': 'off',
		'@typescript-eslint/no-unused-vars': 'off',
		'unused-imports/no-unused-imports': 'error',
		'unused-imports/no-unused-vars': [
			'error',
			{
				vars: 'all',
				varsIgnorePattern: '^_',
				args: 'after-used',
				argsIgnorePattern: '^_',
			},
		],

		//React rules
		'react/jsx-uses-react': 'off',
		'react/react-in-jsx-scope': 'off',
		'@typescript-eslint/no-unused-vars': [
			'error',
			{
				args: 'all',
				argsIgnorePattern: '^_',
				caughtErrors: 'all',
				caughtErrorsIgnorePattern: '^_',
				destructuredArrayIgnorePattern: '^_',
				varsIgnorePattern: '^_',
				ignoreRestSiblings: true,
			},
		],

		//Turbo rules
		// 'turbo/no-undeclared-env-vars': 'off',
	},
};
