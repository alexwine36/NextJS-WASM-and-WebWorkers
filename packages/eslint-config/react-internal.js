/*
 * This is a custom ESLint configuration for use with
 * internal (bundled by their consumer) libraries
 * that utilize React.
 *
 * This config extends the Vercel Engineering Style Guide.
 * For more information, see https://github.com/vercel/style-guide
 *
 */

/** @type {import("eslint").Linter.Config} */
module.exports = {
	extends: ['./base.js', require.resolve('@vercel/style-guide/eslint/react')],

	rules: {
		'react/jsx-no-leaked-render': ['warn', { validStrategies: ['coerce', 'ternary'] }],
		'react/self-closing-comp': 'error',
		'react/jsx-no-useless-fragment': 'error',

		// Accessibility rule overrides
		'jsx-a11y/no-autofocus': 'off',
		'jsx-a11y/no-static-element-interactions': 'off',
		'jsx-a11y/click-events-have-key-events': 'off',
		'jsx-a11y/iframe-has-title': 'off',

		'react/react-in-jsx-scope': 'off',
		'react/function-component-definition': ['error', { namedComponents: 'arrow-function' }],
	},
};
