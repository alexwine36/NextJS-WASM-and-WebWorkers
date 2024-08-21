import { type NextRequest } from 'next/server';
import * as wasm from 'sample-wasm';
import { timeFunction } from '../../src/utils/time-func';

// export const runtime = 'edge';

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export async function GET(request: NextRequest) {
	const searchParams = request.nextUrl.searchParams;
	const query = searchParams.get('query');
	let fibNum = 14;
	if (query) {
		fibNum = parseInt(query);
	}
	console.log('RUNTIME', process.env.NEXT_RUNTIME);
	// const wasm = await import('sample-wasm');
	const { res, duration } = await timeFunction(wasm.fibonacci, fibNum);
	// const result = wasm.fibonacci(fibNum);
	console.log(res);
	return Response.json({
		result: res,
		duration,
	});
}
