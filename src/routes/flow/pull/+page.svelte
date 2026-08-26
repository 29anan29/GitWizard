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
	import fileText from '$lib/assets/icons/file-text.svg?raw';
	import { createWizard } from '$lib/flows/commit/state.svelte';
	import { friendlyErrorKey } from '$lib/flows/commit/errors';
	import { repoStore, openDialog, refresh as refreshRepo } from '$lib/state/repo.svelte';
	import { config } from '$lib/state/config.svelte';
	import { git } from '$lib/services/git';
	import type { PullOutcome } from '$lib/services/git';
	import { i18n, t } from '$lib/i18n/index.svelte';

	const commitWizard = createWizard();

	type Phase = 'checking' | 'ready' | 'running' | 'done' | 'error';

	let phase = $state<Phase>('checking');
	let outcome = $state<PullOutcome | null>(null);
	let pushedToo = $state(false);
	let errorMsg = $state('');

	const steps = $derived([
		t('pull.step.check'),
		t('pull.step.run'),
		t('pull.step.result')
	]);
	const navStep = $derived(phase === 'checking' ? 1 : phase === 'ready' ? 2 : phase === 'done' || phase === 'error' ? 3 : 2);
	const busy = $derived(phase === 'running');

	const info = $derived(repoStore.info);
	const dirty = $derived(repoStore.entries.length > 0);
	const canPull = $derived(!!info?.branch && !dirty);

	async function precheck(): Promise<void> {
		phase = 'checking';
		outcome = null;
		pushedToo = false;
		errorMsg = '';
		await refreshRepo();
		phase = 'ready';
	}

	async function run(): Promise<void> {
		if (!info?.branch) return;
		phase = 'running';
		errorMsg = '';
		try {
			outcome = await git.pull(
				info.path,
				'origin',
				info.branch,
				config.userName,
				config.userEmail
			);
			await refreshRepo();

			if (outcome.status !== 'conflict' && autoPush && (repoStore.info?.ahead ?? 0) > 0) {
				await git.push(
					repoStore.info!.path,
					'origin',
					repoStore.info!.branch!,
					config.userName,
					config.userEmail
				);
				pushedToo = true;
				await refreshRepo();
			}
			phase = 'done';
		} catch (e) {
			errorMsg = typeof e === 'string' ? e : String((e as Error)?.message ?? e);
			if (errorMsg.startsWith('WORKTREE_DIRTY')) {
				await refreshRepo();
				phase = 'ready';
				return;
			}
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
		}
	}

	let autoPush = $state(config.autoPush && (repoStore.info?.ahead ?? 0) > 0);

	precheck();
</script>

