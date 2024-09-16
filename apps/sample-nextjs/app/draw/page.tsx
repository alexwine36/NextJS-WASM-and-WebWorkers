import dynamic from 'next/dynamic';
import styles from './page.module.css';

const DrawingApp = dynamic(
	() => import('../../src/components/DrawingApp').then((mod) => mod.DrawingApp),
	{
		ssr: false,
	},
);

export default async function Draw() {
	return (
		<div className={styles.layout}>
			<h1>Draw</h1>
			<p>This is the Draw page</p>
			<DrawingApp />
		</div>
	);
}
