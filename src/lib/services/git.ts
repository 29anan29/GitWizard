import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

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
		username: string | null,
		password: string | null
	) =>
		invoke<void>('push_remote', {
			repoPath,
			remoteName,
			branch,
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
	openExternal: (url: string) => invoke<void>('open_external', { url })
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
