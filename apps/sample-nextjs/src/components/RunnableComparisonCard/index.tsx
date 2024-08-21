import { PrimitiveAtom, useAtom, useAtomValue } from 'jotai';
import { ComparisonOptionAtom, inputAtom, microtaskQueueAtom } from '../../state/atoms';
import { timeFunction } from '../../utils/time-func';
import { ComparisonCard } from '../ComparisonCard';

type RunnableComparisonCardProps = {
	atom: PrimitiveAtom<ComparisonOptionAtom>;
};

export const runTask = async (
	data: ComparisonOptionAtom,
	setData: (data: ComparisonOptionAtom) => void,
	input: number,
	queue = false,
) => {
	const { running, run } = data;
	if (running) return;
	setData({ ...data, running: true });
	const runFunc = async () => {
		const { duration, res } = await timeFunction(run, input);
		const result = await res;
		setData({ ...data, duration, result, running: false });
	};
	if (queue) {
		queueMicrotask(async () => {
			await runFunc();
		});
	} else {
		await runFunc();
	}
};

export const RunnableComparisonCard = ({ atom }: RunnableComparisonCardProps) => {
	// const [duration, setDuration] = useState<number | undefined>(undefined);
	// const [result, setResult] = useState<number | undefined>(undefined);
	const [data, setResult] = useAtom(atom);
	const useMicrotaskQueue = useAtomValue(microtaskQueueAtom);
	const { title, description, duration, result, running, mainThread } = data;
	const input = useAtomValue(inputAtom);

	const handleRun = async () => {
		runTask(data, setResult, input, useMicrotaskQueue);
		// setResult((prev) => ({ ...prev, running: true }));
		// const { duration, res } = await timeFunction(run, input);

		// const result = await res;
		// setResult((prev) => ({ ...prev, duration, result, running: false }));
		// return {
		// 	duration,
		// 	result,
		// };
	};

	return (
		<ComparisonCard title={title} description={description} danger={mainThread}>
			<button
				disabled={running}
				onClick={async () => {
					await handleRun();
				}}
			>
				Run {mainThread ? 'Main Thread' : 'Web Worker'}
			</button>
			<div>
				<p>Duration: {duration}ms</p>
				<p>Result: {result}</p>
			</div>
		</ComparisonCard>
	);
};
