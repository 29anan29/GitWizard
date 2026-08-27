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
	import { repoStore, openDialog, refresh as refreshRepo } from '$lib/state/repo.svelte';
	import { git } from '$lib/services/git';
	import type { BranchList } from '$lib/services/git';
	import { friendlyErrorKey } from '$lib/flows/commit/errors';
	import { t } from '$lib/i18n/index.svelte';

	type Phase = 'checking' | 'ready' | 'running' | 'done' | 'error';

	let phase = $state<Phase>('checking');
	let branches = $state<BranchList>({ local: [], remote: [] });
	let selectedBranch = $state('');
	let errorMsg = $state('');
	let resultStatus = $state<'fast_forward' | 'merged' | 'conflict' | 'up_to_date' | ''>('');
	let conflicts = $state<string[]>([]);

	const info = $derived(repoStore.info);
	const dirty = $derived(repoStore.entries.length > 0);
	const current = $derived(info?.branch ?? '');
	const busy = $derived(phase === 'running');

	const steps = $derived([t('merge.step.check'), t('merge.step.select'), t('merge.step.result')]);
	const navStep = $derived(
		phase === 'checking' ? 1 : phase === 'ready' ? 2 : 3
	);

	const availableBranches = $derived(
		branches.local.filter((b) => b !== current)
	);

	async function precheck(): Promise<void> {
		phase = 'checking';
		errorMsg = '';
		resultStatus = '';
		conflicts = [];
		await refreshRepo();
		if (info) {
			try {
				branches = await git.listBranches(info.path);
			} catch {
				/* noop */
			}
		}
		if (availableBranches.length > 0 && !selectedBranch) {
			selectedBranch = availableBranches[0];
		}
		phase = 'ready';
	}

	async function execute(): Promise<void> {
		if (!info || !selectedBranch) return;
		phase = 'running';
		errorMsg = '';
		try {
			const outcome = await git.merge(info.path, selectedBranch);
			await refreshRepo();
			resultStatus = outcome.status;
			conflicts = outcome.conflicts;
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
		try {
			await openDialog();
		} catch {
			/* noop */
		}
	}

	precheck();
</script>

<div class="container page">
	<WizardNav title={t('merge.title')} {steps} current={navStep} {busy} oncancel={cancel} />

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
			<div class="checks">
				<div class="checkrow-item">
					<span class="k">{t('merge.currentBranch')}</span>
					<span class="v mono">{current}</span>
				</div>
				<div class="checkrow-item" class:bad={dirty}>
					<span class="k">{t('merge.dirtyStatus')}</span>
					{#if dirty}
						<span class="v err">{t('merge.dirtyWarn')}</span>
					{:else}
						<span class="v ok">✓</span>
					{/if}
				</div>
			</div>

			{#if availableBranches.length === 0}
				<div class="empty-state">
					<p>{t('merge.noBranches')}</p>
				</div>
			{:else}
				<div class="selectcard">
					<span class="label">{t('merge.selectLabel')}</span>
					<div class="selectrow">
						<select bind:value={selectedBranch} disabled={busy}>
							{#each availableBranches as b (b)}
								<option value={b}>{b}</option>
							{/each}
						</select>
						<Button variant="accent" disabled={!selectedBranch || busy} onclick={() => void execute()}>
							{t('merge.execute')}
						</Button>
					</div>
					<p class="hint">{t('merge.hint')}</p>
				</div>
			{/if}

			<footer class="bar">
				<Button variant="ghost" onclick={() => goto('/')}>{t('common.cancel')}</Button>
			</footer>
		</section>
	{:else if phase === 'running'}
		<section class="center">
			<Icon svg={loader} size={30} />
			<p>{t('merge.running')}</p>
		</section>
	{:else if phase === 'done'}
		<section class="body narrow" in:fly={{ y: 14, duration: 240, easing: cubicOut }}>
			{#if resultStatus === 'conflict'}
				<div class="panel bad">
					<span class="halo"><Icon svg={alertTriangle} size={40} /></span>
					<h2>{t('merge.conflictTitle')}</h2>
					<p class="desc">{t('merge.conflictDesc')}</p>
					<ul class="files">
						{#each conflicts as f (f)}
							<li>{f}</li>
						{/each}
					</ul>
				</div>
			{:else if resultStatus === 'up_to_date'}
				<div class="panel ok">
					<span class="halo"><Icon svg={circleCheck} size={40} /></span>
					<h2>{t('merge.upToDate')}</h2>
				</div>
			{:else}
				<div class="panel ok">
					<span class="halo"><Icon svg={circleCheck} size={40} /></span>
					<h2>
						{resultStatus === 'fast_forward'
							? t('merge.done.ff')
							: t('merge.done.merged')}
					</h2>
				</div>
			{/if}
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

	.checks { display: flex; flex-direction: column; background: var(--surface-100); box-shadow: rgba(38, 37, 30, 0.1) 0 0 0 1px inset; border-radius: var(--radius-comfortable); overflow: hidden; margin-bottom: 16px; }
	.checkrow-item { display: flex; align-items: center; gap: 10px; padding: 11px 16px; font-size: 13.5px; }
	.checkrow-item + .checkrow-item { border-top: 1px solid var(--border-subtle); }
	.checkrow-item.bad { color: var(--color-error); }
	.k { color: var(--text-secondary); }
	.v { margin-left: auto; }
	.mono { font-family: var(--font-mono); font-size: 12px; }
	.ok { color: var(--color-success); }
	.err { color: var(--color-error); font-size: 12.5px; }

	.selectcard { background: var(--surface-400); box-shadow: rgba(38, 37, 30, 0.1) 0 0 0 1px inset; border-radius: var(--radius-featured); padding: 16px 20px 14px; display: flex; flex-direction: column; gap: 10px; margin-bottom: 16px; }
	.label { font-size: 11px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.048px; color: var(--text-secondary); }
	.selectrow { display: flex; align-items: center; gap: 10px; }
	select { flex: 1; background: var(--surface-100); border: 1px solid var(--border-subtle); border-radius: var(--radius-standard); padding: 8px 11px; font-family: var(--font-mono); font-size: 13px; outline: none; }
	select:focus { border-color: var(--color-accent); }
	.hint { margin: 0; font-size: 11px; color: var(--text-secondary); }
	.empty-state { text-align: center; padding: 24px; color: var(--text-secondary); font-family: var(--font-serif); font-size: 14px; }

	.bar { display: flex; align-items: center; justify-content: space-between; gap: 12px; padding-top: 16px; border-top: 1px solid var(--border-subtle); }
	.center-bar { justify-content: center; margin-top: 22px; }

	.panel { display: flex; flex-direction: column; align-items: center; gap: 12px; text-align: center; padding: 30px 32px; background: var(--surface-100); box-shadow: rgba(38, 37, 30, 0.1) 0 0 0 1px inset, var(--shadow-card); border-radius: 14px; }
	.halo { width: 68px; height: 68px; border-radius: 50%; display: flex; align-items: center; justify-content: center; animation: pop-in 0.4s cubic-bezier(0.22, 1, 0.36, 1) both; }
	.panel.ok .halo { background: rgba(31, 138, 101, 0.1); }
	.panel.ok :global(.icon) { color: var(--color-success); }
	.panel.bad .halo { background: rgba(207, 45, 86, 0.09); }
	.panel.bad :global(.icon) { color: var(--color-error); }
	h2 { margin: 4px 0 0; font-size: 24px; font-weight: 400; letter-spacing: -0.325px; }
	.desc { margin: 0; font-family: var(--font-serif); font-size: 14.5px; color: var(--text-secondary); }
	.files { list-style: none; margin: 4px 0 0; padding: 10px 16px; background: var(--surface-300); border-radius: var(--radius-comfortable); font-family: var(--font-mono); font-size: 12px; line-height: 1.8; color: var(--color-error); text-align: left; max-width: 100%; word-break: break-all; user-select: text; }
	.friendly { margin: 0; font-family: var(--font-serif); font-size: 14.5px; line-height: 1.65; text-align: left; background: var(--surface-300); border-radius: var(--radius-comfortable); padding: 12px 16px; max-width: 100%; white-space: pre-wrap; }
	.rawwrap { width: 100%; text-align: left; }
	.rawwrap summary { font-size: 11.5px; color: var(--text-tertiary); cursor: pointer; padding: 2px 0 6px; }
	.errbox { margin: 4px 0 0; padding: 12px 16px; background: var(--surface-300); border-radius: var(--radius-comfortable); font-family: var(--font-mono); font-size: 12px; line-height: 1.67; color: var(--color-error); white-space: pre-wrap; word-break: break-all; user-select: text; }
</style>
