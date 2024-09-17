'use client';
import { useRef } from 'react';
import { useDrawingApp } from '../../hooks/use-drawing-app';
import { Toolbar } from './Toolbar';

export const DrawingApp = () => {
	const canvasRef = useRef<HTMLCanvasElement>(null);

	const { tools, colors, penSizes } = useDrawingApp(canvasRef);

	return (
		<>
			<Toolbar tools={tools} colors={colors} penSizes={penSizes} />
			<div
				style={{
					width: '100%',
					height: '100%',
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
		</>
	);
};
