<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import chevronDown from '$lib/assets/icons/chevron-down.svg?raw';
	import { repoStore } from '$lib/state/repo.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import type { Wizard } from '../state.svelte';

	let { wizard, onexecute }: { wizard: Wizard; onexecute: () => void } = $props();

	let showCmd = $state(false);

	const cmds = $derived.by(() => {
		const info = repoStore.info;
		if (!info) return [] as string[];
		const files = [...wizard.selected];
		if (files.length === 0) return [] as string[];
		const head = files.slice(0, 4).join(' ');
		const addLine = `git add ${head}${files.length > 4 ? ` … (+${files.length - 4})` : ''}`;
		const first = wizard.state.message.trim().split('\n')[0];
		const lines = [addLine, `git commit -m "${first}"`];
		if (wizard.state.autoPush && info.branch) {
			lines.push(`git push origin ${info.branch}`);
		}
		return lines;
	});
</script>

<div class="wrap">
	<div class="stats">
		<div class="stat">
			<span class="num">{wizard.selected.size}</span>
			<span class="unit">{t('strategy.filesWillCommit')}</span>
		</div>
		<div class="stat">
			<span class="num ins">+{wizard.state.summary.insertions}</span>
			<span class="unit">insertions</span>
		</div>
		<div class="stat">
			<span class="num del">−{wizard.state.summary.deletions}</span>
			<span class="unit">deletions</span>
		</div>
	</div>

	<label class="checkrow">
		<input type="checkbox" bind:checked={wizard.state.autoPush} />
		<span class="box" aria-hidden="true"></span>
		{t('strategy.autoPush')}
	</label>

	<div class="preview">
		<span class="micro">{t('strategy.preview')}</span>
		<button class="toggle" onclick={() => (showCmd = !showCmd)}>
			{showCmd ? t('strategy.hideCmd') : t('strategy.showCmd')}
			<svg
				class="chev"
				class:open={showCmd}
				viewBox="0 0 24 24"
				width="14"
				height="14"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="m6 9 6 6 6-6" />
			</svg>
		</button>
		{#if showCmd}
			<pre>{cmds.join('\n')}</pre>
		{/if}
	</div>

	<footer class="bar">
		<Button variant="ghost" onclick={() => (wizard.state.step = 2)}>{t('common.back')}</Button>
		<Button variant="accent" disabled={cmds.length === 0} onclick={onexecute}>
			{t('strategy.execute')}
		</Button>
	</footer>
</div>

<style>
	.wrap {
		display: flex;
		flex-direction: column;
		gap: 22px;
		max-width: 640px;
	}

	.stats {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 12px;
	}
	.stat {
		background: var(--surface-400);
		box-shadow: rgba(38, 37, 30, 0.1) 0 0 0 1px inset;
		border-radius: var(--radius-featured);
		padding: 18px 20px;
		display: flex;
		flex-direction: column;
		gap: 4px;
	}
	.num {
		font-size: 28px;
		line-height: 1.1;
		letter-spacing: -0.325px;
		font-family: var(--font-mono);
	}
	.num.ins {
		color: var(--color-success);
	}
	.num.del {
		color: var(--color-error);
	}
	.unit {
		font-family: var(--font-serif);
		font-size: 13px;
		color: var(--text-secondary);
	}

	.checkrow {
		display: flex;
		align-items: center;
		gap: 10px;
		font-size: 14.5px;
		cursor: pointer;
		user-select: none;
		align-self: flex-start;
	}
	.checkrow input {
		position: absolute;
		opacity: 0;
		width: 0;
		height: 0;
	}
	.box {
		width: 17px;
		height: 17px;
		border-radius: var(--radius-medium);
		border: 1px solid var(--border-strong);
		display: inline-flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		transition:
			background-color 120ms ease,
			border-color 120ms ease;
	}
	.checkrow input:checked + .box {
		background: var(--color-accent);
		border-color: var(--color-accent);
	}
	.checkrow input:checked + .box::after {
		content: '';
		width: 5px;
		height: 9px;
		border-right: 2px solid #fff;
		border-bottom: 2px solid #fff;
		transform: rotate(45deg) translate(-1px, -1px);
	}

	.preview {
		display: flex;
		flex-direction: column;
		gap: 8px;
		padding: 14px 16px;
		border: 1px solid var(--border-subtle);
		border-radius: var(--radius-comfortable);
		background: var(--surface-100);
	}
	.micro {
		font-size: 11px;
		text-transform: uppercase;
		letter-spacing: 0.048px;
		color: var(--text-secondary);
	}
	.toggle {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		align-self: flex-start;
		background: transparent;
		border: none;
		color: var(--color-accent);
		font-size: 13px;
		cursor: pointer;
		padding: 0;
	}
	.chev {
		transition: transform 180ms ease;
	}
	.chev.open {
		transform: rotate(180deg);
	}
	pre {
		margin: 4px 0 0;
		padding: 12px 14px;
		background: #191813;
		color: #cfccc0;
		border-radius: var(--radius-comfortable);
		font-family: var(--font-mono);
		font-size: 12px;
		line-height: 1.67;
		white-space: pre-wrap;
		word-break: break-all;
		user-select: text;
	}

	.bar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
		margin-top: 4px;
		padding-top: 16px;
		border-top: 1px solid var(--border-subtle);
	}
</style>
