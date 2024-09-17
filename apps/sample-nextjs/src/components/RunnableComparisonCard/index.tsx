import { PrimitiveAtom, useAtom, useAtomValue } from 'jotai';
import { ComparisonOptionAtom, inputAtom, microtaskQueueAtom } from '../../state/atoms';
import { formatNumber } from '../../utils/format-number';
import { timeFunction } from '../../utils/time-func';
import { Button } from '../Button';
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
		<ComparisonCard danger={mainThread} description={description} title={title}>
			<Button
				disabled={running}
				onClick={async () => {
					await handleRun();
				}}
			>
				Run
				{/* Run {mainThread ? 'Main Thread' : 'Web Worker'} */}
			</Button>
			<div>
				<p>Duration: {formatNumber(duration)}ms</p>
				<p>Result: {formatNumber(result)}</p>
			</div>
		</ComparisonCard>
	);
};
