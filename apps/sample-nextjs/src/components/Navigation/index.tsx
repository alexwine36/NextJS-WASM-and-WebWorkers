import styles from './index.module.css';

const pages = [
	{ name: 'Home', href: '/' },
	{ name: 'Draw', href: '/draw' },
];

export const Navigation = () => {
	return (
		<nav className={styles.navigation}>
			<ul>
				{pages.map((page) => (
					<li key={page.href}>
						<a href={page.href}>{page.name}</a>
					</li>
				))}
			</ul>
		</nav>
	);
};
