<script lang="ts">
	import { fly, slide } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import Icon from './Icon.svelte';
	import Button from './Button.svelte';
	import history from '$lib/assets/icons/history.svg?raw';
	import alertTriangle from '$lib/assets/icons/alert-triangle.svg?raw';
	import { repoStore, refresh as refreshRepo } from '$lib/state/repo.svelte';
	import { git } from '$lib/services/git';
	import { t } from '$lib/i18n/index.svelte';

	let open = $state(false);
	let busy = $state(false);

	const info = $derived(repoStore.info);
	const headHash = $derived(info?.branch ?? 'HEAD');
	const dirty = $derived(repoStore.entries.length > 0);

	async function undoKeepChanges(): Promise<void> {
		if (!info) return;
		busy = true;
		try {
			await git.reset(info.path, 'soft', 'HEAD~1');
			await refreshRepo();
		} catch {
			/* noop */
		} finally {
			busy = false;
		}
	}

	async function undoDiscard(): Promise<void> {
		if (!info) return;
		busy = true;
		try {
			await git.reset(info.path, 'hard', 'HEAD~1');
			await refreshRepo();
		} catch {
			/* noop */
		} finally {
			busy = false;
		}
	}
</script>

{#if info}
	<button class="trigger" onclick={() => (open = !open)} aria-label="undo sidebar">
		<Icon svg={history} size={16} />
	</button>

	{#if open}
		<div class="backdrop" onclick={() => (open = false)} role="presentation" in:fly={{ x: -20, duration: 200 }} out:fly={{ x: -20, duration: 150 }}></div>
		<aside class="sidebar" in:fly={{ x: 0, duration: 240, easing: cubicOut }} out:fly={{ x: -260, duration: 200 }}>
			<div class="head">
				<span class="head-title">{t('undo.title')}</span>
				<button class="close-btn" onclick={() => (open = false)}>✕</button>
			</div>

			<div class="info-card">
				<span class="info-label">{t('undo.headHash')}</span>
				<span class="info-value mono">{headHash}</span>
			</div>

			{#if dirty}
				<p class="warnline">
					<Icon svg={alertTriangle} size={12} />
					{t('undo.dirtyWarn')}
				</p>
			{/if}

			<div class="actions">
				<Button variant="surface" disabled={busy} onclick={() => void undoKeepChanges()}>
					{t('undo.keepChanges')}
				</Button>
				<Button variant="surface" disabled={busy} onclick={() => void undoDiscard()}>
					{t('undo.discard')}
				</Button>
			</div>
		</aside>
	{/if}
{/if}

<style>
	.trigger {
		position: fixed;
		left: 12px;
		bottom: 20px;
		z-index: 30;
		width: 38px;
		height: 38px;
		border-radius: 50%;
		border: none;
		background: var(--surface-400);
		box-shadow: var(--color-card-border) 0 0 0 1px inset, var(--shadow-card);
		color: var(--text-tertiary);
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 150ms ease;
	}
	.trigger:hover {
		color: var(--color-accent);
		transform: scale(1.05);
	}

	.backdrop {
		position: fixed;
		inset: 0;
		z-index: 40;
		background: rgba(0, 0, 0, 0.15);
	}

	.sidebar {
		position: fixed;
		left: 0;
		top: 0;
		bottom: 0;
		z-index: 50;
		width: 252px;
		background: var(--surface-400);
		box-shadow: var(--shadow-card);
		display: flex;
		flex-direction: column;
		padding: 16px;
		gap: 12px;
	}

	.head {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	.head-title { font-size: 14px; font-weight: 600; }
	.close-btn {
		background: transparent;
		border: none;
		color: var(--text-tertiary);
		font-size: 14px;
		cursor: pointer;
		padding: 4px;
		border-radius: var(--radius-small);
	}
	.close-btn:hover { color: var(--color-error); }

	.info-card {
		background: var(--surface-100);
		box-shadow: var(--color-card-border) 0 0 0 1px inset;
		border-radius: var(--radius-comfortable);
		padding: 10px 14px;
		display: flex;
		flex-direction: column;
		gap: 4px;
	}
	.info-label { font-size: 11px; color: var(--text-secondary); text-transform: uppercase; letter-spacing: 0.04px; }
	.info-value { font-size: 13px; }
	.mono { font-family: var(--font-mono); font-size: 12px; }

	.warnline { margin: 0; font-size: 12px; color: var(--color-gold); display: flex; align-items: center; gap: 6px; }

	.actions { display: flex; flex-direction: column; gap: 8px; }
</style>
