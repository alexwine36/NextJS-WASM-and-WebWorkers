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
			<canvas
				style={{
					width: '75vw',
					height: '50vh',
					border: '1px solid black',
				}}
				ref={canvasRef}
			/>
		</div>
	);
};
