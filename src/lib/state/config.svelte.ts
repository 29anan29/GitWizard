import { git, type UpdateInfo } from '$lib/services/git';
import type { AppConfig } from '$lib/services/git';

const defaults: AppConfig = {
	locale: 'zh-CN',
	userName: null,
	userEmail: null,
	autoPush: true,
	recentRepos: [],
	commitPrefixes: ['feat', 'fix', 'docs', 'chore', 'refactor', 'test'],
	updateProxy: null,
	autoCheckUpdate: true,
	credentialUsername: null
};

export const config = $state<AppConfig>({ ...defaults });

let ready = false;

export async function initConfig(): Promise<void> {
	if (ready) return;
	try {
		const loaded = await git.getConfig();
		Object.assign(config, loaded);
	} catch {
		Object.assign(config, defaults);
	}
	ready = true;
}

export async function persistConfig(): Promise<void> {
	try {
		await git.saveConfig({
			...config,
			recentRepos: [...config.recentRepos],
			commitPrefixes: [...config.commitPrefixes]
		});
	} catch {
	}
}
