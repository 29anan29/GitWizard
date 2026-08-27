<script lang="ts">
	import { fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { goto } from '$app/navigation';
	import WizardNav from '$lib/components/WizardNav.svelte';
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import check from '$lib/assets/icons/circle-check.svg?raw';
	import alertTriangle from '$lib/assets/icons/alert-triangle.svg?raw';
	import { repoStore, refresh as refreshRepo } from '$lib/state/repo.svelte';
	import { git } from '$lib/services/git';
	import { t } from '$lib/i18n/index.svelte';

	let content = $state('');
	let busy = $state(false);
	let saved = $state(false);
	let loadError = $state('');
	let saveError = $state('');

	const PLACEHOLDER = $derived(t('ignore.placeholder'));

	async function load(): Promise<void> {
		if (!repoStore.info) return;
		busy = true;
		loadError = '';
		try {
			content = await git.getGitignore(repoStore.info.path);
		} catch (e) {
			loadError = typeof e === 'string' ? e : String(e);
		} finally {
			busy = false;
		}
	}

	async function save(): Promise<void> {
		if (!repoStore.info) return;
		busy = true;
		saveError = '';
		saved = false;
		try {
			await git.setGitignore(repoStore.info.path, content);
			saved = true;
			await refreshRepo();
			setTimeout(() => (saved = false), 2000);
		} catch (e) {
			saveError = typeof e === 'string' ? e : String(e);
		} finally {
			busy = false;
		}
	}

	load();
</script>

<div class="container page">
	<WizardNav title={t('ignore.title')} steps={[t('ignore.title')]} current={1} {busy} oncancel={() => goto('/')} />

	{#if !repoStore.info}
		<section class="guard">
			<h2>{t('guard.noRepo.title')}</h2>
			<p>{t('guard.noRepo.desc')}</p>
			<div class="actions">
				<Button variant="ghost" onclick={() => goto('/')}>{t('run.backHome')}</Button>
			</div>
		</section>
	{:else}
		<section class="body" in:fly={{ y: 14, duration: 240, easing: cubicOut }}>
			<div class="toolbar">
				<span class="repo">{repoStore.info.name}</span>
				<div class="spacer"></div>
				{#if saved}
					<span class="saved-badge"><Icon svg={check} size={12} /> {t('ignore.saved')}</span>
				{/if}
				<Button variant="accent" disabled={busy} onclick={save}>{t('common.save')}</Button>
			</div>

			<div class="editor-wrap">
				<textarea
					class="editor"
					spellcheck="false"
					placeholder={PLACEHOLDER}
					bind:value={content}
				></textarea>
			</div>

			{#if loadError}
				<p class="errline">{loadError}</p>
			{/if}
			{#if saveError}
				<p class="errline">{saveError}</p>
			{/if}

			<footer class="foot">
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

	.body {
		max-width: 720px;
		margin: 28px auto 0;
		padding: 0 8px;
	}

	.toolbar {
		display: flex;
		align-items: center;
		gap: 10px;
		margin-bottom: 14px;
	}
	.repo {
		font-family: var(--font-mono);
		font-size: 13px;
		color: var(--text-secondary);
	}
	.spacer {
		flex: 1;
	}
	.saved-badge {
		display: inline-flex;
		align-items: center;
		gap: 5px;
		font-size: 12.5px;
		color: var(--color-success);
		background: rgba(31, 138, 101, 0.1);
		padding: 4px 10px;
		border-radius: var(--radius-pill);
	}

	.editor-wrap {
		background: var(--surface-400);
		box-shadow: rgba(38, 37, 30, 0.1) 0 0 0 1px inset;
		border-radius: var(--radius-featured);
		padding: 4px;
		margin-bottom: 12px;
	}
	.editor {
		width: 100%;
		min-height: 420px;
		border: none;
		background: transparent;
		font-family: var(--font-mono);
		font-size: 13px;
		line-height: 1.65;
		color: var(--color-text);
		resize: vertical;
		padding: 14px 16px;
		outline: none;
		box-sizing: border-box;
	}
	.editor::placeholder {
		color: var(--text-tertiary);
	}

	.errline {
		margin: 0 0 12px;
		font-size: 12.5px;
		color: var(--color-error);
		white-space: pre-wrap;
	}

	.foot {
		display: flex;
		justify-content: center;
		margin-top: 24px;
	}
</style>
