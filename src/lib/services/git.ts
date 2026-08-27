import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import {
	check as updaterCheck,
	type Update as UpdaterUpdate
} from '@tauri-apps/plugin-updater';

export interface RepoInfo {
	path: string;
	name: string;
	branch: string | null;
	ahead: number;
	behind: number;
	dirtyCount: number;
	remoteUrl: string | null;
}

export type FileKind = 'added' | 'modified' | 'deleted';

export interface FileEntry {
	path: string;
	worktree: FileKind | null;
	staged: FileKind | null;
	conflicted: boolean;
}

export interface StagedSummary {
	files: number;
	insertions: number;
	deletions: number;
}

export interface AppConfig {
	locale: 'zh-CN' | 'en';
	userName: string | null;
	userEmail: string | null;
	autoPush: boolean;
	recentRepos: string[];
	commitPrefixes: string[];
	updateProxy: string | null;
	autoCheckUpdate: boolean;
	credentialUsername: string | null;
	theme: 'system' | 'light' | 'dark';
}

export interface BranchList {
	local: string[];
	remote: string[];
}

export interface UpdateInfo {
	current: string;
	latestTag: string | null;
	releaseUrl: string;
	available: boolean;
}

export type PullStatus = 'up_to_date' | 'fast_forward' | 'merged' | 'conflict';

export interface PullOutcome {
	status: PullStatus;
	conflicts: string[];
	fetchedOid: string | null;
}

export const git = {
	openDialog: (title: string) => invoke<string | null>('open_repo_dialog', { title }),
	validate: (repoPath: string) => invoke<RepoInfo>('validate_repo', { repoPath }),
	status: (repoPath: string) => invoke<FileEntry[]>('get_status', { repoPath }),
	stage: (repoPath: string, files: string[]) => invoke<void>('stage_files', { repoPath, files }),
	unstage: (repoPath: string, files: string[]) =>
		invoke<void>('unstage_files', { repoPath, files }),
	summary: (repoPath: string, files: string[]) =>
		invoke<StagedSummary>('staged_summary', { repoPath, files }),
	commit: (repoPath: string, message: string, user: string | null, email: string | null) =>
		invoke<string>('commit_repo', {
			repoPath,
			message,
			identityUser: user,
			identityEmail: email
		}),
	push: (
		repoPath: string,
		remoteName: string,
		branch: string,
		targetBranch: string | null,
		username: string | null,
		password: string | null
	) =>
		invoke<void>('push_remote', {
			repoPath,
			remoteName,
			branch,
			targetBranch,
			username,
			password
		}),
	pull: (
		repoPath: string,
		remoteName: string,
		branch: string,
		username: string | null,
		password: string | null
	) =>
		invoke<PullOutcome>('pull_branch', {
			repoPath,
			remoteName,
			branch,
			username,
			password
		}),
	getConfig: () => invoke<AppConfig>('get_config'),
	saveConfig: (config: AppConfig) => invoke<void>('save_config', { config }),
	checkUpdates: (proxy: string | null) => invoke<UpdateInfo>('check_updates', { proxy }),
	openExternal: (url: string) => invoke<void>('open_external', { url }),
	saveCredential: (username: string, password: string) =>
		invoke<void>('save_credential', { username, password }),
	loadCredential: (username: string) => invoke<string | null>('load_credential', { username }),
	deleteCredential: (username: string) => invoke<void>('delete_credential', { username }),
	listBranches: (repoPath: string) => invoke<BranchList>('list_branches', { repoPath }),
	createBranch: (repoPath: string, name: string, switchTo: boolean) =>
		invoke<void>('create_branch', { repoPath, name, switch: switchTo }),
	checkoutBranch: (repoPath: string, name: string) =>
		invoke<void>('checkout_branch', { repoPath, name }),
	deleteBranch: (repoPath: string, name: string, force: boolean) =>
		invoke<void>('delete_branch', { repoPath, name, force }),
	renameBranch: (repoPath: string, oldName: string, newName: string) =>
		invoke<void>('rename_branch', { repoPath, oldName, newName }),
	initRepo: (path: string) => invoke<RepoInfo>('init_repo', { path }),
	getGitignore: (repoPath: string) => invoke<string>('get_gitignore', { repoPath }),
	setGitignore: (repoPath: string, content: string) =>
		invoke<void>('set_gitignore', { repoPath, content }),

	checkUpdater: () => updaterCheck(),
	downloadAndInstall: (update: UpdaterUpdate) => update.downloadAndInstall((p) => {}),

	merge: (repoPath: string, branch: string) =>
		invoke<{ status: 'fast_forward' | 'merged' | 'conflict' | 'up_to_date'; conflicts: string[] }>(
			'merge_branch',
			{ repoPath, branch }
		),
	reset: (repoPath: string, mode: 'soft' | 'mixed' | 'hard', target: string) =>
		invoke<void>('git_reset', { repoPath, mode, target }),
	revert: (repoPath: string, commitHash: string) =>
		invoke<void>('git_revert', { repoPath, commitHash }),
	getLog: (repoPath: string, maxCount: number) =>
		invoke<{ hash: string; message: string; author: string; time: string; parents: string[] }[]>(
			'get_log',
			{ repoPath, maxCount }
		),
	readFile: (repoPath: string, filePath: string) =>
		invoke<string>('read_repo_file', { repoPath, filePath }),
	writeFile: (repoPath: string, filePath: string, content: string) =>
		invoke<void>('write_repo_file', { repoPath, filePath, content }),
	listSshKeys: () =>
		invoke<{ name: string; type: string; path: string; isDefault: boolean }[]>('list_ssh_keys'),
	generateSshKey: (name: string, keyType: string, comment?: string) =>
		invoke<string>('generate_ssh_key', { name, keyType, comment })
};

export interface LogLine {
	kind: 'cmd' | 'out' | 'err';
	line: string;
}

export interface PushProgress {
	received: number;
	total: number;
	percent: number;
}

type Unlisten = () => void;

export function onLog(cb: (line: LogLine) => void): Promise<Unlisten> {
	return listen<LogLine>('git-log', (e) => cb(e.payload));
}

export function onPushProgress(cb: (p: PushProgress) => void): Promise<Unlisten> {
	return listen<PushProgress>('push-progress', (e) => cb(e.payload));
}
