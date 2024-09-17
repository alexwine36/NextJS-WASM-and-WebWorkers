/* eslint-disable jsx-a11y/no-noninteractive-element-interactions */
import { UseDrawingAppReturn } from '../../../hooks/use-drawing-app';
import styles from './index.module.css';

type ToolbarProps = {
	tools: UseDrawingAppReturn['tools'];
	colors: UseDrawingAppReturn['colors'];
	penSizes: UseDrawingAppReturn['penSizes'];
};

export const Toolbar = (props: ToolbarProps) => {
	const { tools, colors, penSizes } = props;

	return (
		<div className={styles.toolbar}>
			<div className={styles.dropdown}>
				<button className={styles.dropbtn} type="button">
					Tools
				</button>
				<menu className={styles.dropdownContent}>
					{tools.map(({ name, onClick, active }) => (
						<li className={active ? styles.active : ''} key={name} onClick={onClick}>
							{name}
						</li>
					))}
				</menu>
			</div>
			<div className={styles.dropdown}>
				<button className={styles.dropbtn} type="button">
					Colors
				</button>
				<menu className={styles.dropdownContent}>
					{colors.map((color) => (
						<li
							className={color.active ? styles.active : ''}
							key={color.color}
							onClick={() => color.onClick()}
							style={{
								// backgroundColor: color.color,
								color: color.color,
							}}
						>
							{color.color}
						</li>
					))}
				</menu>
			</div>
			<div className={styles.dropdown}>
				<button className={styles.dropbtn} type="button">
					Pen Sizes
				</button>
				<menu className={styles.dropdownContent}>
					{penSizes.map(({ size, onClick, active }) => (
						<li className={active ? styles.active : ''} key={size} onClick={onClick}>
							{size}
						</li>
					))}
				</menu>
			</div>
		</div>
	);
};
