export class LocalUrl {
	urlString: string;
	url: URL;
	path?: string;
	queries: URLSearchParams = new URLSearchParams();

	constructor(path?: string) {
		this.urlString = 'http://localhost:3001';
		const env_url = process.env.NEXT_PUBLIC_VERCEL_URL;
		if (env_url) {
			this.urlString = env_url;
		}
		this.url = new URL(this.urlString);
		this.path = path;
	}

	addQuery(key: string, value: string) {
		this.queries.append(key, value);
		return this;
	}

	toString() {
		if (this.path) {
			this.url.pathname = this.path;
		}
		this.url.search = this.queries.toString();
		return this.url.toString();
	}
}
