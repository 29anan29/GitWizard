<script lang="ts">
import Icon from './Icon.svelte';
import Badge from './Badge.svelte';
import { t, type Key } from '$lib/i18n/index.svelte';

interface Props {
	icon: string;
	titleKey: Key;
	descKey: Key;
	badge?: number;
	enabled?: boolean;
	onclick?: () => void;
}

	let { icon, titleKey, descKey, badge = 0, enabled = true, onclick }: Props = $props();
</script>

<button class="card" class:off={!enabled} onclick={enabled ? onclick : undefined} disabled={!enabled}>
	<span class="icon"><Icon svg={icon} size={22} /></span>
	<span class="texts">
		<span class="title">{t(titleKey)}</span>
		<span class="desc">{t(descKey)}</span>
	</span>
	{#if enabled && badge > 0}
		<Badge n={badge} />
	{:else if !enabled}
		<span class="soon">{t('badge.soon')}</span>
	{/if}
</button>

<style>
	.card {
		display: grid;
		grid-template-columns: auto 1fr auto;
		gap: 14px;
		align-items: center;
		text-align: left;
		background: var(--surface-400);
		box-shadow: rgba(38, 37, 30, 0.1) 0 0 0 1px inset;
		border-radius: var(--radius-featured);
		padding: 18px;
		cursor: pointer;
		transition:
			box-shadow 200ms ease,
			transform 160ms ease,
			background-color 200ms ease;
	}
	.card:hover:not(.off) {
		background: var(--surface-300);
		box-shadow: var(--shadow-card);
		transform: translateY(-2px);
	}
	.card:focus-visible {
		outline: none;
		box-shadow: var(--shadow-focus), rgba(38, 37, 30, 0.1) 0 0 0 1px inset;
	}
	.off {
		cursor: default;
		opacity: 0.55;
	}

	.icon {
		width: 42px;
		height: 42px;
		border-radius: var(--radius-comfortable);
		background: linear-gradient(140deg, var(--surface-100), var(--surface-500));
		box-shadow:
			rgba(38, 37, 30, 0.1) 0 0 0 1px inset,
			var(--shadow-ambient);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-text);
		transition: color 180ms ease;
	}
	.card:hover:not(.off) .icon {
		color: var(--color-accent);
	}

	.texts {
		display: flex;
		flex-direction: column;
		gap: 3px;
		min-width: 0;
	}
	.title {
		font-size: 17px;
		letter-spacing: -0.11px;
		line-height: 1.3;
		transition: color 160ms ease;
	}
	.card:hover:not(.off) .title {
		color: var(--color-error);
	}
	.desc {
		font-family: var(--font-serif);
		font-size: 13.5px;
		line-height: 1.35;
		color: var(--text-secondary);
	}

	.soon {
		font-size: 11px;
		color: var(--text-tertiary);
		background: var(--surface-500);
		padding: 3px 8px;
		border-radius: var(--radius-pill);
		white-space: nowrap;
	}
</style>
