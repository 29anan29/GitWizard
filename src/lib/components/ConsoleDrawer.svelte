<script lang="ts">
	import { fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import Icon from './Icon.svelte';
	import terminal from '$lib/assets/icons/terminal-2.svg?raw';
	import chevronDown from '$lib/assets/icons/chevron-down.svg?raw';
	import x from '$lib/assets/icons/x.svg?raw';
	import { consoleStore, toggleConsole, clearConsole } from '$lib/state/console.svelte';
	import { t } from '$lib/i18n/index.svelte';

	let linesEl: HTMLDivElement | undefined = $state();
	let seen = $state(0);

	$effect(() => {
		if (consoleStore.open && linesEl) {
			linesEl.scrollTop = linesEl.scrollHeight;
			seen = consoleStore.lines.length;
		}
	});

	const unread = $derived(consoleStore.lines.length - seen);
</script>

{#if !consoleStore.open}
	<button class="tab" onclick={toggleConsole} transition:fly={{ y: 20, duration: 200 }}>
		<Icon svg={terminal} size={13} />
		<span>{t('console.toggle')}</span>
		{#if unread > 0}
			<span class="dot"></span>
		{/if}
	</button>
{/if}

{#if consoleStore.open}
	<section
		class="sheet"
		transition:fly={{ y: 26, duration: 240, easing: cubicOut }}
		aria-label={t('console.toggle')}
	>
		<header>
			<div class="dots"><i></i><i></i><i></i></div>
			<span class="title">{t('console.toggle')}</span>
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
		padding: 8px 15px;
		border-radius: var(--radius-pill);
		font-size: 12px;
		cursor: pointer;
		box-shadow:
			rgba(38, 37, 30, 0.1) 0 0 0 1px inset,
			var(--shadow-card);
		transition:
			color 150ms ease,
			transform 160ms ease;
	}
	.tab:hover {
		color: var(--color-error);
		transform: translateY(-2px);
	}
	.dot {
		width: 7px;
		height: 7px;
		border-radius: 50%;
		background: var(--color-accent);
		box-shadow: 0 0 0 3px rgba(245, 78, 0, 0.18);
	}

	.sheet {
		position: fixed;
		right: 18px;
		bottom: 16px;
		z-index: 50;
		width: min(580px, calc(100vw - 36px));
		height: min(320px, 46vh);
		display: flex;
		flex-direction: column;
		background: #191813;
		border-radius: 14px;
		border: 1px solid rgba(242, 241, 237, 0.09);
		box-shadow: var(--shadow-card);
		overflow: hidden;
	}
	header {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 9px 14px;
		border-bottom: 1px solid rgba(242, 241, 237, 0.1);
	}
	.dots {
		display: flex;
		gap: 5px;
	}
	.dots i {
		width: 9px;
		height: 9px;
		border-radius: 50%;
		display: inline-block;
	}
	.dots i:nth-child(1) {
		background: #e2556f;
	}
	.dots i:nth-child(2) {
		background: #d9a44a;
	}
	.dots i:nth-child(3) {
		background: #57a583;
	}
	.title {
		font-size: 12px;
		color: #b9b6a8;
	}
	.actions {
		margin-left: auto;
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
		transition: color 140ms ease;
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
