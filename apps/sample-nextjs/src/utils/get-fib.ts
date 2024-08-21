// import 'server-only';

export const getFibonacci = async (data: number) => {
	const wasm = await import('sample-wasm');

	return wasm.fibonacci(data);
};
