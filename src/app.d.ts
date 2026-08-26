declare global {
	namespace App {}

	interface Window {}
}

declare module '*.svg?raw' {
	const content: string;
	export default content;
}

export {};
