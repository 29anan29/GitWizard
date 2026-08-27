<script lang="ts">
	import { fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { goto } from '$app/navigation';
	import WizardNav from '$lib/components/WizardNav.svelte';
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import circleCheck from '$lib/assets/icons/circle-check.svg?raw';
	import alertTriangle from '$lib/assets/icons/alert-triangle.svg?raw';
	import loader from '$lib/assets/icons/loader-2.svg?raw';
	import history from '$lib/assets/icons/history.svg?raw';
	import { repoStore, openDialog, refresh as refreshRepo } from '$lib/state/repo.svelte';
	import { git } from '$lib/services/git';
	import { friendlyErrorKey } from '$lib/flows/commit/errors';
	import { t, type Key } from '$lib/i18n/index.svelte';

	type Phase = 'checking' | 'ready' | 'running' | 'done' | 'error';
	type Tab = 'reset' | 'revert';

	let phase = $state<Phase>('checking');
	let tab = $state<Tab>('reset');
	let mode = $state<'soft' | 'mixed' | 'hard'>('mixed');
	let target = $state('HEAD~1');
	let errorMsg = $state('');
	let doneMsg = $state('');
	let logEntries = $state<{ hash: string; message: string; author: string; time: string }[]>([]);
	let selectedRevertHash = $state('');
	let confirmArmed = $state(false);

	const info = $derived(repoStore.info);
	const dirty = $derived(repoStore.entries.length > 0);
	const current = $derived(info?.branch ?? '');
	const busy = $derived(phase === 'running');

	const steps = $derived([t('reset.step.check'), t('reset.step.configure'), t('reset.step.result')]);
	const navStep = $derived(
		phase === 'checking' ? 1 : phase === 'ready' ? 2 : 3
	);

	async function precheck(): Promise<void> {
		phase = 'checking';
		errorMsg = '';
		doneMsg = '';
		confirmArmed = false;
		await refreshRepo();
		if (info) {
			try {
				const log = await git.getLog(info.path, 30);
				logEntries = log;
			} catch {
				logEntries = [];
			}
		}
		phase = 'ready';
	}

	async function executeReset(): Promise<void> {
		if (!info) return;
		phase = 'running';
		errorMsg = '';
		try {
			await git.reset(info.path, mode, target);
			await refreshRepo();
			doneMsg = t(('reset.done.' + mode) as Key);
			phase = 'done';
		} catch (e) {
			errorMsg = typeof e === 'string' ? e : String((e as Error)?.message ?? e);
			phase = 'error';
		}
	}

	async function executeRevert(): Promise<void> {
		if (!info || !selectedRevertHash) return;
		phase = 'running';
		errorMsg = '';
		try {
			await git.revert(info.path, selectedRevertHash);
			await refreshRepo();
			doneMsg = t('reset.done.revert');
			phase = 'done';
		} catch (e) {
			errorMsg = typeof e === 'string' ? e : String((e as Error)?.message ?? e);
			phase = 'error';
		}
	}

	function cancel(): void {
		if (busy) return;
		goto('/');
	}

	async function browse(): Promise<void> {
		try { await openDialog(); } catch { /* noop */ }
	}

	precheck();
</script>

<div class="container page">
	<WizardNav title={t('reset.title')} {steps} current={navStep} {busy} oncancel={cancel} />

	{#if !info}
		<section class="guard">
			<h2>{t('guard.noRepo.title')}</h2>
			<p>{t('guard.noRepo.desc')}</p>
			<div class="actions">
				<Button variant="accent" onclick={browse}>{t('repo.openAction')}</Button>
				<Button variant="ghost" onclick={() => goto('/')}>{t('run.backHome')}</Button>
			</div>
		</section>
	{:else if phase === 'checking'}
		<section class="center"><Icon svg={loader} size={28} /></section>
	{:else if phase === 'ready'}
		<section class="body narrow" in:fly={{ y: 14, duration: 240, easing: cubicOut }}>
			<div class="tabs">
				<button class="tab" class:active={tab === 'reset'} onclick={() => (tab = 'reset')}>
					<Icon svg={history} size={14} />
					{t('reset.tab.reset')}
				</button>
				<button class="tab" class:active={tab === 'revert'} onclick={() => (tab = 'revert')}>
					<Icon svg={circleCheck} size={14} />
					{t('reset.tab.revert')}
				</button>
			</div>

			{#if tab === 'reset'}
				<div class="card">
					<span class="label">{t('reset.mode.label')}</span>
					<div class="modes">
						{#each (['soft', 'mixed', 'hard'] as const) as m}
							<button
								class="mode-btn"
								class:active={mode === m}
								class:danger={mode === m && m === 'hard'}
								onclick={() => (mode = m)}
							>
								{t(('reset.mode.' + m) as Key)}
							</button>
						{/each}
					</div>
					<p class="modedesc">{t(('reset.mode.desc.' + mode) as Key)}</p>

					<span class="label">{t('reset.target.label')}</span>
					<input type="text" bind:value={target} placeholder="HEAD~1" spellcheck="false" />

					<div class="preview">
						<span class="k">{t('reset.preview.from')}</span>
						<span class="v mono">{current}</span>
						<span class="k">{t('reset.preview.to')}</span>
						<span class="v mono">{target || '—'}</span>
					</div>

					{#if mode === 'hard' && !confirmArmed}
						<Button
							variant="accent"
							disabled={!target.trim() || busy}
							onclick={() => (confirmArmed = true)}
						>
							{t('reset.execute')}
						</Button>
					{:else if mode === 'hard' && confirmArmed}
						<div class="forcepanel">
							<span>{t('reset.confirmHard')}</span>
							<Button variant="accent" onclick={() => void executeReset()} disabled={busy}>
								{t('reset.confirmHardExecute')}
							</Button>
						</div>
					{:else}
						<Button variant="accent" disabled={!target.trim() || busy} onclick={() => void executeReset()}>
							{t('reset.execute')}
						</Button>
					{/if}
				</div>
			{:else}
				<div class="card">
					<span class="label">{t('reset.revert.label')}</span>
					{#if logEntries.length === 0}
						<p class="empty">{t('reset.revert.empty')}</p>
					{:else}
						<ul class="loglist">
							{#each logEntries as entry (entry.hash)}
								<li
									class:selected={selectedRevertHash === entry.hash}
									onclick={() => (selectedRevertHash = entry.hash)}
								>
									<span class="hash">{entry.hash.slice(0, 7)}</span>
									<span class="msg">{entry.message}</span>
									<span class="meta">{entry.author}</span>
								</li>
							{/each}
						</ul>
						<Button
							variant="accent"
							disabled={!selectedRevertHash || busy}
							onclick={() => void executeRevert()}
						>
							{t('reset.revert.execute')}
						</Button>
					{/if}
				</div>
			{/if}

			{#if dirty && tab === 'reset'}
				<p class="warnline">
					<Icon svg={alertTriangle} size={12} />
					{t('reset.dirtyWarn')}
				</p>
			{/if}

			<footer class="bar">
				<Button variant="ghost" onclick={() => goto('/')}>{t('common.cancel')}</Button>
			</footer>
		</section>
	{:else if phase === 'running'}
		<section class="center">
			<Icon svg={loader} size={30} />
			<p>{t('reset.running')}</p>
		</section>
	{:else if phase === 'done'}
		<section class="body narrow" in:fly={{ y: 14, duration: 240, easing: cubicOut }}>
			<div class="panel ok">
				<span class="halo"><Icon svg={circleCheck} size={40} /></span>
				<h2>{doneMsg}</h2>
			</div>
			<footer class="bar center-bar">
				<Button variant="ghost" onclick={() => void precheck()}>{t('repo.refresh')}</Button>
				<Button variant="ghost" onclick={() => goto('/')}>{t('run.backHome')}</Button>
			</footer>
		</section>
	{:else if phase === 'error'}
		<section class="body narrow" in:fly={{ y: 14, duration: 240, easing: cubicOut }}>
			<div class="panel bad">
				<span class="halo"><Icon svg={alertTriangle} size={40} /></span>
				<h2>{t('run.failedTitle')}</h2>
				<p class="friendly">{t(friendlyErrorKey(errorMsg))}</p>
				<details class="rawwrap">
					<summary>{t('err.detail')}</summary>
					<pre class="errbox">{errorMsg}</pre>
				</details>
			</div>
			<footer class="bar center-bar">
				<Button variant="ghost" onclick={() => void precheck()}>{t('common.retry')}</Button>
				<Button variant="ghost" onclick={() => goto('/')}>{t('run.backHome')}</Button>
			</footer>
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

	.body { max-width: 640px; margin: 28px auto 0; padding: 0 8px; }
	.narrow { max-width: 560px; }

	.tabs { display: flex; gap: 6px; margin-bottom: 16px; }
	.tab { display: inline-flex; align-items: center; gap: 6px; background: var(--surface-300); border: none; color: var(--text-tertiary); font-size: 13px; padding: 8px 14px; border-radius: var(--radius-comfortable); cursor: pointer; transition: all 150ms ease; }
	.tab.active { background: var(--surface-400); color: var(--color-text); box-shadow: rgba(38, 37, 30, 0.1) 0 0 0 1px inset; }

	.card { background: var(--surface-400); box-shadow: rgba(38, 37, 30, 0.1) 0 0 0 1px inset; border-radius: var(--radius-featured); padding: 16px 20px 14px; display: flex; flex-direction: column; gap: 10px; margin-bottom: 16px; }
	.label { font-size: 11px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.048px; color: var(--text-secondary); }
	.modes { display: flex; gap: 6px; }
	.mode-btn { flex: 1; background: var(--surface-100); border: 1px solid var(--border-subtle); border-radius: var(--radius-standard); padding: 8px 12px; font-size: 13px; cursor: pointer; transition: all 150ms ease; }
	.mode-btn.active { background: var(--surface-300); border-color: var(--color-text); color: var(--color-text); }
	.mode-btn.danger { border-color: var(--color-error); color: var(--color-error); background: rgba(207, 45, 86, 0.06); }
	.modedesc { margin: 0; font-size: 12px; color: var(--text-secondary); font-family: var(--font-serif); }
	input[type='text'] { background: var(--surface-100); border: 1px solid var(--border-subtle); border-radius: var(--radius-standard); padding: 8px 11px; font-family: var(--font-mono); font-size: 13px; outline: none; transition: border-color 150ms ease; }
	input:focus { border-color: var(--color-accent); }
	.preview { display: grid; grid-template-columns: auto 1fr; gap: 6px 12px; font-size: 12.5px; padding: 8px 12px; background: var(--surface-100); border-radius: var(--radius-comfortable); }
	.k { color: var(--text-secondary); }
	.v { font-family: var(--font-mono); font-size: 12px; }
	.mono { font-family: var(--font-mono); }
	.forcepanel { width: 100%; display: flex; align-items: center; justify-content: space-between; gap: 12px; padding: 10px 14px; background: rgba(207, 45, 86, 0.07); box-shadow: rgba(207, 45, 86, 0.3) 0 0 0 1px inset; border-radius: var(--radius-comfortable); font-size: 12.5px; color: var(--color-error); }

	.loglist { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 4px; max-height: 300px; overflow-y: auto; }
	.loglist li { display: flex; align-items: center; gap: 8px; padding: 8px 12px; background: var(--surface-100); border-radius: var(--radius-comfortable); cursor: pointer; font-size: 12.5px; transition: background 120ms ease; }
	.loglist li:hover { background: var(--surface-300); }
	.loglist li.selected { background: rgba(245, 78, 0, 0.06); box-shadow: rgba(245, 78, 0, 0.25) 0 0 0 1px inset; }
	.hash { font-family: var(--font-mono); font-size: 11px; color: var(--color-accent); flex-shrink: 0; }
	.msg { flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
	.meta { font-size: 11px; color: var(--text-tertiary); flex-shrink: 0; }
	.empty { margin: 0; text-align: center; color: var(--text-secondary); font-family: var(--font-serif); font-size: 14px; padding: 20px 0; }

	.warnline { margin: 0 0 12px; font-size: 12.5px; color: var(--color-gold); display: flex; align-items: center; gap: 6px; }

	.bar { display: flex; align-items: center; justify-content: space-between; gap: 12px; padding-top: 16px; border-top: 1px solid var(--border-subtle); }
	.center-bar { justify-content: center; margin-top: 22px; }

	.panel { display: flex; flex-direction: column; align-items: center; gap: 12px; text-align: center; padding: 30px 32px; background: var(--surface-100); box-shadow: rgba(38, 37, 30, 0.1) 0 0 0 1px inset, var(--shadow-card); border-radius: 14px; }
	.halo { width: 68px; height: 68px; border-radius: 50%; display: flex; align-items: center; justify-content: center; animation: pop-in 0.4s cubic-bezier(0.22, 1, 0.36, 1) both; }
	.panel.ok .halo { background: rgba(31, 138, 101, 0.1); }
	.panel.ok :global(.icon) { color: var(--color-success); }
	.panel.bad .halo { background: rgba(207, 45, 86, 0.09); }
	.panel.bad :global(.icon) { color: var(--color-error); }
	h2 { margin: 4px 0 0; font-size: 24px; font-weight: 400; letter-spacing: -0.325px; }
	.friendly { margin: 0; font-family: var(--font-serif); font-size: 14.5px; line-height: 1.65; text-align: left; background: var(--surface-300); border-radius: var(--radius-comfortable); padding: 12px 16px; max-width: 100%; white-space: pre-wrap; }
	.rawwrap { width: 100%; text-align: left; }
	.rawwrap summary { font-size: 11.5px; color: var(--text-tertiary); cursor: pointer; padding: 2px 0 6px; }
	.errbox { margin: 4px 0 0; padding: 12px 16px; background: var(--surface-300); border-radius: var(--radius-comfortable); font-family: var(--font-mono); font-size: 12px; line-height: 1.67; color: var(--color-error); white-space: pre-wrap; word-break: break-all; user-select: text; }
</style>
