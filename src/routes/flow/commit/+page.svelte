<script lang="ts">
	import WizardNav from '$lib/components/WizardNav.svelte';
	import Button from '$lib/components/Button.svelte';
	import StepFiles from '$lib/flows/commit/components/StepFiles.svelte';
	import StepMessage from '$lib/flows/commit/components/StepMessage.svelte';
	import StepStrategy from '$lib/flows/commit/components/StepStrategy.svelte';
	import StepRun from '$lib/flows/commit/components/StepRun.svelte';
	import folderOpen from '$lib/assets/icons/folder-open.svg?raw';
	import { createWizard } from '$lib/flows/commit/state.svelte';
	import { repoStore, openDialog } from '$lib/state/repo.svelte';
	import { config, persistConfig } from '$lib/state/config.svelte';
	import { goto } from '$app/navigation';
	import { t } from '$lib/i18n/index.svelte';

	const wizard = createWizard();

	const steps = $derived([
		t('commit.step.files'),
		t('commit.step.message'),
		t('commit.step.strategy'),
		t('commit.step.run')
	]);
	const busy = $derived(wizard.state.phase === 'running');

	function back(): void {
		if (!busy) wizard.state.step = Math.max(1, wizard.state.step - 1);
	}

	function cancel(): void {
		if (busy) return;
		wizard.reset();
		goto('/');
	}

	function goHome(): void {
		wizard.reset();
		goto('/');
	}

	function newCommit(): void {
		wizard.reset();
	}

	async function browse(): Promise<void> {
		try {
			await openDialog();
		} catch {
		}
	}

	function execute(): void {
		config.autoPush = wizard.state.autoPush;
		void persistConfig();
		wizard.state.step = 4;
		void wizard.execute();
	}
</script>

<div class="container page">
	<WizardNav
		title={t('commit.title')}
		{steps}
		current={wizard.state.step}
		{busy}
		onback={back}
		oncancel={cancel}
	/>

	{#if !repoStore.info}
		<section class="guard">
			<h2>{t('guard.noRepo.title')}</h2>
			<p>{t('guard.noRepo.desc')}</p>
			<div class="actions">
				<Button variant="accent" onclick={browse}>
					<span class="btnicon">{@html folderOpen}</span>
					{t('repo.openAction')}
				</Button>
				<Button variant="ghost" onclick={() => goto('/')}>{t('run.backHome')}</Button>
			</div>
		</section>
	{:else if wizard.state.step === 1}
		<section class="body"><StepFiles {wizard} /></section>
	{:else if wizard.state.step === 2}
		<section class="body narrow"><StepMessage {wizard} /></section>
	{:else if wizard.state.step === 3}
		<section class="body"><StepStrategy {wizard} onexecute={execute} /></section>
	{:else}
		<section class="body"><StepRun {wizard} onhome={goHome} onnew={newCommit} /></section>
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
		letter-spacing: -0.325px;
	}
	.guard p {
		margin: 0 0 10px;
		font-family: var(--font-serif);
		font-size: 15.5px;
		color: var(--text-secondary);
	}
	.actions {
		display: flex;
		gap: 10px;
	}
	.btnicon {
		display: inline-flex;
		width: 15px;
		height: 15px;
	}
	.btnicon :global(svg) {
		width: 100%;
		height: 100%;
		stroke: currentColor;
	}

	.body {
		max-width: 760px;
		margin: 28px auto 0;
		padding: 0 8px;
	}
	.body.narrow {
		max-width: 620px;
	}
</style>
