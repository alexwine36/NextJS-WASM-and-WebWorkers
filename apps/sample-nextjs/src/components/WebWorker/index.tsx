'use client';

import { useSample } from 'web-worker';
import { timeFunction } from '../../utils/time-func';

export const WebWorker = () => {
	const worker = useSample();

	const handleClick = () => {
		// timeFunction(async () => {
		// 	const res = await worker.runFibonacci(42);
		// 	console.log(res);
		//     return res

		// });
		timeFunction(worker.runFibonacci, 42).then((r) => {
			console.log(r);
		});
		// console.log(res);

		// const runWasm = async () => {
		// 	const wasmModule = await loadWasm();

		// 	const res = wasmModule.fibonacci(42);
		// 	console.log(res);
		// 	console.log(wasmModule);
		// 	wasmModule.greet();
		// };
		// runWasm();
	};
	return (
		<div>
			<button onClick={handleClick}>Press Me</button>
		</div>
	);
};
