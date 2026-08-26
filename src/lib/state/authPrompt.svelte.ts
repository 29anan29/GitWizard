export const authPromptStore = $state({
	open: false
});

let resolver: ((ok: boolean) => void) | null = null;

export function requestAuth(): Promise<boolean> {
	authPromptStore.open = true;
	return new Promise((resolve) => {
		resolver = resolve;
	});
}

export function settleAuth(ok: boolean): void {
	authPromptStore.open = false;
	resolver?.(ok);
	resolver = null;
}
