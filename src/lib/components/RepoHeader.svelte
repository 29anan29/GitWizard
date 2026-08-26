<script lang="ts">
	import Icon from './Icon.svelte';
	import Badge from './Badge.svelte';
	import folderOpen from '$lib/assets/icons/folder-open.svg?raw';
	import refreshIcon from '$lib/assets/icons/refresh.svg?raw';
	import settings from '$lib/assets/icons/settings.svg?raw';
	import chevronDown from '$lib/assets/icons/chevron-down.svg?raw';
	import { repoStore, openDialog, openPath } from '$lib/state/repo.svelte';
	import { config } from '$lib/state/config.svelte';
	import { i18n, t } from '$lib/i18n/index.svelte';
	import { persistConfig } from '$lib/state/config.svelte';
	import SettingsModal from './SettingsModal.svelte';

	let picker = $state(false);
	let showSettings = $state(false);

	function switchLocale(): void {
		i18n.locale = i18n.locale === 'zh-CN' ? 'en' : 'zh-CN';
		config.locale = i18n.locale;
		persistConfig();
	}

	async function pick(path: string): Promise<void> {
		picker = false;
		try {
			await openPath(path);
		} catch {
			alert(t('repo.invalid'));
		}
	}

	async function browse(): Promise<void> {
		picker = false;
		try {
			await openDialog();
		} catch {
			alert(t('repo.invalid'));
		}
	}

	async function refresh(): Promise<void> {
		await refreshStatus();
	}

	import { refresh as refreshStatus } from '$lib/state/repo.svelte';
</script>

<header>
	<div class="left">
		<div class="repowrap">
			<button class="repo" onclick={() => (picker = !picker)}>
				<Icon svg={folderOpen} size={16} />
				<span class="name">{repoStore.info?.name ?? t('repo.open')}</span>
				<Icon svg={chevronDown} size={13} />
			</button>

			{#if picker}
				<button class="backdrop" aria-label="close" onclick={() => (picker = false)}></button>
				<div class="popover">
					<span class="micro">{t('repo.recent')}</span>
					{#if config.recentRepos.length === 0}
						<span class="none">{t('repo.empty.title')}</span>
					{/if}
					{#each config.recentRepos as p (p)}
						<button class="item" onclick={() => pick(p)}>{p}</button>
					{/each}
					<div class="sep"></div>
					<button class="item accent" onclick={browse}>{t('repo.openAction')}</button>
				</div>
			{/if}
		</div>

		{#if repoStore.info}
			<span class="branch">
				{repoStore.info.branch ?? 'HEAD'}
				{#if repoStore.info.ahead > 0}<em>+{repoStore.info.ahead}</em>{/if}
				{#if repoStore.info.behind > 0}<em>−{repoStore.info.behind}</em>{/if}
			</span>
			{#if repoStore.info.dirtyCount > 0}
				<Badge n={repoStore.info.dirtyCount} />
			{/if}
			<button
				class="iconbtn"
				class:spin={repoStore.loading}
				onclick={refresh}
				aria-label={t('repo.refresh')}
			>
				<Icon svg={refreshIcon} size={14} />
			</button>
		{/if}
	</div>

	<div class="right">
		<button class="lang" onclick={switchLocale}>{t('lang.switch')}</button>
		<button class="iconbtn" onclick={() => (showSettings = true)} aria-label={t('settings.title')}>
			<Icon svg={settings} size={15} />
		</button>
	</div>
</header>

{#if showSettings}
	<SettingsModal onclose={() => (showSettings = false)} />
{/if}

<style>
	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
		padding: 14px 24px;
		border-bottom: 1px solid var(--border-subtle);
		position: relative;
		z-index: 30;
		background: var(--surface-200);
	}

	.left,
	.right {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	.repowrap {
		position: relative;
	}
	.repo {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		background: var(--surface-300);
		border: none;
		padding: 8px 10px 8px 12px;
		border-radius: var(--radius-comfortable);
		font-size: 14px;
		cursor: pointer;
		max-width: 320px;
		transition: color 150ms ease;
	}
	.repo:hover {
		color: var(--color-error);
	}
	.name {
		font-weight: 500;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.backdrop {
		position: fixed;
		inset: 0;
		background: transparent;
		border: none;
		cursor: default;
		z-index: 31;
	}
	.popover {
		position: absolute;
		top: calc(100% + 6px);
		left: 0;
		width: 300px;
		max-height: 320px;
		overflow-y: auto;
		background: var(--surface-100);
		border-radius: var(--radius-featured);
		box-shadow: var(--shadow-card);
		padding: 10px;
		display: flex;
		flex-direction: column;
		gap: 3px;
		z-index: 32;
	}
	.micro {
		font-size: 11px;
		text-transform: uppercase;
		color: var(--text-secondary);
		padding: 2px 6px 6px;
	}
	.none {
		font-size: 13px;
		color: var(--text-secondary);
		padding: 4px 6px 8px;
	}
	.item {
		text-align: left;
		background: transparent;
		border: none;
		font-family: var(--font-mono);
		font-size: 11px;
		line-height: 1.5;
		padding: 7px 8px;
		border-radius: var(--radius-comfortable);
		cursor: pointer;
		word-break: break-all;
		transition: background-color 120ms ease, color 150ms ease;
	}
	.item:hover {
		background: var(--surface-300);
		color: var(--color-error);
	}
	.item.accent {
		color: var(--color-accent);
	}
	.sep {
		height: 1px;
		background: var(--border-subtle);
		margin: 6px 2px;
	}

	.branch {
		font-family: var(--font-mono);
		font-size: 11px;
		line-height: 1;
		color: var(--text-tertiary);
		background: var(--surface-400);
		padding: 6px 10px;
		border-radius: var(--radius-pill);
		display: inline-flex;
		align-items: center;
		gap: 6px;
	}
	.branch em {
		font-style: normal;
		color: var(--color-gold);
	}

	.lang {
		background: var(--surface-400);
		color: var(--text-tertiary);
		border: none;
		border-radius: var(--radius-pill);
		padding: 6px 13px;
		font-size: 12px;
		cursor: pointer;
		transition: color 150ms ease;
	}
	.lang:hover {
		color: var(--color-error);
	}

	.iconbtn {
		background: rgba(38, 37, 30, 0.06);
		border: none;
		color: rgba(38, 37, 30, 0.55);
		width: 30px;
		height: 30px;
		border-radius: var(--radius-comfortable);
		display: inline-flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		transition: color 150ms ease;
	}
	.iconbtn:hover:not(:disabled) {
		color: var(--color-error);
	}
	.iconbtn.spin :global(.icon) {
		animation: rotate 0.9s linear infinite;
	}
	@keyframes rotate {
		to {
			transform: rotate(360deg);
		}
	}
</style>
