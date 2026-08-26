<script lang="ts">
	import Button from './Button.svelte';
	import x from '$lib/assets/icons/x.svg?raw';
	import Icon from './Icon.svelte';
	import { config, persistConfig } from '$lib/state/config.svelte';
	import { i18n, t } from '$lib/i18n/index.svelte';

	interface Props {
		onclose: () => void;
	}
	let { onclose }: Props = $props();

	let userName = $state(config.userName ?? '');
	let userEmail = $state(config.userEmail ?? '');
	let autoPush = $state(config.autoPush);

	function setLocale(l: 'zh-CN' | 'en'): void {
		i18n.locale = l;
		config.locale = l;
	}

	async function save(): Promise<void> {
		config.userName = userName.trim() || null;
		config.userEmail = userEmail.trim() || null;
		config.autoPush = autoPush;
		await persistConfig();
		onclose();
	}
</script>

<div class="overlay" role="presentation" onclick={(e) => e.target === e.currentTarget && onclose()}>
	<div class="card" role="dialog" aria-modal="true" aria-label={t('settings.title')}>
		<header>
			<h2>{t('settings.title')}</h2>
			<button class="close" onclick={onclose} aria-label={t('common.close')}>
				<Icon svg={x} size={16} />
			</button>
		</header>

		<div class="field">
			<label for="username">{t('settings.userName')}</label>
			<input id="username" type="text" bind:value={userName} placeholder="Your Name" spellcheck="false" />
		</div>

		<div class="field">
			<label for="useremail">{t('settings.userEmail')}</label>
			<input id="useremail" type="email" bind:value={userEmail} placeholder="you@example.com" spellcheck="false" />
			<span class="hint">{t('settings.identityHint')}</span>
		</div>

		<div class="field">
			<span class="label">{t('settings.locale')}</span>
			<div class="segmented">
				<button class:on={i18n.locale === 'zh-CN'} onclick={() => setLocale('zh-CN')}>中文</button>
				<button class:on={i18n.locale === 'en'} onclick={() => setLocale('en')}>English</button>
			</div>
		</div>

		<label class="checkrow">
			<input type="checkbox" bind:checked={autoPush} />
			<span class="box" aria-hidden="true"></span>
			{t('settings.autoPush')}
		</label>

		<footer>
			<Button variant="ghost" onclick={onclose}>{t('common.cancel')}</Button>
			<Button variant="accent" onclick={save}>{t('common.save')}</Button>
		</footer>
	</div>
</div>

<style>
	.overlay {
		position: fixed;
		inset: 0;
		z-index: 60;
		background: rgba(38, 37, 30, 0.32);
		display: flex;
		align-items: center;
		justify-content: center;
	}
	.card {
		width: 420px;
		max-width: calc(100vw - 48px);
		background: var(--surface-100);
		border-radius: 12px;
		box-shadow: var(--shadow-card);
		padding: 22px 24px 20px;
		display: flex;
		flex-direction: column;
		gap: 16px;
	}
	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	h2 {
		margin: 0;
		font-size: 20px;
		font-weight: 400;
		letter-spacing: -0.11px;
	}
	.close {
		background: transparent;
		border: none;
		color: var(--text-secondary);
		cursor: pointer;
		padding: 4px;
		border-radius: var(--radius-small);
	}
	.close:hover {
		color: var(--color-error);
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}
	label,
	.label {
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.048px;
		color: var(--text-secondary);
	}
	input[type='text'],
	input[type='email'] {
		background: transparent;
		border: 1px solid var(--border-subtle);
		border-radius: var(--radius-standard);
		padding: 8px 10px;
		font-size: 14px;
		outline: none;
		transition: border-color 150ms ease;
	}
	input:focus {
		border-color: var(--color-accent);
	}
	.hint {
		font-size: 11px;
		color: var(--text-secondary);
	}

	.segmented {
		display: inline-flex;
		gap: 4px;
		background: var(--surface-300);
		border-radius: var(--radius-pill);
		padding: 3px;
		align-self: flex-start;
	}
	.segmented button {
		border: none;
		background: transparent;
		color: var(--text-tertiary);
		font-size: 13px;
		padding: 5px 16px;
		border-radius: var(--radius-pill);
		cursor: pointer;
		transition: all 160ms ease;
	}
	.segmented button.on {
		background: var(--surface-100);
		color: var(--color-text);
		box-shadow: var(--shadow-ambient);
	}

	.checkrow {
		display: flex;
		align-items: center;
		gap: 9px;
		font-size: 13.5px;
		text-transform: none;
		letter-spacing: normal;
		font-weight: 400;
		color: var(--color-text);
		cursor: pointer;
		user-select: none;
	}
	.checkrow input {
		position: absolute;
		opacity: 0;
		width: 0;
		height: 0;
	}
	.box {
		width: 15px;
		height: 15px;
		border-radius: var(--radius-medium);
		border: 1px solid var(--border-strong);
		display: inline-flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}
	.checkrow input:checked + .box {
		background: var(--color-text);
		border-color: var(--color-text);
	}
	.checkrow input:checked + .box::after {
		content: '';
		width: 4px;
		height: 8px;
		border-right: 2px solid #fff;
		border-bottom: 2px solid #fff;
		transform: rotate(45deg) translate(-1px, -1px);
	}

	footer {
		display: flex;
		justify-content: flex-end;
		gap: 8px;
		margin-top: 2px;
	}
</style>
