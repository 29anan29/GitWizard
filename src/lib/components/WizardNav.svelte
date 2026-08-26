<script lang="ts">
	import Icon from './Icon.svelte';
	import chevronLeft from '$lib/assets/icons/chevron-left.svg?raw';
	import x from '$lib/assets/icons/x.svg?raw';
	import { t } from '$lib/i18n/index.svelte';

	interface Props {
		title: string;
		steps: string[];
		current: number;
		busy?: boolean;
		onback?: () => void;
		oncancel?: () => void;
	}

	let { title, steps, current, busy = false, onback, oncancel }: Props = $props();
</script>

<header>
	<div class="left">
		{#if current > 1 && onback}
			<button class="navbtn" onclick={onback} disabled={busy} aria-label="back">
				<Icon svg={chevronLeft} size={16} />
				<span>{t('common.back')}</span>
			</button>
		{/if}
	</div>

	<div class="center">
		<h1>{title}</h1>
		<div class="track" role="progressbar" aria-valuemin={1} aria-valuemax={steps.length} aria-valuenow={current}>
			{#each steps as label, i}
				<div
					class="seg"
					class:done={i + 1 < current}
					class:active={i + 1 === current}
					title={label}
				></div>
			{/each}
		</div>
		<span class="steplabel">{current}/{steps.length} · {steps[current - 1]}</span>
	</div>

	<div class="right">
		<button class="navbtn" onclick={oncancel} disabled={busy} aria-label="cancel">
			<Icon svg={x} size={15} />
			<span>{t('common.cancel')}</span>
		</button>
	</div>
</header>

<style>
	header {
		position: sticky;
		top: 0;
		z-index: 20;
		display: grid;
		grid-template-columns: 1fr auto 1fr;
		align-items: center;
		gap: 16px;
		padding: 12px 24px;
		background: var(--surface-200);
		border-bottom: 1px solid var(--border-subtle);
	}

	.left {
		justify-self: start;
	}
	.right {
		justify-self: end;
	}

	.navbtn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		background: transparent;
		border: none;
		color: rgba(38, 37, 30, 0.55);
		font-size: 13px;
		padding: 6px 10px;
		border-radius: var(--radius-comfortable);
		cursor: pointer;
		transition: color 150ms ease;
	}
	.navbtn:hover:not(:disabled) {
		color: var(--color-error);
	}
	.navbtn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.center {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 6px;
		min-width: 260px;
	}
	h1 {
		margin: 0;
		font-size: 16px;
		font-weight: 400;
		letter-spacing: -0.11px;
		line-height: 1.3;
	}

	.track {
		display: flex;
		gap: 5px;
		width: 220px;
	}
	.seg {
		flex: 1;
		height: 4px;
		border-radius: var(--radius-pill);
		background: var(--border-subtle);
		transition: background-color 200ms ease;
	}
	.seg.done {
		background: var(--border-medium);
	}
	.seg.active {
		background: var(--color-text);
	}

	.steplabel {
		font-size: 11px;
		color: var(--text-secondary);
	}
</style>
