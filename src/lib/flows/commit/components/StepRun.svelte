<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import loader from '$lib/assets/icons/loader-2.svg?raw';
	import circleCheck from '$lib/assets/icons/circle-check.svg?raw';
	import alertTriangle from '$lib/assets/icons/alert-triangle.svg?raw';
	import { consoleStore } from '$lib/state/console.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import { friendlyErrorKey } from '../errors';
	import type { Wizard } from '../state.svelte';

	let { wizard, onhome, onnew }: { wizard: Wizard; onhome: () => void; onnew: () => void } =
		$props();

	const s = $derived(wizard.state);
	const pushing = $derived(s.phase === 'running' && s.oid !== '');
	const percent = $derived(consoleStore.pushPercent);
	const friendly = $derived(s.errorKind === 'none' ? null : t(friendlyErrorKey(s.errorMsg)));
</script>

<div class="wrap">
	{#if s.phase === 'running'}
		<div class="running">
			<Icon svg={loader} size={30} />
			<p>{pushing ? t('run.pushing') : t('run.running')}</p>
			{#if pushing && percent >= 0}
				<div class="track">
					<div class="fill" style="width:{percent}%"></div>
				</div>
				<span class="pct">{percent}%</span>
			{/if}
		</div>
	{:else if s.phase === 'success'}
		<div class="panel ok">
			<span class="halo"><Icon svg={circleCheck} size={44} /></span>
			<h2>{t('run.successTitle')}</h2>
			{#if s.oid}
				<span class="oid">[{s.oid.slice(0, 7)}]</span>
			{/if}
			<span class="chip" class:ok={s.pushed}>
				{s.pushed ? t('run.pushed') : t('run.notPushed')}
			</span>
			<div class="actions">
				<Button variant="ghost" onclick={onnew}>{t('run.newCommit')}</Button>
				<Button variant="ghost" onclick={onhome}>{t("run.backHome")}')}</Button>
			</div>
		</div>
	{:else if s.phase === 'error'}
		<div class="panel bad">
			<span class="halo"><Icon svg={alertTriangle} size={44} /></span>
			<h2>{s.oid ? t('run.partialTitle') : t('run.failedTitle')}</h2>
			{#if s.oid}
				<span class="oid">[{s.oid.slice(0, 7)}]</span>
			{/if}
			{#if friendly}
				<p class="friendly">{friendly}</p>
			{/if}
			<details class="rawwrap">
				<summary>{t('err.detail')}</summary>
				<pre class="errbox">{s.errorMsg}</pre>
			</details>
			{#if s.errorKind === 'nonff'}
				<p class="hint">{t('run.nonffHint')}</p>
				<Button variant="accent" onclick={() => goto('/flow/pull')}>
					{t('home.ahead.action')}
				</Button>
			{/if}
			<div class="actions">
				<Button variant="ghost" onclick={() => void wizard.retry()}>{t('common.retry')}</Button>
				<Button variant="ghost" onclick={onhome}>{t("run.backHome")}')}</Button>
			</div>
		</div>
	{/if}
</div>

<style>
	.wrap {
		min-height: 320px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.running {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 14px;
		color: var(--text-secondary);
	}
	.running :global(.icon) {
		animation: spin 0.9s linear infinite;
		color: var(--color-accent);
	}
	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
	.running p {
		margin: 0;
		font-size: 15px;
	}
	.track {
		width: 260px;
		height: 5px;
		border-radius: var(--radius-pill);
		background: var(--surface-500);
		overflow: hidden;
	}
	.fill {
		height: 100%;
		background: var(--color-accent);
		border-radius: var(--radius-pill);
		transition: width 200ms ease;
	}
	.pct {
		font-family: var(--font-mono);
		font-size: 11px;
	}

	.panel {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 12px;
		text-align: center;
		max-width: 520px;
		padding: 32px 36px;
		background: var(--surface-100);
		box-shadow:
			rgba(38, 37, 30, 0.1) 0 0 0 1px inset,
			var(--shadow-card);
		border-radius: 14px;
	}
	.halo {
		width: 74px;
		height: 74px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
	}
	.panel.ok .halo {
		background: rgba(31, 138, 101, 0.1);
		animation: ring-pulse 1.8s ease-out 0.15s both;
	}
	.panel.ok .halo :global(.icon) {
		color: var(--color-success);
		animation: pop-in 0.4s cubic-bezier(0.22, 1, 0.36, 1) both;
	}
	.panel.bad .halo {
		background: rgba(207, 45, 86, 0.09);
	}
	.panel.bad .halo :global(.icon) {
		color: var(--color-error);
		animation: pop-in 0.4s cubic-bezier(0.22, 1, 0.36, 1) both;
	}
	h2 {
		margin: 4px 0 0;
		font-size: 26px;
		font-weight: 400;
		letter-spacing: -0.325px;
	}
	.oid {
		font-family: var(--font-mono);
		font-size: 12px;
		color: var(--text-tertiary);
		background: var(--surface-400);
		padding: 4px 10px;
		border-radius: var(--radius-pill);
	}
	.chip {
		font-size: 12.5px;
		color: var(--color-gold);
		background: rgba(192, 133, 50, 0.12);
		padding: 5px 13px;
		border-radius: var(--radius-pill);
	}
	.chip.ok {
		color: var(--color-success);
		background: rgba(31, 138, 101, 0.12);
	}

	.friendly {
		margin: 2px 0 0;
		font-family: var(--font-serif);
		font-size: 15px;
		line-height: 1.65;
		color: var(--color-text);
		max-width: 460px;
		text-align: left;
		background: var(--surface-300);
		border-radius: var(--radius-comfortable);
		padding: 12px 16px;
	}

	.rawwrap {
		width: 100%;
		max-width: 460px;
		text-align: left;
	}
	.rawwrap summary {
		font-size: 11.5px;
		color: var(--text-tertiary);
		cursor: pointer;
		user-select: none;
		padding: 2px 0 6px;
	}
	.rawwrap summary:hover {
		color: var(--color-error);
	}

	.errbox {
		margin: 4px 0 0;
		padding: 12px 16px;
		background: var(--surface-300);
		border-radius: var(--radius-comfortable);
		font-family: var(--font-mono);
		font-size: 12px;
		line-height: 1.67;
		color: var(--color-error);
		white-space: pre-wrap;
		word-break: break-all;
		max-width: 100%;
		user-select: text;
	}
	.hint {
		margin: 0;
		font-family: var(--font-serif);
		font-size: 15px;
		line-height: 1.6;
		color: var(--text-secondary);
	}

	.actions {
		display: flex;
		gap: 10px;
		margin-top: 10px;
		flex-wrap: wrap;
		justify-content: center;
	}
</style>
