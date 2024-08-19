import { type NextRequest } from 'next/server';
import * as wasm from 'sample-wasm';

export const runtime = 'edge';

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export async function GET(_request: NextRequest) {
	console.log('RUNTIME', process.env.NEXT_RUNTIME);
	// const wasm = await import('sample-wasm');
	const result = wasm.fibonacci(14);
	console.log(result);
	return Response.json({
		message: 'Hello from the API',
		result,
	});
}
