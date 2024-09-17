import { ButtonHTMLAttributes } from 'react';
import styles from './index.module.css';

export const Button = (props: ButtonHTMLAttributes<HTMLButtonElement>) => {
	console.log(styles);
	return <button className={styles.button} type="button" {...props} />;
};
