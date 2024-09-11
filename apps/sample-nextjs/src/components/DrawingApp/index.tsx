'use client';
import { useEffect, useRef } from 'react';

export const DrawingApp = () => {
	const canvasRef = useRef<HTMLCanvasElement>(null);
	const loadWasm = async () => {
		const wasmModule = await import('wasm-draw');
		return wasmModule;
	};

	const runDrawingApp = async (canvas: HTMLCanvasElement) => {
		const { init_app } = await loadWasm();
		init_app(canvas);
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
			}}
			ref={canvasRef}
		/>
	);
};
