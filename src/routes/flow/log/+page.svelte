<script lang="ts">
	import { fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { goto } from '$app/navigation';
	import WizardNav from '$lib/components/WizardNav.svelte';
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import loader from '$lib/assets/icons/loader-2.svg?raw';
	import { repoStore, openDialog } from '$lib/state/repo.svelte';
	import { git } from '$lib/services/git';
	import { t } from '$lib/i18n/index.svelte';

	interface CommitEntry {
		hash: string;
		message: string;
		author: string;
		time: string;
		parents: string[];
	}

	let commits = $state<CommitEntry[]>([]);
	let busy = $state(false);
	let loadError = $state('');
	let selectedHash = $state('');
	let expanded = $state(false);

	const info = $derived(repoStore.info);
	const selected = $derived(commits.find((c) => c.hash === selectedHash));

	async function loadLog(): Promise<void> {
		if (!info) return;
		busy = true;
		loadError = '';
		try {
			commits = await git.getLog(info.path, 50);
		} catch (e) {
			loadError = typeof e === 'string' ? e : String(e);
		} finally {
			busy = false;
		}
	}

	function toggleDetail(hash: string): void {
		if (selectedHash === hash) {
			expanded = !expanded;
		} else {
			selectedHash = hash;
			expanded = true;
		}
	}

	function graphLine(index: number, total: number): string {
		const isLast = index === total - 1;
		if (isLast) return '●';
		return '│';
	}

	async function browse(): Promise<void> {
		try { await openDialog(); } catch { /* noop */ }
	}

	loadLog();
</script>

<div class="container page">
	<WizardNav title={t('log.title')} steps={[t('log.title')]} current={1} {busy} oncancel={() => goto('/')} />

	{#if !info}
		<section class="guard">
			<h2>{t('guard.noRepo.title')}</h2>
			<p>{t('guard.noRepo.desc')}</p>
			<div class="actions">
				<Button variant="accent" onclick={browse}>{t('repo.openAction')}</Button>
				<Button variant="ghost" onclick={() => goto('/')}>{t('run.backHome')}</Button>
			</div>
		</section>
	{:else}
		<section class="body" in:fly={{ y: 14, duration: 240, easing: cubicOut }}>
			{#if busy}
				<section class="center"><Icon svg={loader} size={28} /></section>
			{:else if loadError}
				<p class="errline">{loadError}</p>
			{:else}
				<div class="loglist">
					{#each commits as commit, i (commit.hash)}
						<div class="commit-row">
							<div class="graph">
								<span class="graph-symbol">{graphLine(i, commits.length)}</span>
							</div>
							<button
								class="commit-btn"
								class:selected={selectedHash === commit.hash}
								onclick={() => toggleDetail(commit.hash)}
							>
								<div class="commit-main">
									<span class="hash">{commit.hash.slice(0, 7)}</span>
									<span class="msg">{commit.message}</span>
								</div>
								<div class="commit-meta">
									<span class="author">{commit.author}</span>
									<span class="time">{commit.time}</span>
								</div>
							</button>
						</div>

						{#if selectedHash === commit.hash && expanded}
							<div class="detail" in:fly={{ y: 6, duration: 160, easing: cubicOut }}>
								<div class="detail-row">
									<span class="k">{t('log.detail.hash')}</span>
									<span class="v mono">{commit.hash}</span>
								</div>
								<div class="detail-row">
									<span class="k">{t('log.detail.parents')}</span>
									<span class="v mono">{commit.parents.length > 0 ? commit.parents.join(', ') : t('log.detail.root')}</span>
								</div>
								<div class="detail-row">
									<span class="k">{t('log.detail.author')}</span>
									<span class="v">{commit.author}</span>
								</div>
								<div class="detail-row">
									<span class="k">{t('log.detail.time')}</span>
									<span class="v">{commit.time}</span>
								</div>
							</div>
						{/if}
					{/each}

					{#if commits.length === 0}
						<p class="empty">{t('log.empty')}</p>
					{/if}
				</div>

				<footer class="foot">
					<Button variant="ghost" onclick={() => goto('/')}>{t('run.backHome')}</Button>
				</footer>
			{/if}
		</section>
	{/if}
</div>

<style>
	.page { padding-bottom: 80px; }
	.guard { display: flex; flex-direction: column; align-items: center; gap: 10px; text-align: center; padding: 90px 24px; }
	.guard h2 { margin: 0; font-size: 26px; font-weight: 400; }
	.guard p { margin: 0 0 10px; font-family: var(--font-serif); color: var(--text-secondary); }
	.actions { display: flex; gap: 10px; }

	.center { display: flex; flex-direction: column; align-items: center; gap: 14px; padding: 90px 24px; color: var(--text-secondary); }
	.center :global(.icon) { animation: spin 0.9s linear infinite; color: var(--color-accent); }
	@keyframes spin { to { transform: rotate(360deg); } }

	.body { max-width: 720px; margin: 28px auto 0; padding: 0 8px; }

	.loglist { display: flex; flex-direction: column; gap: 2px; }

	.commit-row { display: flex; align-items: stretch; gap: 0; }

	.graph { display: flex; flex-direction: column; align-items: center; width: 28px; flex-shrink: 0; }
	.graph-symbol { font-family: var(--font-mono); font-size: 13px; color: var(--color-accent); line-height: 1; padding-top: 12px; }

	.commit-btn { flex: 1; display: flex; flex-direction: column; gap: 3px; background: var(--surface-100); border: 1px solid transparent; border-radius: var(--radius-comfortable); padding: 8px 14px; cursor: pointer; text-align: left; transition: all 140ms ease; }
	.commit-btn:hover { background: var(--surface-300); }
	.commit-btn.selected { background: rgba(245, 78, 0, 0.04); border-color: rgba(245, 78, 0, 0.2); }

	.commit-main { display: flex; align-items: center; gap: 8px; }
	.hash { font-family: var(--font-mono); font-size: 12px; color: var(--color-accent); flex-shrink: 0; }
	.msg { font-size: 13px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }

	.commit-meta { display: flex; gap: 10px; font-size: 11px; color: var(--text-tertiary); }
	.author { }
	.time { }

	.detail { margin: 0 0 8px 28px; padding: 10px 16px; background: var(--surface-300); border-radius: var(--radius-comfortable); display: flex; flex-direction: column; gap: 6px; }
	.detail-row { display: flex; align-items: baseline; gap: 10px; font-size: 12.5px; }
	.k { color: var(--text-secondary); flex-shrink: 0; min-width: 70px; }
	.v { font-family: var(--font-mono); font-size: 12px; word-break: break-all; }
	.mono { font-family: var(--font-mono); }

	.empty { text-align: center; padding: 40px 0; color: var(--text-secondary); font-family: var(--font-serif); font-size: 14px; }
	.errline { margin: 0 0 12px; font-size: 12.5px; color: var(--color-error); white-space: pre-wrap; }
	.foot { display: flex; justify-content: center; margin-top: 24px; }
</style>