<div class="container page">
	<WizardNav title={t('pull.title')} {steps} current={navStep} {busy} oncancel={cancel} />

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
					<span class="k">{t('pull.branch')}</span>
					<span class="v mono">{info.branch ?? 'HEAD'}</span>
				</div>
				<div class="checkrow-item">
					<span class="k">{t('pull.upstream')}</span>
					<span class="v mono">{info.remoteUrl ? `origin/${info.branch}` : '—'}</span>
				</div>
				<div class="checkrow-item">
					<span class="k">{t('status.ahead', { n: info.ahead })}</span>
					<span class="k">{t('status.behind', { n: info.behind })}</span>
				</div>
				<div class="checkrow-item" class:bad={dirty}>
					<span class="k">{t('pull.clean')}</span>
					{#if dirty}
						<span class="dirtybox">
							<Icon svg={alertTriangle} size={13} />
							{t('pull.dirtyWarn')}
							<button class="golink" onclick={() => goto('/flow/commit')}>
								<Icon svg={fileText} size={12} />
								{t('pull.goCommit')}
							</button>
						</span>
					{:else}
						<span class="v ok">✓</span>
					{/if}
				</div>
			</div>

			<label class="autorow">
				<input type="checkbox" bind:checked={autoPush} />
				<span class="knobbox" aria-hidden="true"></span>
				{t('pull.autoPush')}
			</label>

			<footer class="bar">
				<Button variant="ghost" onclick={() => goto('/')}>{t('common.cancel')}</Button>
				<Button variant="accent" disabled={!canPull} onclick={() => void run()}>
					{t('pull.execute')}
				</Button>
			</footer>
		</section>
	{:else if phase === 'running'}
		<section class="center">
			<Icon svg={loader} size={30} />
			<p>{t('pull.running')}</p>
		</section>
	{:else if phase === 'done' && outcome}
		<section class="body narrow" in:fly={{ y: 14, duration: 240, easing: cubicOut }}>
			{#if outcome.status === 'conflict'}
				<div class="panel bad">
					<span class="halo"><Icon svg={alertTriangle} size={40} /></span>
					<h2>{t('pull.conflictTitle')}</h2>
					<p class="desc">{t('pull.conflictDesc')}</p>
					<ul class="files">
						{#each outcome.conflicts as f (f)}
							<li>{f}</li>
						{/each}
					</ul>
				</div>
			{:else}
				<div class="panel ok">
					<span class="halo"><Icon svg={circleCheck} size={40} /></span>
					<h2>
						{outcome.status === 'up_to_date'
							? t('pull.done.uptodate')
							: outcome.status === 'fast_forward'
								? t('pull.done.ff')
								: t('pull.done.merged')}
					</h2>
					{#if pushedToo}
						<span class="chip ok">{t('pull.pushedToo')}</span>
					{/if}
				</div>
			{/if}
			<footer class="bar center-bar">
				<Button variant="ghost" onclick={() => void precheck()}>{t('repo.refresh')}</Button>
				<Button variant="surface" onclick={() => goto('/')}>{t('run.backHome')}</Button>
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
				<Button variant="ghost" onclick={() => void run()}>{t('common.retry')}</Button>
				<Button variant="surface" onclick={() => goto('/')}>{t('run.backHome')}</Button>
			</footer>
		</section>
	{/if}
</div>

<style>
	.page {
		padding-bottom: 80px;
	}
	.guard {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 10px;
		text-align: center;
		padding: 90px 24px;
	}
	.guard h2 {
		margin: 0;
		font-size: 26px;
		font-weight: 400;
	}
	.guard p {
		margin: 0 0 10px;
		font-family: var(--font-serif);
		color: var(--text-secondary);
	}
	.actions {
		display: flex;
		gap: 10px;
	}

	.center {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 14px;
		padding: 90px 24px;
		color: var(--text-secondary);
	}
	.center :global(.icon) {
		animation: spin 0.9s linear infinite;
		color: var(--color-accent);
	}
	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.body {
		max-width: 640px;
		margin: 28px auto 0;
		padding: 0 8px;
	}
	.narrow {
		max-width: 560px;
	}

	.checks {
		display: flex;
		flex-direction: column;
		background: var(--surface-100);
		box-shadow: rgba(38, 37, 30, 0.1) 0 0 0 1px inset;
		border-radius: var(--radius-comfortable);
		overflow: hidden;
		margin-bottom: 16px;
	}
	.checkrow-item {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 11px 16px;
		font-size: 13.5px;
	}
	.checkrow-item + .checkrow-item {
		border-top: 1px solid var(--border-subtle);
	}
	.checkrow-item.bad {
		color: var(--color-error);
	}
	.k {
		color: var(--text-secondary);
	}
	.v {
		margin-left: auto;
	}
	.mono {
		font-family: var(--font-mono);
		font-size: 12px;
	}
	.ok {
		color: var(--color-success);
	}
	.dirtybox {
		margin-left: auto;
		display: inline-flex;
		align-items: center;
		gap: 8px;
		font-size: 12.5px;
		text-align: right;
		max-width: 320px;
		line-height: 1.4;
	}
	.golink {
		display: inline-flex;
		align-items: center;
		gap: 5px;
		border: none;
		background: transparent;
		color: var(--color-accent);
		font-size: 12.5px;
		cursor: pointer;
		padding: 3px 6px;
		border-radius: var(--radius-small);
		white-space: nowrap;
	}
	.golink:hover {
		filter: brightness(1.1);
	}

	.autorow {
		display: flex;
		align-items: center;
		gap: 9px;
		font-size: 13.5px;
		cursor: pointer;
		user-select: none;
		margin-bottom: 20px;
	}
	.autorow input {
		position: absolute;
		opacity: 0;
		width: 0;
		height: 0;
	}
	.knobbox {
		width: 15px;
		height: 15px;
		border-radius: var(--radius-medium);
		border: 1px solid var(--border-strong);
		display: inline-flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}
	.autorow input:checked + .knobbox {
		background: var(--color-accent);
		border-color: var(--color-accent);
	}
	.autorow input:checked + .knobbox::after {
		content: '';
		width: 4px;
		height: 8px;
		border-right: 2px solid #fff;
		border-bottom: 2px solid #fff;
		transform: rotate(45deg) translate(-1px, -1px);
	}

	.bar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
		padding-top: 16px;
		border-top: 1px solid var(--border-subtle);
	}
	.center-bar {
		justify-content: center;
		margin-top: 22px;
	}

	.panel {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 12px;
		text-align: center;
		padding: 30px 32px;
		background: var(--surface-100);
		box-shadow:
			rgba(38, 37, 30, 0.1) 0 0 0 1px inset,
			var(--shadow-card);
		border-radius: 14px;
	}
	.halo {
		width: 68px;
		height: 68px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		animation: pop-in 0.4s cubic-bezier(0.22, 1, 0.36, 1) both;
	}
	.panel.ok .halo {
		background: rgba(31, 138, 101, 0.1);
		animation: ring-pulse 1.8s ease-out 0.15s both;
	}
	.panel.ok :global(.icon),
	.panel.ok .halo :global(.icon) {
		color: var(--color-success);
	}
	.panel.bad .halo {
		background: rgba(207, 45, 86, 0.09);
	}
	.panel.bad :global(.icon) {
		color: var(--color-error);
	}
	h2 {
		margin: 4px 0 0;
		font-size: 24px;
		font-weight: 400;
		letter-spacing: -0.325px;
	}
	.desc {
		margin: 0;
		font-family: var(--font-serif);
		font-size: 14.5px;
		color: var(--text-secondary);
	}
	.chip {
		font-family: var(--font-mono);
		font-size: 12px;
		color: var(--color-success);
		background: rgba(31, 138, 101, 0.12);
		padding: 5px 13px;
		border-radius: var(--radius-pill);
	}
	.files {
		list-style: none;
		margin: 4px 0 0;
		padding: 10px 16px;
		background: var(--surface-300);
		border-radius: var(--radius-comfortable);
		font-family: var(--font-mono);
		font-size: 12px;
		line-height: 1.8;
		color: var(--color-error);
		text-align: left;
		max-width: 100%;
		word-break: break-all;
		user-select: text;
	}

	.friendly {
		margin: 0;
		font-family: var(--font-serif);
		font-size: 14.5px;
		line-height: 1.65;
		text-align: left;
		background: var(--surface-300);
		border-radius: var(--radius-comfortable);
		padding: 12px 16px;
		max-width: 100%;
		white-space: pre-wrap;
	}
	.rawwrap {
		width: 100%;
		text-align: left;
	}
	.rawwrap summary {
		font-size: 11.5px;
		color: var(--text-tertiary);
		cursor: pointer;
		padding: 2px 0 6px;
	}
	.errbox {
		margin: 4px 0 0;
		padding: 12px 16px;
		background: var(--surface-300);
		border-radius: var(--radius-comfortable);
		font-family: var(--font-mono);
		font-size: 12px;
		line-height: 1.67;
		color: var(--color-error);
		white-space: pre-wrap;
		word-break: break-all;
		user-select: text;
	}
</style>
