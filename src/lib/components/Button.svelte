<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		variant?: 'surface' | 'pill' | 'ghost' | 'accent';
		disabled?: boolean;
		onclick?: () => void;
		children?: Snippet;
	}

	let { variant = 'surface', disabled = false, onclick, children }: Props = $props();
</script>

<button class={variant} {disabled} {onclick}>
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

	.surface {
		background: var(--surface-300);
		color: var(--color-text);
		padding: 10px 12px 10px 14px;
		border-radius: var(--radius-comfortable);
		font-size: 14px;
		line-height: 1;
	}
	.surface:hover:not(:disabled) {
		color: var(--color-error);
	}

	.accent {
		background: var(--color-accent);
		color: #ffffff;
		padding: 11px 20px;
		border-radius: var(--radius-comfortable);
		font-size: 14px;
		line-height: 1;
	}
	.accent:hover:not(:disabled) {
		filter: brightness(1.06);
	}

	.pill {
		background: var(--surface-400);
		color: var(--text-tertiary);
		padding: 6px 14px;
		border-radius: var(--radius-pill);
		font-size: 14px;
		line-height: 1.4;
	}
	.pill:hover:not(:disabled) {
		color: var(--color-error);
	}

	.ghost {
		background: rgba(38, 37, 30, 0.06);
		color: rgba(38, 37, 30, 0.55);
		padding: 6px 12px;
		border-radius: var(--radius-comfortable);
		font-size: 13px;
		line-height: 1.4;
	}
	.ghost:hover:not(:disabled) {
		color: var(--color-error);
	}
</style>
