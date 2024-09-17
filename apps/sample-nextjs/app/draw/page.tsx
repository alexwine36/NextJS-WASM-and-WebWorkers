import dynamic from 'next/dynamic';

const DrawingApp = dynamic(
	() => import('../../src/components/DrawingApp').then((mod) => mod.DrawingApp),
	{
		ssr: false,
	},
);

export default async function Draw() {
	return (
		// <div className={styles.layout}>
		<>
			<h1>Draw</h1>

			<DrawingApp />
		</>
		// </div>
	);
}
