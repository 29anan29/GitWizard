<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		variant?: 'surface' | 'pill' | 'ghost' | 'accent';
		size?: 'sm' | 'md';
		disabled?: boolean;
		onclick?: () => void;
		children?: Snippet;
	}

	let {
		variant = 'surface',
		size = 'md',
		disabled = false,
		onclick,
		children
	}: Props = $props();
</script>

<button class="{variant} {size}" {disabled} {onclick}>
	{@render children?.()}
</button>

<style>
	button {
		border: none;
		cursor: pointer;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
		font-weight: 400;
		font-size: 14px;
		line-height: 1;
		transition:
			color 150ms ease,
			background-color 200ms ease,
			box-shadow 200ms ease,
			transform 120ms ease;
	}
	button:active:not(:disabled) {
		transform: translateY(1px);
	}
	button:focus-visible {
		outline: none;
		box-shadow: var(--shadow-focus);
	}
	button:disabled {
		opacity: 0.45;
		cursor: not-allowed;
	}

	.sm {
		padding: 6px 12px;
		font-size: 13px;
	}
	.md {
		padding: 10px 16px;
	}

	.surface {
		background: var(--surface-300);
		color: var(--color-text);
		border-radius: var(--radius-comfortable);
	}
	.surface:hover:not(:disabled) {
		color: var(--color-error);
	}

	.accent {
		background: linear-gradient(180deg, #ff5d0b, #f04b00);
		color: #ffffff;
		border-radius: var(--radius-comfortable);
		box-shadow: 0 1px 2px rgba(240, 75, 0, 0.35);
	}
	.accent:hover:not(:disabled) {
		filter: brightness(1.05);
		box-shadow: 0 8px 22px rgba(245, 78, 0, 0.3);
		transform: translateY(-1px);
	}

	.pill {
		background: var(--surface-400);
		color: var(--text-tertiary);
		border-radius: var(--radius-pill);
	}
	.pill:hover:not(:disabled) {
		color: var(--color-error);
	}

	.ghost {
		background: rgba(38, 37, 30, 0.06);
		color: rgba(38, 37, 30, 0.55);
		border-radius: var(--radius-comfortable);
	}
	.ghost:hover:not(:disabled) {
		color: var(--color-error);
	}
</style>
