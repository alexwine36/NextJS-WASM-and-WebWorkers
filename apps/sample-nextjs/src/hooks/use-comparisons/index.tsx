'use client';

import { useAtom, useAtomValue } from 'jotai';
import { splitAtom } from 'jotai/utils';
import { useEffect } from 'react';
import { fibonacci, useSample } from 'web-worker';
import { runTask } from '../../components/RunnableComparisonCard';
import { comparisionsAtom, inputAtom, microtaskQueueAtom } from '../../state/atoms';
export type ComparisonOption = {
	title: string;
	description: string;

	run: (n: number) => Promise<number>;
};

export type ComparisonReturn = Omit<ComparisonOption, 'run'> & {
	run: () => Promise<{
		duration: number;
		res: Promise<number>;
	}>;
};

export const useComparisons = () => {
	const [comparisons, setComparisons] = useAtom(comparisionsAtom);
	const useMicrotaskQueue = useAtomValue(microtaskQueueAtom);
	const optionsAtomsAtom = splitAtom(comparisionsAtom);
	const input = useAtomValue(inputAtom);
	const worker = useSample();
	const loadWasm = async () => {
		const wasmModule = await import('sample-wasm');
		return wasmModule;
	};

	useEffect(() => {
		setComparisons([
			{
				title: 'Web Worker with WASM',
				description: 'This runs the code in a web worker with function code compiled to WASM',
				run: worker.runFibonacci,
			},
			{
				title: 'Web Worker JS',
				description: 'This runs the code in a web worker',
				run: worker.runJsFibonacci,
			},

			{
				title: 'Node WASM',
				description: 'This is an API call using the WASM module',
				run: async (n: number) => {
					const res = await fetch(`http://localhost:3001/api?query=${n}`).then((res) => res.json());
					console.log(res);
					return res.result;
				},
			},
			{
				title: 'Node JS',
				description: 'This is an API call using the JS module',
				run: async (n: number) => {
					const res = await fetch(`http://localhost:3001/api/js?query=${n}`).then((res) =>
						res.json(),
					);
					console.log(res);
					return res.result;
				},
			},
			{
				title: 'WASM',
				mainThread: true,
				description: 'This runs the code in WASM',
				run: async (n: number) => {
					const wasmModule = await loadWasm();
					return wasmModule.fibonacci(n);
				},
			},
			{
				title: 'JS',
				mainThread: true,
				description: 'This runs the code in JS',
				run: async (n: number) => {
					return fibonacci(n);
				},
			},
		]);
	}, []);

	const handleRunAll = () => {
		comparisons.forEach((comp, idx) => {
			if (comp.mainThread) {
				return;
			}
			const setComparison = (data: ComparisonOption) => {
				setComparisons((prev) => {
					const newComparisons = [...prev];
					newComparisons[idx] = data;
					return newComparisons;
				});
			};
			runTask(comp, setComparison, input, useMicrotaskQueue);
		});
	};

	return {
		handleRunAll,
		optionsAtomsAtom,
		// options: comparisons,
	};
};
