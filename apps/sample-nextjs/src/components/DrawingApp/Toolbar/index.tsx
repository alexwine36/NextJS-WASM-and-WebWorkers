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

	const activeTool = tools.find((tool) => tool.active);
	const activeColor = colors.find((color) => color.active);
	const activePenSize = penSizes.find((penSize) => penSize.active);
	return (
		<div className={styles.toolbar}>
			<div className={styles.dropdown}>
				<button className={styles.dropbtn} type="button">
					Tools <b>({activeTool?.name})</b>
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
					<b
						style={{
							backgroundColor: activeColor?.color,
							width: '1em',
							height: '1em',
							display: 'inline-block',
						}}
					/>
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
					Pen Sizes <b>({activePenSize?.size})</b>
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
