'use client';

import { useSample } from 'web-worker';
import { GameOfLife } from '../GameOfLife';
export const SampleThing = () => {
	const worker = useSample();
	// const loadWasm = async () => {
	// 	const wasmModule = await import('../../../../../crates/sample-wasm/pkg/sample_wasm');
	// 	return wasmModule;
	// };

	const handleClick = () => {
		worker.runReturnNumber(42).then((res) => {
			console.log(res);
		});
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
			<GameOfLife />
			<button onClick={handleClick}>Press Me</button>
		</div>
	);
};
