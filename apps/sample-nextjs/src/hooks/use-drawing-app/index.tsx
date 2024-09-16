import { RefObject, useEffect, useMemo, useState } from 'react';
import { App as AppRef, ToolType } from 'wasm-draw';

const TOOLS: Record<ToolType, { name: string; value: ToolType }> = {
	[ToolType.Line]: { name: 'Line', value: ToolType.Line },
	[ToolType.Rectangle]: { name: 'Rectangle', value: ToolType.Rectangle },
	[ToolType.Pen]: { name: 'Pen', value: ToolType.Pen },
	[ToolType.Fill]: { name: 'Fill', value: ToolType.Fill },
};

export const useDrawingApp = (canvasRef: RefObject<HTMLCanvasElement>) => {
	const [app, setApp] = useState<AppRef | undefined>();
	const [activeTool, setActiveTool] = useState<ToolType>(ToolType.Line);
	const [colors, setColors] = useState<string[]>([]);
	const [activeColor, setActiveColor] = useState<string>('black');
	const loadWasm = async () => {
		const wasmModule = await import('wasm-draw');
		return wasmModule;
	};

	const runDrawingApp = async (canvas: HTMLCanvasElement) => {
		const { App } = await loadWasm();
		// init_app(canvas);
		const app = new App(canvas);
		setActiveTool(app.get_active_tool());
		setApp(app);
		// console.log(app.get_colors());
		setColors(app.get_colors());
		setActiveColor(app.get_active_color());
		app.run();
	};

	useEffect(() => {
		const canvas = canvasRef.current;
		if (!canvas) return;
		runDrawingApp(canvas);
		const ctx = canvas.getContext('2d');
		if (!ctx) return;

		ctx.fillStyle = 'white';
	}, [canvasRef]);

	const setTool = (tool: ToolType) => {
		app?.set_active_tool(tool);
		setActiveTool(tool);
	};
	const setColor = (color: string) => {
		app?.set_active_color(color);
		setActiveColor(color);
	};

	const tools = useMemo(() => {
		return Object.entries(TOOLS).map(([tool, { name, value }]) => ({
			name,
			onClick: () => setTool(value),
			active: activeTool === value,
		}));
	}, [activeTool, TOOLS]);

	const colorList = useMemo(() => {
		return colors.map((color) => ({
			color,
			onClick: () => setColor(color),
			active: activeColor === color,
		}));
	}, [colors, activeColor]);

	return {
		app,
		tools,
		setTool,
		colors: colorList,
	};
};
