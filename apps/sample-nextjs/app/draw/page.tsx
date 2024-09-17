import dynamic from 'next/dynamic';
import styles from './page.module.css';

const DrawingApp = dynamic(
	() => import('../../src/components/DrawingApp').then((mod) => mod.DrawingApp),
	{
		ssr: false,
	},
);

export const Draw = () => {
	return (
		<div className={styles.layout}>
			<div className={styles.wrapper}>
				<h1>Draw</h1>

				<DrawingApp />
			</div>
		</div>
	);
};

export default Draw;
