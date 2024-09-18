import { RefObject, useCallback, useEffect, useMemo, useState } from 'react';
import { IconType } from 'react-icons';
import { BsPaintBucket } from 'react-icons/bs';
import { FaPen, FaRegCircle, FaRegSquare } from 'react-icons/fa';
import { IoAnalyticsOutline } from 'react-icons/io5';
import { App as AppRef, ToolType } from 'wasm-draw';

const TOOLS: Record<ToolType, { name: string; value: ToolType; icon: IconType }> = {
	[ToolType.Pen]: { name: 'Pen', value: ToolType.Pen, icon: FaPen },
	[ToolType.Line]: { name: 'Line', value: ToolType.Line, icon: IoAnalyticsOutline },
	[ToolType.Rectangle]: { name: 'Rectangle', value: ToolType.Rectangle, icon: FaRegSquare },
	[ToolType.Circle]: { name: 'Circle', value: ToolType.Circle, icon: FaRegCircle },
	[ToolType.Fill]: { name: 'Fill', value: ToolType.Fill, icon: BsPaintBucket },
};

export const useDrawingApp = (canvasRef: RefObject<HTMLCanvasElement>) => {
	const [app, setApp] = useState<AppRef | undefined>();
	const [activeTool, setActiveTool] = useState<ToolType>(ToolType.Line);
	const [colors, setColors] = useState<string[]>([]);
	const [activeColor, setActiveColor] = useState<string>('black');
	const [penSizes, setPenSizes] = useState<number[]>([]);
	const [activePenSize, setActivePenSize] = useState<number>(1);
	const [doState, setDoState] = useState<{
		undo: boolean;
		redo: boolean;
	}>({ undo: false, redo: false });

	const loadWasm = async () => {
		const wasmModule = await import('wasm-draw');
		return wasmModule;
	};

	const measurementCallback = () => {
		// console.log(state);
		// checkDoState();
		if (!app) {
			console.log('no app');
			return;
		}
		const can_undo = app.can_undo() || false;
		const can_redo = app.can_redo() || false;
		console.log({ can_undo, can_redo });
		setDoState({ undo: can_undo, redo: can_redo });
		app.draw();
	};

	const runDrawingApp = async (canvas: HTMLCanvasElement) => {
		const { App } = await loadWasm();
		// init_app(canvas);
		const app = new App(canvas);
		setApp(app);

		// app.set_measurement_callback(measurementCallback);
	};

	useEffect(() => {
		if (!app) {
			console.log('no app');
			return;
		}
		app.set_measurement_callback(measurementCallback);
		setActiveTool(app.get_active_tool());

		// console.log(app.get_colors());
		setColors(app.get_colors());

		setActiveColor(app.get_active_color());
		setPenSizes(Array.from(app.get_pen_sizes()));
		setActivePenSize(app.get_pen_sizes()[0]);

		app.run();
	}, [app]);

	const checkDoState = useCallback(() => {
		if (!app) {
			console.log('no app');
			return;
		}
		const can_undo = app.can_undo() || false;
		const can_redo = app.can_redo() || false;
		console.log({ can_undo, can_redo });
		setDoState({ undo: can_undo, redo: can_redo });
		app.draw();
	}, [app]);

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

	const undo = useCallback(() => {
		app?.undo_measurement();
		checkDoState();
	}, [app]);
	const redo = useCallback(() => {
		app?.redo_measurement();
		checkDoState();
	}, [app]);

	const tools = useMemo(() => {
		return Object.entries(TOOLS).map(([_, { name, value, icon }]) => ({
			name,
			onClick: () => setTool(value),
			active: activeTool === value,
			icon,
		}));
	}, [activeTool, TOOLS]);

	const colorList = useMemo(() => {
		return colors.map((color) => ({
			color,
			onClick: () => setColor(color),
			active: activeColor === color,
		}));
	}, [colors, activeColor]);

	const penSizesList = useMemo(() => {
		return penSizes.map((size) => ({
			size,
			onClick: () => {
				app?.set_active_pen_size(size);
				setActivePenSize(size);
			},
			active: activePenSize === size,
		}));
	}, [penSizes, activePenSize]);

	return {
		app,
		tools,
		setTool,
		colors: colorList,
		penSizes: penSizesList,
		undo,
		redo,
		canUndo: doState.undo,
		canRedo: doState.redo,
	};
};

export type UseDrawingAppReturn = ReturnType<typeof useDrawingApp>;
