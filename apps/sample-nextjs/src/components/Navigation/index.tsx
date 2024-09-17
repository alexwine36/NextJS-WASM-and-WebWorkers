import Link from 'next/link';
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
						<Link href={page.href}>{page.name}</Link>
					</li>
				))}
			</ul>
		</nav>
	);
};
