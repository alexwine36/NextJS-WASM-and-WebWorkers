'use client';

import { Provider } from 'jotai';

export const Providers = ({ children }: { children: React.ReactNode }) => {
	return <Provider>{children}</Provider>;
};
