<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import Button from './Button.svelte';
	import Icon from './Icon.svelte';
	import x from '$lib/assets/icons/x.svg?raw';
	import { git } from '$lib/services/git';
	import { t } from '$lib/i18n/index.svelte';
	import { getVersion } from '@tauri-apps/api/app';
	import type { Update as UpdaterUpdate } from '@tauri-apps/plugin-updater';

	interface Props {
		update: UpdaterUpdate;
		onclose: () => void;
	}
	let { update, onclose }: Props = $props();

	let phase = $state<'confirm' | 'downloading' | 'ready' | 'error'>('confirm');
	let error = $state('');

	const version = $derived(update.version ?? '…');
	let currentVersion = $state('');
	getVersion().then((v) => (currentVersion = v)).catch(() => {});

	async function download(): Promise<void> {
		phase = 'downloading';
		error = '';
		try {
			await git.downloadAndInstall(update);
			phase = 'ready';
		} catch (e) {
			error = typeof e === 'string' ? e : String(e);
			phase = 'error';
		}
	}
</script>

<div
	class="overlay"
	role="presentation"
	transition:fade={{ duration: 150 }}
	onclick={(e) => e.target === e.currentTarget && phase !== 'downloading' && onclose()}
>
	<div
		class="card"
		role="dialog"
		aria-modal="true"
		aria-label={t('update.title')}
		transition:fly={{ y: 18, duration: 220, easing: cubicOut }}
	>
		<header>
			<h2>{t('update.title')}</h2>
			{#if phase !== 'downloading'}
				<button class="close" onclick={onclose} aria-label={t('common.close')}>
					<Icon svg={x} size={16} />
				</button>
			{/if}
		</header>

		{#if phase === 'confirm'}
			<p class="desc">
				{t('update.desc', { from: currentVersion, to: version })}
			</p>
			<div class="actions">
				<Button variant="ghost" onclick={onclose}>{t('update.later')}</Button>
				<Button variant="accent" onclick={() => void download()}>
					{t('update.download')}
				</Button>
			</div>
		{:else if phase === 'downloading'}
			<p class="desc">{t('update.downloading')}</p>
			<div class="progress-bar">
				<div class="progress-fill"></div>
			</div>
		{:else if phase === 'ready'}
			<p class="desc">{t('update.ready')}</p>
			<div class="actions">
				<Button
					variant="accent"
					onclick={async () => {
						try {
							const { exit } = await import('@tauri-apps/plugin-process');
							await exit(0);
						} catch {
							window.location.reload();
						}
					}}
				>
					{t('update.restart')}
				</Button>
			</div>
		{:else if phase === 'error'}
			<p class="errline">{error}</p>
			<div class="actions">
				<Button variant="ghost" onclick={onclose}>{t('common.cancel')}</Button>
				<Button variant="accent" onclick={() => void download()}>{t('common.retry')}</Button>
			</div>
		{/if}
	</div>
</div>

<style>
	.overlay {
		position: fixed;
		inset: 0;
		z-index: 70;
		background: var(--color-overlay);
		backdrop-filter: blur(5px);
		-webkit-backdrop-filter: blur(5px);
		display: flex;
		align-items: center;
		justify-content: center;
	}
	.card {
		width: 400px;
		max-width: calc(100vw - 48px);
		background: var(--surface-100);
		border-radius: 14px;
		box-shadow: var(--shadow-card);
		padding: 22px 24px 20px;
		display: flex;
		flex-direction: column;
		gap: 16px;
	}
	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	h2 {
		margin: 0;
		font-size: 20px;
		font-weight: 400;
		letter-spacing: -0.11px;
	}
	.close {
		background: transparent;
		border: none;
		color: var(--text-secondary);
		cursor: pointer;
		padding: 4px;
		border-radius: var(--radius-small);
	}
	.close:hover {
		color: var(--color-error);
	}
	.desc {
		margin: 0;
		font-family: var(--font-serif);
		font-size: 14px;
		line-height: 1.6;
		color: var(--text-secondary);
	}
	.actions {
		display: flex;
		justify-content: flex-end;
		gap: 8px;
	}

	.progress-bar {
		width: 100%;
		height: 4px;
		background: var(--surface-300);
		border-radius: 2px;
		overflow: hidden;
	}
	.progress-fill {
		height: 100%;
		background: var(--color-accent);
		border-radius: 2px;
		animation: indeterminate 1.4s ease-in-out infinite;
	}
	@keyframes indeterminate {
		0% {
			width: 0%;
			margin-left: 0%;
		}
		50% {
			width: 55%;
			margin-left: 22%;
		}
		100% {
			width: 0%;
			margin-left: 100%;
		}
	}

	.errline {
		margin: 0;
		font-size: 13px;
		color: var(--color-error);
		white-space: pre-wrap;
		word-break: break-all;
	}
</style>
