'use client';
import { useEffect, useRef } from 'react';
import { ToolType } from 'wasm-draw';

const TOOLS: Record<ToolType, { name: string }> = {
	[ToolType.Line]: { name: 'Line' },
	[ToolType.Rectangle]: { name: 'Rectangle' },
	// [ToolType.Circle]: { name: 'Circle' },
	[ToolType.Pen]: { name: 'Pen' },
	// [ToolType.Eraser]: { name: 'Eraser' },
	// [ToolType.Fill]: { name: 'Fill' },
};

export const DrawingApp = () => {
	const canvasRef = useRef<HTMLCanvasElement>(null);
	const loadWasm = async () => {
		const wasmModule = await import('wasm-draw');
		return wasmModule;
	};

	const runDrawingApp = async (canvas: HTMLCanvasElement) => {
		const { App, ToolType } = await loadWasm();
		// init_app(canvas);
		const app = new App(canvas);
		app.run();
		// console.log(app.get_state().get_tool());
		console.log(app.get_active_tool());
		console.log(app.get_state());
		app.set_active_tool(ToolType.Line);
		console.log(app.get_active_tool());
	};
	useEffect(() => {
		const canvas = canvasRef.current;
		if (!canvas) return;
		runDrawingApp(canvas);
		const ctx = canvas.getContext('2d');
		if (!ctx) return;

		ctx.fillStyle = 'white';
		// ctx.fillRect(0, 0, canvas.width, canvas.height);
	}, [canvasRef]);

	return (
		<canvas
			style={{
				width: '50vh',
				height: '50vh',
				border: '1px solid black',
			}}
			ref={canvasRef}
		/>
	);
};
