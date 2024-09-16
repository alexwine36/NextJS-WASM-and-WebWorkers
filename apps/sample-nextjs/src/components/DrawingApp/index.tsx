'use client';
import { useRef } from 'react';
import { useDrawingApp } from '../../hooks/use-drawing-app';

export const DrawingApp = () => {
	const canvasRef = useRef<HTMLCanvasElement>(null);

	const { tools, colors } = useDrawingApp(canvasRef);

	return (
		<div>
			<div>
				{tools.map(({ name, onClick, active }) => (
					<button key={name} onClick={onClick} style={{ fontWeight: active ? 'bold' : 'normal' }}>
						{name}
					</button>
				))}
				{colors.map((color) => (
					<button
						key={color.color}
						onClick={() => color.onClick()}
						style={{ backgroundColor: color.color, color: color.color }}
					>
						{color.color}
					</button>
				))}
			</div>
			<div
				style={{
					width: '75vw',
					height: '50vh',
					border: '1px solid black',
					resize: 'both',
					overflow: 'hidden',
				}}
			>
				<canvas
					style={{
						width: '100%',
						height: '100%',
					}}
					ref={canvasRef}
				/>
			</div>
		</div>
	);
};
