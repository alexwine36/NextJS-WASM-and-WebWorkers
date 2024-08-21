import { access, symlink } from 'fs/promises';
import { join } from 'path';

/** @type {import('next').NextConfig} */
const nextConfig = {
	headers: async () => {
		/**
		 * Protect against clickjacking everywhere.
		 * This will need to be adjusted if we want to embed a part of our app as an iframe
		 *
		 * @link https://nextjs.org/docs/pages/api-reference/next-config-js/headers#content-security-policy
		 * @link https://github.com/vercel/next.js/blob/canary/examples/with-strict-csp/middleware.js
		 */
		const ContentSecurityPolicy = `frame-ancestors 'self';`;

		// const CORS_HEADERS = [
		//   'Upload-Offset',
		//   'X-Requested-With',
		//   'Tus-Version',
		//   'Tus-Resumable',
		//   'Tus-Extension',
		//   'Tus-Max-Size',
		//   'X-HTTP-Method-Override',
		//   'X-CSRF-Token',
		//   'Accept',
		//   'Accept-Version',
		//   'Authorization',
		//   'Content-Length',
		//   'Content-MD5',
		//   'Content-Type',
		//   'Date',
		//   'X-Api-Version',
		//   'upload-length',
		//   'upload-metadata',
		// ];

		// const CORS_METHODS = ['GET', 'DELETE', 'POST', 'HEAD', 'PATCH', 'OPTIONS'];

		return [
			{
				source: '/:path*',
				// source: '/((?!api|_next/static|_next/image|favicon.ico).*)',
				headers: [
					{
						key: 'X-Frame-Options',
						value: 'SAMEORIGIN',
					},
					{
						key: 'Content-Security-Policy',
						value: ContentSecurityPolicy.replace(/\s{2,}/g, ' ').trim(),
					},
				],
			},
			// {
			//   source: '/api/:path*',
			//   headers: [
			//     { key: 'Access-Control-Allow-Credentials', value: 'true' },
			//     { key: 'Access-Control-Allow-Origin', value: '*' }, // replace this your actual origin
			//     {
			//       key: 'Access-Control-Allow-Methods',
			//       value: CORS_METHODS.join(', '),
			//     },
			//     {
			//       key: 'Access-Control-Allow-Headers',
			//       value: CORS_HEADERS.join(', '),
			//     },
			//   ],
			// },
		];
	},
	webpack(config, { isServer, dev }) {
		// Use the client static directory in the server bundle and prod mode
		// Fixes `Error occurred prerendering page "/"`
		// config.output.webassemblyModuleFilename =
		// 	isServer && !dev ? '../static/pkg/[modulehash].wasm' : 'static/pkg/[modulehash].wasm';
		config.experiments = { ...config.experiments, asyncWebAssembly: true };
		if (!dev) {
			config.plugins.push(
				new (class {
					apply(compiler) {
						compiler.hooks.afterEmit.tapPromise('SymlinkWebpackPlugin', async (compiler) => {
							if (isServer) {
								const from = join(compiler.options.output.path, '../static');
								const to = join(compiler.options.output.path, 'static');

								try {
									await access(from);
									console.log(`${from} already exists`);
									return;
								} catch (error) {
									if (error.code === 'ENOENT') {
										// No link exists
									} else {
										throw error;
									}
								}

								await symlink(to, from, 'junction');
								console.log(`created symlink ${from} -> ${to}`);
							}
						});
					}
				})(),
			);
			// NOTE: need to handle "Edge as a separate case"
			/*
			type WebpackConfigContext = {
			dev: boolean;
			isServer: boolean;
				nextRuntime?: 'nodejs' | 'edge';
			}
				*/
			// https://github.com/vercel/next.js/issues/25852
			if (isServer) {
				config.output.webassemblyModuleFilename = './../static/wasm/[modulehash].wasm';
			} else {
				config.output.webassemblyModuleFilename = 'static/wasm/[modulehash].wasm';
			}
		}

		return config;
	},
	transpilePackages: ['web-worker'],
};

export default nextConfig;
