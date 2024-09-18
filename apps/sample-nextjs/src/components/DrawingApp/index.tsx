'use client';
import { useRef } from 'react';
import { useDrawingApp } from '../../hooks/use-drawing-app';
import { Toolbar } from './Toolbar';

export const DrawingApp = () => {
	const canvasRef = useRef<HTMLCanvasElement>(null);

	const { tools, colors, penSizes, undo, redo, canUndo, canRedo } = useDrawingApp(canvasRef);

	return (
		<>
			<Toolbar
				canRedo={canRedo}
				canUndo={canUndo}
				colors={colors}
				penSizes={penSizes}
				redo={redo}
				tools={tools}
				undo={undo}
			/>
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
