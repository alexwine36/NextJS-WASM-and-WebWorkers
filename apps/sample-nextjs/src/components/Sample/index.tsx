'use client';

import { GameOfLife } from '../GameOfLife';

export const SampleThing = () => {
	const loadWasm = async () => {
		const wasmModule = await import('../../../../../crates/sample-wasm/pkg/sample_wasm');
		return wasmModule;
	};

	const handleClick = () => {
		const runWasm = async () => {
			const wasmModule = await loadWasm();

			const res = wasmModule.fibonacci(42);
			console.log(res);
			console.log(wasmModule);
			wasmModule.greet();
		};
		runWasm();
	};
	return (
		<div>
			<GameOfLife />
			<button onClick={handleClick}>Press Me</button>
		</div>
	);
};
