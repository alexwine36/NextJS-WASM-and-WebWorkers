import { atom } from 'jotai';
import { ComparisonOption } from '../hooks/use-comparisons';

export const inputAtom = atom<number>(40);

export type ComparisonOptionAtom = ComparisonOption & {
	duration?: number;
	result?: number;
	running?: boolean;
	mainThread?: boolean;
};

export const comparisionsAtom = atom<ComparisonOptionAtom[]>([]);

export const microtaskQueueAtom = atom(true);
