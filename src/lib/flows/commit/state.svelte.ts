import { SvelteSet } from 'svelte/reactivity';
import { git } from '$lib/services/git';
import { repoStore, refresh } from '$lib/state/repo.svelte';
import { config } from '$lib/state/config.svelte';
import { resolveAuth } from '$lib/state/creds.svelte';
import { requestAuth } from '$lib/state/authPrompt.svelte';
import { t } from '$lib/i18n/index.svelte';

export type Phase = 'idle' | 'running' | 'success' | 'error';
export type ErrorKind = 'none' | 'nonff' | 'generic';

function classifyError(raw: string): { kind: ErrorKind; msg: string } {
	if (raw === 'CRED_MISSING') {
		return { kind: 'generic', msg: t('err.noCred') };
	}
	if (raw === 'DETACHED_HEAD') {
		return { kind: 'generic', msg: t('push.needBranch') };
	}
	if (raw.startsWith('NON_FF:') || /non-fast-forward|fetch first|rejected/i.test(raw)) {
		return { kind: 'nonff', msg: raw.replace(/^NON_FF:/, '') };
	}
	return { kind: 'generic', msg: raw };
}

function errorText(e: unknown): string {
	return typeof e === 'string' ? e : e instanceof Error ? e.message : String(e);
}

async function ensureAuth() {
	let auth = await resolveAuth();
	if (auth) return auth;
	const ok = await requestAuth();
	if (!ok) return null;
	return resolveAuth();
}

export function createWizard() {
	const selected = new SvelteSet<string>();

	const state = $state({
		step: 1,
		summary: { files: 0, insertions: 0, deletions: 0 },
		message: '',
		prefix: '',
		autoPush: config.autoPush,
		pushTarget: '',
		phase: 'idle' as Phase,
		oid: '',
		pushed: false,
		errorKind: 'none' as ErrorKind,
		errorMsg: ''
	});

	let timer: ReturnType<typeof setTimeout> | undefined;

	function toggle(path: string): void {
		if (selected.has(path)) selected.delete(path);
		else selected.add(path);
		scheduleSummary();
	}

	function setGroup(paths: string[], on: boolean): void {
		for (const p of paths) {
			if (on) selected.add(p);
			else selected.delete(p);
		}
		scheduleSummary();
	}

	function scheduleSummary(): void {
		if (timer) clearTimeout(timer);
		timer = setTimeout(() => void computeSummary(), 250);
	}

	async function computeSummary(): Promise<void> {
		const info = repoStore.info;
		if (!info) return;
		const files = [...selected];
		if (files.length === 0) {
			state.summary = { files: 0, insertions: 0, deletions: 0 };
			return;
		}
		try {
			state.summary = await git.summary(info.path, files);
		} catch {
			state.summary = { files: files.length, insertions: 0, deletions: 0 };
		}
	}

	async function execute(): Promise<void> {
		const info = repoStore.info;
		if (!info) return;

		state.phase = 'running';
		state.errorKind = 'none';
		state.errorMsg = '';

		try {
			const preStaged = repoStore.entries
				.filter((e) => e.staged && !selected.has(e.path))
				.map((e) => e.path);
			if (preStaged.length > 0) {
				await git.unstage(info.path, preStaged);
			}

			await git.stage(info.path, [...selected]);
			state.oid = await git.commit(info.path, state.message.trim(), config.userName, config.userEmail);
			state.pushed = false;

			if (state.autoPush) {
				if (!info.branch) throw new Error('DETACHED_HEAD');
				const auth = await ensureAuth();
				if (!auth) throw new Error('CRED_MISSING');
				await git.push(
					info.path,
					'origin',
					info.branch,
					state.pushTarget || null,
					auth.username,
					auth.password
				);
				state.pushed = true;
			}
			state.phase = 'success';
			await refresh();
		} catch (e) {
			if (state.oid) state.pushed = false;
			const c = classifyError(errorText(e));
			state.errorKind = c.kind;
			state.errorMsg = c.msg;
			state.phase = 'error';
			await refresh();
		}
	}

	async function retry(): Promise<void> {
		const info = repoStore.info;
		if (!info) return;

		if (state.oid && !state.pushed) {
			state.phase = 'running';
			try {
				if (!info.branch) throw new Error('DETACHED_HEAD');
				const auth = await ensureAuth();
				if (!auth) throw new Error('CRED_MISSING');
				await git.push(
					info.path,
					'origin',
					info.branch,
					state.pushTarget || null,
					auth.username,
					auth.password
				);
				state.pushed = true;
				state.phase = 'success';
				await refresh();
			} catch (e) {
				const c = classifyError(errorText(e));
				state.errorKind = c.kind;
				state.errorMsg = c.msg;
				state.phase = 'error';
			}
			return;
		}
		await execute();
	}

	async function pushOnly(): Promise<void> {
		const info = repoStore.info;
		if (!info) return;

		state.oid = '';
		state.pushed = false;
		state.errorKind = 'none';
		state.errorMsg = '';
		state.phase = 'running';
		try {
			if (!info.branch) throw new Error('DETACHED_HEAD');
			const auth = await ensureAuth();
			if (!auth) throw new Error('CRED_MISSING');
			await git.push(
				info.path,
				'origin',
				info.branch,
				state.pushTarget || null,
				auth.username,
				auth.password
			);
			state.pushed = true;
			state.phase = 'success';
			await refresh();
		} catch (e) {
			const c = classifyError(errorText(e));
			state.errorKind = c.kind;
			state.errorMsg = c.msg;
			state.phase = 'error';
			await refresh();
		}
	}

	function reset(): void {
		selected.clear();
		state.step = 1;
		state.summary = { files: 0, insertions: 0, deletions: 0 };
		state.message = '';
		state.prefix = '';
		state.autoPush = config.autoPush;
		state.pushTarget = '';
		state.phase = 'idle';
		state.oid = '';
		state.pushed = false;
		state.errorKind = 'none';
		state.errorMsg = '';
	}

	return {
		selected,
		toggle,
		setGroup,
		computeSummary,
		execute,
		retry,
		pushOnly,
		reset,
		state
	};
}

export type Wizard = ReturnType<typeof createWizard>;
