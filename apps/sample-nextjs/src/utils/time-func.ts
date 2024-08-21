/* eslint-disable @typescript-eslint/no-explicit-any */
// eslint-disable-next-line @typescript-eslint/no-unused-vars
type Parameter<T> = T extends (arg: infer T) => any ? T : never;

export const timeFunction = async <F extends (arg: any) => any>(
	func: F,
	arg: Parameter<F>,
): Promise<{
	duration: number;
	res: ReturnType<F>;
}> => {
	const start = performance.now();
	const res: ReturnType<F> = await func(arg);
	const end = performance.now();
	console.log(`Time taken: ${end - start}ms`);
	return {
		res,
		duration: end - start,
	};
};
