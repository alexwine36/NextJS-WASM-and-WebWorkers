import { LocalUrl } from '../utils/get-url';

describe('first', () => {
	test('should first', () => {
		const url = new LocalUrl('/api/route');

		expect(url.toString()).toBe('http://localhost:3001/api/route');

		url.addQuery('query', '12');

		expect(url.toString()).toBe('http://localhost:3001/api/route?query=12');
	});
});
