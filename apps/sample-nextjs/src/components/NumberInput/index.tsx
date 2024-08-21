'use client';

import { useAtom } from 'jotai';
import { inputAtom } from '../../state/atoms';
import styles from './index.module.css';

export const NumberInput = () => {
	const [value, setValue] = useAtom(inputAtom);

	const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		setValue(parseInt(e.target.value, 10));
	};
	return (
		<div className={styles.panel}>
			<label htmlFor="value">Value:</label>
			<input type="number" id="value" value={value} onChange={handleChange} />
		</div>
	);
};
