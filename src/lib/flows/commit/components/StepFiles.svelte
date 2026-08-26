<script lang="ts">
	import FileList from '$lib/components/FileList.svelte';
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import fileText from '$lib/assets/icons/file-text.svg?raw';
	import { repoStore } from '$lib/state/repo.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import type { Wizard } from '../state.svelte';

	let { wizard }: { wizard: Wizard } = $props();

	const selectedCount = $derived(wizard.selected.size);
</script>

{#if repoStore.entries.length === 0}
	<div class="empty">
		<Icon svg={fileText} size={36} />
		<h2>{t('files.empty.title')}</h2>
		<p>{t('files.empty.desc')}</p>
		{#if (repoStore.info?.ahead ?? 0) > 0}
			<Button
				variant="surface"
				onclick={() => {
					wizard.state.step = 4;
					void wizard.pushOnly();
				}}
			>
				{t('files.empty.push')}
			</Button>
			<span class="aheadnote">{t('status.ahead', { n: repoStore.info!.ahead })}</span>
		{/if}
	</div>
{:else}
	<FileList
		entries={repoStore.entries}
		selected={wizard.selected}
		toggle={wizard.toggle}
		setGroup={wizard.setGroup}
	/>

	<footer class="bar">
		<span class="info">
			{t('files.selected', { n: selectedCount })}
			{#if selectedCount > 0 && wizard.state.summary.insertions + wizard.state.summary.deletions > 0}
				<em>{t('files.stats', wizard.state.summary)}</em>
			{/if}
		</span>
		<Button
			variant="accent"
			disabled={selectedCount === 0}
			onclick={() => {
				void wizard.computeSummary();
				wizard.state.step = 2;
			}}
		>
			{t('common.next')}
		</Button>
	</footer>
{/if}

<style>
	.empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 10px;
		padding: 80px 24px;
		text-align: center;
		color: var(--text-secondary);
	}
	.empty h2 {
		margin: 6px 0 0;
		font-size: 22px;
		font-weight: 400;
		color: var(--color-text);
	}
	.empty p {
		margin: 0;
		font-family: var(--font-serif);
		font-size: 15.5px;
		max-width: 380px;
	}
	.aheadnote {
		font-family: var(--font-mono);
		font-size: 11px;
		color: var(--color-gold);
		background: rgba(192, 133, 50, 0.1);
		padding: 4px 11px;
		border-radius: var(--radius-pill);
	}

	.bar {
		position: sticky;
		bottom: 0;
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
		margin-top: 26px;
		padding: 14px 18px;
		background: var(--surface-200);
		border-top: 1px solid var(--border-subtle);
	}
	.info {
		font-size: 14px;
	}
	.info em {
		font-style: normal;
		font-family: var(--font-mono);
		font-size: 12px;
		color: var(--text-secondary);
		margin-left: 8px;
	}
</style>
