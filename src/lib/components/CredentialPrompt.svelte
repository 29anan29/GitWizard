<script lang="ts">
	import { fly, fade } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import Button from './Button.svelte';
	import Icon from './Icon.svelte';
	import shieldLock from '$lib/assets/icons/shield-lock.svg?raw';
	import x from '$lib/assets/icons/x.svg?raw';
	import { authPromptStore, settleAuth } from '$lib/state/authPrompt.svelte';
	import { credStore } from '$lib/state/creds.svelte';
	import { config, persistConfig } from '$lib/state/config.svelte';
	import { git } from '$lib/services/git';
	import { t } from '$lib/i18n/index.svelte';

	let username = $state('');
	let token = $state('');
	let remember = $state(true);
	let saved = $state(false);
	let fail = $state('');

	$effect(() => {
		if (authPromptStore.open) {
			username = config.credentialUsername ?? config.userName ?? '';
			token = '';
			saved = false;
			fail = '';
		}
	});

	async function confirm(): Promise<void> {
		const u = username.trim();
		if (!u || !token.trim()) {
			fail = t('auth.required');
			return;
		}
		config.credentialUsername = u;
		await persistConfig();
		try {
			if (remember) {
				await git.saveCredential(u, token);
			}
			credStore.sessionToken = token;
			credStore.keyringUnavailable = false;
			settleAuth(true);
		} catch {
			credStore.sessionToken = token;
			credStore.keyringUnavailable = true;
			settleAuth(true);
		}
	}

	function cancel(): void {
		settleAuth(false);
	}
</script>

{#if authPromptStore.open}
	<div
		class="overlay"
		role="presentation"
		transition:fade={{ duration: 140 }}
		onclick={(e) => e.target === e.currentTarget && cancel()}
	>
		<div
			class="card"
			role="dialog"
			aria-modal="true"
			aria-label={t('auth.title')}
			transition:fly={{ y: 16, duration: 200, easing: cubicOut }}
		>
			<header>
				<span class="icon"><Icon svg={shieldLock} size={20} /></span>
				<h2>{t('auth.title')}</h2>
				<button class="close" onclick={cancel} aria-label={t('common.close')}>
					<Icon svg={x} size={15} />
				</button>
			</header>

			<p class="desc">{t('auth.desc')}</p>

			<div class="field">
				<label for="ap-user">{t('settings.cred.user')}</label>
				<input id="ap-user" type="text" bind:value={username} placeholder="username" spellcheck="false" autocomplete="off" />
			</div>

			<div class="field">
				<label for="ap-token">{t('settings.cred.token')}</label>
				<input
					id="ap-token"
					type="password"
					bind:value={token}
					placeholder={t('settings.cred.tokenPh')}
					spellcheck="false"
					autocomplete="off"
				/>
			</div>

			<span class="hint">{t('settings.cred.hint')}</span>

			<label class="checkrow">
				<input type="checkbox" bind:checked={remember} />
				<span class="box" aria-hidden="true"></span>
				{t('settings.cred.remember')}
			</label>

			{#if fail}
				<p class="fail">{fail}</p>
			{:else if !remember}
				<p class="warn">{t('settings.cred.keyringFail')}</p>
			{/if}

			<footer>
				<Button variant="ghost" onclick={cancel}>{t('auth.cancel')}</Button>
				<Button variant="accent" onclick={confirm}>{t('auth.confirm')}</Button>
			</footer>
		</div>
	</div>
{/if}

<style>
	.overlay {
		position: fixed;
		inset: 0;
		z-index: 70;
		background: rgba(38, 37, 30, 0.32);
		backdrop-filter: blur(5px);
		-webkit-backdrop-filter: blur(5px);
		display: flex;
		align-items: center;
		justify-content: center;
	}
	.card {
		width: 400px;
		max-width: calc(100vw - 48px);
		background: var(--surface-100);
		border-radius: 14px;
		box-shadow: var(--shadow-card);
		padding: 22px 24px 20px;
		display: flex;
		flex-direction: column;
		gap: 13px;
	}

	header {
		display: flex;
		align-items: center;
		gap: 10px;
	}
	header .icon {
		width: 32px;
		height: 32px;
		border-radius: var(--radius-comfortable);
		background: rgba(245, 78, 0, 0.1);
		color: var(--color-accent);
		display: inline-flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}
	h2 {
		margin: 0;
		font-size: 19px;
		font-weight: 400;
		letter-spacing: -0.11px;
		flex: 1;
	}
	.close {
		background: transparent;
		border: none;
		color: var(--text-secondary);
		cursor: pointer;
		padding: 4px;
		border-radius: var(--radius-small);
		display: inline-flex;
	}
	.close:hover {
		color: var(--color-error);
	}

	.desc {
		margin: 0;
		font-family: var(--font-serif);
		font-size: 14.5px;
		line-height: 1.5;
		color: var(--text-secondary);
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}
	label {
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.048px;
		color: var(--text-secondary);
	}
	input[type='text'],
	input[type='password'] {
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

	.checkrow {
		display: flex;
		align-items: center;
		gap: 9px;
		font-size: 13px;
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

	.fail {
		margin: 0;
		font-size: 12.5px;
		color: var(--color-error);
	}
	.warn {
		margin: 0;
		font-size: 12px;
		color: var(--color-gold);
	}

	footer {
		display: flex;
		justify-content: flex-end;
		gap: 8px;
		margin-top: 2px;
	}
</style>
