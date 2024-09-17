'use client';
import { useRef } from 'react';
import { useDrawingApp } from '../../hooks/use-drawing-app';
import { Toolbar } from './Toolbar';

export const DrawingApp = () => {
	const canvasRef = useRef<HTMLCanvasElement>(null);

	const { tools, colors, penSizes } = useDrawingApp(canvasRef);

	return (
		<>
			<Toolbar colors={colors} penSizes={penSizes} tools={tools} />
			<div
				style={{
					width: '100%',
					// maxHeight: '100%',
					height: '100%',
					border: '1px solid black',
					resize: 'both',
					overflow: 'hidden',
				}}
			>
				<canvas
					ref={canvasRef}
					style={{
						width: '100%',
						height: 'max-content',
					}}
				/>
			</div>
		</>
	);
};
