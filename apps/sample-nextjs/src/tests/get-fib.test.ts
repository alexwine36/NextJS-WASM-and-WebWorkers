// import { test } from 'vitest';
import { getFibonacci } from '../utils/get-fib';
describe('first', () => {
	test('should first', async () => {
		const res = await getFibonacci(12);
		expect(res).toBe(144);
	});
});
