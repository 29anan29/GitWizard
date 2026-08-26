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

	<div class="autorow">
		<div class="autotext">
			<span class="autolabel">{t('strategy.autoPush')}</span>
			{#if wizard.state.autoPush}
				<span class="autosub">git push origin {repoStore.info?.branch ?? ''}</span>
			{/if}
		</div>
		<button
			class="switch"
			class:on={wizard.state.autoPush}
			onclick={() => (wizard.state.autoPush = !wizard.state.autoPush)}
			role="switch"
			aria-checked={wizard.state.autoPush}
			aria-label={t('strategy.autoPush')}
		>
			<span class="knob"></span>
		</button>
	</div>

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
			<div class="term">
				<div class="termhead"><i></i><i></i><i></i><span>sh</span></div>
				<pre>{cmds.join('\n')}</pre>
			</div>
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

	.autorow {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 16px;
		padding: 14px 18px;
		background: var(--surface-300);
		box-shadow: rgba(38, 37, 30, 0.08) 0 0 0 1px inset;
		border-radius: var(--radius-comfortable);
	}
	.autotext {
		display: flex;
		flex-direction: column;
		gap: 3px;
	}
	.autolabel {
		font-size: 14.5px;
	}
	.autosub {
		font-family: var(--font-mono);
		font-size: 11px;
		color: var(--color-accent);
	}

	.switch {
		position: relative;
		width: 42px;
		height: 24px;
		flex-shrink: 0;
		border-radius: var(--radius-pill);
		background: var(--surface-500);
		box-shadow: rgba(38, 37, 30, 0.15) 0 0 0 1px inset;
		border: none;
		cursor: pointer;
		transition:
			background-color 200ms ease,
			box-shadow 200ms ease;
	}
	.switch.on {
		background: var(--color-accent);
		box-shadow: 0 2px 10px rgba(245, 78, 0, 0.35);
	}
	.knob {
		position: absolute;
		top: 3px;
		left: 3px;
		width: 18px;
		height: 18px;
		border-radius: 50%;
		background: #ffffff;
		box-shadow: 0 1.5px 4px rgba(0, 0, 0, 0.28);
		transition: left 200ms cubic-bezier(0.22, 1, 0.36, 1);
	}
	.switch.on .knob {
		left: 21px;
	}
	.switch:focus-visible {
		outline: none;
		box-shadow: var(--shadow-focus);
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
	.term {
		margin-top: 4px;
		background: #191813;
		border-radius: var(--radius-comfortable);
		overflow: hidden;
		box-shadow: var(--shadow-card);
	}
	.termhead {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 9px 12px 7px;
		border-bottom: 1px solid rgba(242, 241, 237, 0.08);
	}
	.termhead i {
		width: 9px;
		height: 9px;
		border-radius: 50%;
		display: inline-block;
	}
	.termhead i:nth-child(1) {
		background: #e2556f;
	}
	.termhead i:nth-child(2) {
		background: #d9a44a;
	}
	.termhead i:nth-child(3) {
		background: #57a583;
	}
	.termhead span {
		margin-left: 8px;
		font-family: var(--font-mono);
		font-size: 10px;
		color: #8f8c7e;
	}
	pre {
		margin: 0;
		padding: 13px 16px 15px;
		background: transparent;
		color: #cfccc0;
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
