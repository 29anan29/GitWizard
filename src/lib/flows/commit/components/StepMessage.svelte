<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import wand from '$lib/assets/icons/wand.svg?raw';
	import { config } from '$lib/state/config.svelte';
	import { i18n, t } from '$lib/i18n/index.svelte';
	import { suggestMessage } from '../suggest';
	import type { Wizard } from '../state.svelte';

	let { wizard }: { wizard: Wizard } = $props();

	let suggestion = $state('');
	const valid = $derived(wizard.state.message.trim().length >= 3);

	function applyPrefix(p: string): void {
		if (wizard.state.prefix === p) {
			wizard.state.prefix = '';
			wizard.state.message = wizard.state.message.replace(/^[a-z]+(\([^)]*\))?:\s*/, '');
			return;
		}
		wizard.state.prefix = p;
		const rest = wizard.state.message.replace(/^[a-z]+(\([^)]*\))?:\s*/, '');
		wizard.state.message = `${p}: ${rest}`;
	}

	function generate(): void {
		suggestion = suggestMessage([...wizard.selected], i18n.locale);
	}

	function applySuggestion(): void {
		if (!suggestion) return;
		const rest = wizard.state.message.replace(/^[a-z]+(\([^)]*\))?:\s*/, '');
		const body = rest.startsWith(suggestion) ? '' : rest;
		wizard.state.message = body ? `${suggestion}\n\n${body}` : suggestion;
	}
</script>

<div class="wrap">
	<div class="chips">
		<span class="label">{t('message.prefixHint')}</span>
		{#each config.commitPrefixes as p (p)}
			<button class="chip" class:on={wizard.state.prefix === p} onclick={() => applyPrefix(p)}>
				{p}:
			</button>
		{/each}
	</div>

	<textarea
		rows="7"
		bind:value={wizard.state.message}
		placeholder={t('message.placeholder')}
		spellcheck="false"
	></textarea>

	<div class="suggest">
		<button class="genbtn" onclick={generate}>
			<Icon svg={wand} size={15} />
			{t('message.suggest')}
		</button>
		{#if suggestion}
			<p class="preview">{suggestion}</p>
			<Button variant="pill" onclick={applySuggestion}>{t('message.apply')}</Button>
		{/if}
	</div>

	{#if !valid && wizard.state.message.length > 0}
		<p class="error">{t('message.required')}</p>
	{/if}

	<footer class="bar">
		<Button variant="ghost" onclick={() => (wizard.state.step = 1)}>{t('common.back')}</Button>
		<Button variant="accent" disabled={!valid} onclick={() => (wizard.state.step = 3)}>
			{t('common.next')}
		</Button>
	</footer>
</div>

<style>
	.wrap {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.chips {
		display: flex;
		align-items: center;
		gap: 7px;
		flex-wrap: wrap;
	}
	.label {
		font-size: 11px;
		text-transform: uppercase;
		letter-spacing: 0.048px;
		color: var(--text-secondary);
		margin-right: 3px;
	}
	.chip {
		background: var(--surface-400);
		border: none;
		color: rgba(38, 37, 30, 0.6);
		font-family: var(--font-mono);
		font-size: 12px;
		padding: 5px 13px;
		border-radius: var(--radius-pill);
		cursor: pointer;
		transition:
			all 150ms ease;
	}
	.chip:hover {
		color: var(--color-error);
	}
	.chip.on {
		background: var(--color-text);
		color: var(--surface-200);
	}

	textarea {
		width: 100%;
		resize: vertical;
		min-height: 140px;
		background: var(--surface-100);
		border: 1px solid var(--border-subtle);
		border-radius: var(--radius-comfortable);
		padding: 14px 16px;
		font-family: var(--font-mono);
		font-size: 13px;
		line-height: 1.67;
		outline: none;
		transition: border-color 150ms ease;
	}
	textarea:focus {
		border-color: var(--color-accent);
	}

	.suggest {
		display: flex;
		align-items: center;
		gap: 12px;
		flex-wrap: wrap;
		padding: 10px 14px;
		border: 1px dashed var(--border-medium);
		border-radius: var(--radius-comfortable);
	}
	.genbtn {
		display: inline-flex;
		align-items: center;
		gap: 7px;
		background: transparent;
		border: none;
		font-size: 13px;
		color: var(--text-tertiary);
		cursor: pointer;
		padding: 4px 6px;
		border-radius: var(--radius-small);
		transition: color 150ms ease;
	}
	.genbtn:hover {
		color: var(--color-accent);
	}
	.preview {
		margin: 0;
		font-family: var(--font-mono);
		font-size: 12px;
		color: var(--color-text);
		flex: 1;
		min-width: 200px;
		user-select: text;
	}
	.error {
		margin: 0;
		font-size: 12.5px;
		color: var(--color-error);
	}

	.bar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
		margin-top: 8px;
		padding-top: 16px;
		border-top: 1px solid var(--border-subtle);
	}
</style>
