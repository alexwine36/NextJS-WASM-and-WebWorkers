'use client';

import { useAtom } from 'jotai';
import { useComparisons } from '../../hooks/use-comparisons';
import { Button } from '../Button';
import { RunnableComparisonCard } from '../RunnableComparisonCard';
import styles from './index.module.css';
export const OptionGrid = () => {
	const { optionsAtomsAtom, handleRunAll } = useComparisons();

	const [optionsAtoms] = useAtom(optionsAtomsAtom);
	return (
		<div>
			<Button onClick={handleRunAll}>Run All</Button>
			<div className={styles.grid}>
				{optionsAtoms.map((option, index) => (
					<RunnableComparisonCard atom={option} key={index} />
				))}
			</div>
		</div>
	);
};
