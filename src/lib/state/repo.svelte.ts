import { git, type FileEntry, type RepoInfo } from '$lib/services/git';
import { config, persistConfig } from './config.svelte';

export const repoStore = $state({
	info: null as RepoInfo | null,
	entries: [] as FileEntry[],
	loading: false
});

export async function openDialog(): Promise<void> {
	const path = await git.openDialog('GitWizard');
	if (path) await openPath(path);
}

export async function openPath(path: string): Promise<void> {
	repoStore.loading = true;
	try {
		const info = await git.validate(path);
		repoStore.info = info;
		await refresh();
		addRecent(path);
	} finally {
		repoStore.loading = false;
	}
}

export async function initRepo(): Promise<void> {
	const path = await git.openDialog('选择要初始化仓库的目录');
	if (!path) return;
	repoStore.loading = true;
	try {
		const info = await git.initRepo(path);
		repoStore.info = info;
		repoStore.entries = [];
		addRecent(path);
	} finally {
		repoStore.loading = false;
	}
}

export async function refresh(): Promise<void> {
	if (!repoStore.info) return;
	const path = repoStore.info.path;
	try {
		const [info, entries] = await Promise.all([git.validate(path), git.status(path)]);
		repoStore.info = info;
		repoStore.entries = entries;
	} catch {
		repoStore.info = null;
		repoStore.entries = [];
	}
}

function addRecent(path: string): void {
	config.recentRepos = [path, ...config.recentRepos.filter((p) => p !== path)].slice(0, 8);
	persistConfig();
}
