/* eslint-disable jsx-a11y/no-noninteractive-element-interactions */
import { FaRedo, FaUndo } from 'react-icons/fa';
import { UseDrawingAppReturn } from '../../../hooks/use-drawing-app';
import { Button, ButtonGroup, IconButton } from '../../Button';
import styles from './index.module.css';

type ToolbarProps = {
	tools: UseDrawingAppReturn['tools'];
	colors: UseDrawingAppReturn['colors'];
	penSizes: UseDrawingAppReturn['penSizes'];
	undo: UseDrawingAppReturn['undo'];
	redo: UseDrawingAppReturn['redo'];
	canUndo: UseDrawingAppReturn['canUndo'];
	canRedo: UseDrawingAppReturn['canRedo'];
};

export const Toolbar = (props: ToolbarProps) => {
	const { tools, colors, penSizes, undo, redo, canRedo, canUndo } = props;

	const activeColor = colors.find((color) => color.active);
	const activePenSize = penSizes.find((penSize) => penSize.active);
	return (
		<div className={styles.toolbar}>
			<ButtonGroup
				style={{
					fontSize: 24,
				}}
			>
				{tools.map(({ name, onClick, active, icon }) => (
					<IconButton
						Icon={icon}
						iconProps={{ color: active ? 'black' : 'gray' }}
						isActive={active}
						key={name}
						onClick={onClick}
					>
						{name}
					</IconButton>
				))}
			</ButtonGroup>

			<div className={styles.dropdown}>
				<Button className={styles.dropbtn} type="button">
					Colors
					<b
						style={{
							backgroundColor: activeColor?.color,
							width: '1em',
							height: '1em',
							display: 'inline-block',
						}}
					/>
				</Button>
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
				<Button className={styles.dropbtn} type="button">
					Pen Sizes <b>({activePenSize?.size})</b>
				</Button>
				<menu className={styles.dropdownContent}>
					{penSizes.map(({ size, onClick, active }) => (
						<li className={active ? styles.active : ''} key={size} onClick={onClick}>
							{size}
						</li>
					))}
				</menu>
			</div>
			<ButtonGroup
				style={{
					fontSize: 20,
					// color: 'gray',
				}}
			>
				<IconButton
					Icon={FaUndo}
					disabled={!canUndo}
					// iconProps={{ color: 'gray' }}
					onClick={() => {
						undo();
					}}
				/>
				<IconButton
					Icon={FaRedo}
					disabled={!canRedo}
					// iconProps={{ color: 'gray' }}
					onClick={() => {
						redo();
					}}
				/>
			</ButtonGroup>
		</div>
	);
};
