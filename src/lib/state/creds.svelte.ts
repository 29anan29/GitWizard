import { git } from '$lib/services/git';
import { config } from './config.svelte';

export const credStore = $state({
	sessionToken: '',
	keyringUnavailable: false
});

export async function resolveAuth(): Promise<{ username: string | null; password: string } | null> {
	const user = config.credentialUsername ?? config.userName;
	let token = credStore.sessionToken;

	if (!token && config.credentialUsername) {
		try {
			const saved = await git.loadCredential(config.credentialUsername);
			if (saved) token = saved;
		} catch {
			credStore.keyringUnavailable = true;
		}
	}

	if (!token) return null;
	return { username: user, password: token };
}
