// import { SampleThing } from '../src/components/Sample';
import dynamic from 'next/dynamic';
import { NumberInput } from '../src/components/NumberInput';
import { Providers } from '../src/components/providers';

const OptionGrid = dynamic(
	() => import('../src/components/OptionGrid').then((mod) => mod.OptionGrid),
	{
		ssr: false,
	},
);

export default async function Home() {
	// const data = await getFibonacci(12);
	// const serverData = await fetch('http://localhost:3001/api').then((res) => res.json());

	return (
		<Providers>
			<NumberInput />
			<OptionGrid />
		</Providers>
	);
}
