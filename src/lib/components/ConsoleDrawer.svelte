<script lang="ts">
	import Icon from './Icon.svelte';
	import terminal from '$lib/assets/icons/terminal-2.svg?raw';
	import chevronDown from '$lib/assets/icons/chevron-down.svg?raw';
	import x from '$lib/assets/icons/x.svg?raw';
	import { consoleStore, toggleConsole, clearConsole } from '$lib/state/console.svelte';
	import { t } from '$lib/i18n/index.svelte';

	let linesEl: HTMLDivElement | undefined = $state();

	$effect(() => {
		void consoleStore.lines.length;
		if (linesEl) linesEl.scrollTop = linesEl.scrollHeight;
	});
</script>

{#if !consoleStore.open}
	<button class="tab" onclick={toggleConsole}>
		<Icon svg={terminal} size={13} />
		<span>{t('console.toggle')}</span>
	</button>
{/if}

{#if consoleStore.open}
	<section class="drawer">
		<header>
			<span class="title"><Icon svg={terminal} size={14} /> {t('console.toggle')}</span>
			<div class="actions">
				<button onclick={clearConsole}>{t('console.clear')}</button>
				<button onclick={toggleConsole} aria-label="collapse">
					<Icon svg={chevronDown} size={15} />
				</button>
				<button onclick={toggleConsole} aria-label="close">
					<Icon svg={x} size={15} />
				</button>
			</div>
		</header>
		<div class="lines" bind:this={linesEl}>
			{#if consoleStore.lines.length === 0}
				<p class="empty">{t('console.empty')}</p>
			{:else}
				{#each consoleStore.lines as l}
					<p class={l.kind}>{l.kind === 'cmd' ? '$ ' : ''}{l.line}</p>
				{/each}
			{/if}
		</div>
	</section>
{/if}

<style>
	.tab {
		position: fixed;
		right: 18px;
		bottom: 16px;
		z-index: 40;
		display: inline-flex;
		align-items: center;
		gap: 7px;
		background: var(--surface-500);
		color: rgba(38, 37, 30, 0.6);
		border: none;
		padding: 7px 14px;
		border-radius: var(--radius-pill);
		font-size: 12px;
		cursor: pointer;
		box-shadow: var(--shadow-ambient);
		transition: color 150ms ease;
	}
	.tab:hover {
		color: var(--color-error);
	}

	.drawer {
		position: fixed;
		left: 0;
		right: 0;
		bottom: 0;
		z-index: 50;
		height: 250px;
		display: flex;
		flex-direction: column;
		background: #191813;
		box-shadow: 0 -10px 40px rgba(38, 37, 30, 0.25);
	}
	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 8px 16px;
		border-bottom: 1px solid rgba(242, 241, 237, 0.12);
	}
	.title {
		display: inline-flex;
		align-items: center;
		gap: 7px;
		color: #b9b6a8;
		font-size: 12px;
	}
	.actions {
		display: flex;
		gap: 4px;
	}
	header button {
		background: transparent;
		border: none;
		color: #8f8c7e;
		font-size: 11px;
		cursor: pointer;
		padding: 4px 8px;
		border-radius: var(--radius-small);
		display: inline-flex;
		align-items: center;
	}
	header button:hover {
		color: #f2f1ed;
	}

	.lines {
		flex: 1;
		overflow-y: auto;
		padding: 10px 16px 14px;
		font-family: var(--font-mono);
		font-size: 12px;
		line-height: 1.67;
		user-select: text;
	}
	.lines p {
		margin: 0;
		white-space: pre-wrap;
		word-break: break-all;
	}
	p.cmd {
		color: #d0a35a;
		margin-top: 8px;
	}
	p.out {
		color: #cfccc0;
	}
	p.err {
		color: #ff7d97;
	}
	.empty {
		color: #6d6a5e;
	}
</style>
