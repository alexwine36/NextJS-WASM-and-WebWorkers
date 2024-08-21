import styles from './index.module.css';

type ComparisonCardProps = {
	title: string;
	description: string;
	children: React.ReactNode;
	danger?: boolean;
};

export const ComparisonCard = ({ title, description, children, danger }: ComparisonCardProps) => {
	return (
		<div
			className={styles.card}
			style={{
				borderColor: danger ? 'red' : 'gray',
			}}
		>
			<h2>{title}</h2>
			<p>{description}</p>
			{children}
		</div>
	);
};
