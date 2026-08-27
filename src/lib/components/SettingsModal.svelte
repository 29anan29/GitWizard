<script lang="ts">
	import { fly, fade } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { getVersion } from '@tauri-apps/api/app';
	import Button from './Button.svelte';
	import Icon from './Icon.svelte';
	import x from '$lib/assets/icons/x.svg?raw';
	import UpdateModal from './UpdateModal.svelte';
	import { config, persistConfig } from '$lib/state/config.svelte';
	import { credStore } from '$lib/state/creds.svelte';
	import { git } from '$lib/services/git';
	import { i18n, t } from '$lib/i18n/index.svelte';
	import type { Update as UpdaterUpdate } from '@tauri-apps/plugin-updater';

	interface Props {
		onclose: () => void;
	}
	let { onclose }: Props = $props();

	let userName = $state(config.userName ?? '');
	let userEmail = $state(config.userEmail ?? '');
	let autoPush = $state(config.autoPush);
	let updateProxy = $state(config.updateProxy ?? '');
	let autoCheck = $state(config.autoCheckUpdate);
	let credUser = $state(config.credentialUsername ?? '');
	let credToken = $state('');
	let credRemember = $state(true);
	let credSaved = $state(false);
	let credFail = $state('');

	let appVersion = $state('');
	getVersion().then((v) => (appVersion = v)).catch(() => {});

	let checking = $state(false);
	let checkResult = $state<UpdaterUpdate | null>(null);
	let checkError = $state('');
	let showUpdateModal = $state(false);

	async function save(): Promise<void> {
		config.userName = userName.trim() || null;
		config.userEmail = userEmail.trim() || null;
		config.autoPush = autoPush;
		config.updateProxy = updateProxy.trim() || null;
		config.autoCheckUpdate = autoCheck;
		config.credentialUsername = credUser.trim() || null;
		await persistConfig();
		onclose();
	}

	async function saveCreds(): Promise<void> {
		const u = credUser.trim();
		config.credentialUsername = u || null;
		await persistConfig();
		if (!u || !credToken) return;
		try {
			if (credRemember) {
				await git.saveCredential(u, credToken);
			}
			credStore.sessionToken = credToken;
			credStore.keyringUnavailable = false;
			credSaved = true;
			credFail = '';
			setTimeout(() => (credSaved = false), 2500);
		} catch (e) {
			credStore.sessionToken = credToken;
			credStore.keyringUnavailable = true;
			credFail = String(e);
		}
	}

	function setLocale(l: 'zh-CN' | 'en'): void {
		i18n.locale = l;
		config.locale = l;
	}

	async function checkUpdate(): Promise<void> {
		checking = true;
		checkResult = null;
		checkError = '';
		try {
			const upd = await git.checkUpdater();
			if (upd) {
				checkResult = upd;
			} else {
				checkResult = null;
			}
		} catch (e) {
			checkError = typeof e === 'string' ? e : String(e);
		} finally {
			checking = false;
		}
	}
</script>

<div
	class="overlay"
	role="presentation"
	transition:fade={{ duration: 150 }}
	onclick={(e) => e.target === e.currentTarget && onclose()}
>
	<div
		class="card"
		role="dialog"
		aria-modal="true"
		aria-label={t('settings.title')}
		transition:fly={{ y: 18, duration: 220, easing: cubicOut }}
	>
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

		<div class="divider"></div>

		<div class="updatesec">
			<span class="label">{t('settings.cred.title')}</span>
			<div class="credgrid">
				<div class="field">
					<label for="credu">{t('settings.cred.user')}</label>
					<input id="credu" type="text" bind:value={credUser} placeholder="username" spellcheck="false" autocomplete="off" />
				</div>
				<div class="field">
					<label for="credt">{t('settings.cred.token')}</label>
					<input
						id="credt"
						type="password"
						bind:value={credToken}
						placeholder={t('settings.cred.tokenPh')}
						spellcheck="false"
						autocomplete="off"
					/>
				</div>
			</div>
			<span class="hint">{t('settings.cred.hint')}</span>
			<label class="checkrow small">
				<input type="checkbox" bind:checked={credRemember} />
				<span class="box" aria-hidden="true"></span>
				{t('settings.cred.remember')}
			</label>
			{#if credSaved}
				<span class="verstate ok">{t('settings.cred.saved')}</span>
			{/if}
			{#if credFail || credStore.keyringUnavailable}
				<p class="checkfail">{t('settings.cred.keyringFail')}</p>
			{/if}
			<Button variant="ghost" onclick={saveCreds}>{t('settings.cred.save')}</Button>
		</div>

		<div class="divider"></div>

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

		<div class="divider"></div>

		<div class="updatesec">
			<span class="label">{t('settings.update.title')}</span>
			<div class="verrow">
				<span class="vermono">v{appVersion || '…'}</span>
				{#if checking}
					<span class="verstate">{t('settings.update.checking')}</span>
				{:else if checkResult}
					<span class="verstate new">
						{t('settings.update.new')} v{checkResult.version ?? ''}
					</span>
					<Button variant="pill" onclick={() => (showUpdateModal = true)}>
						{t('update.download')}
					</Button>
				{:else if !checkError}
					<span class="verstate ok">{t('settings.update.uptodate')}</span>
				{/if}
			</div>

			<label class="checkrow small">
				<input type="checkbox" bind:checked={autoCheck} />
				<span class="box" aria-hidden="true"></span>
				{t('settings.update.auto')}
			</label>

			<div class="field">
				<label for="proxy">{t('settings.update.proxyLabel')}</label>
				<input
					id="proxy"
					type="text"
					bind:value={updateProxy}
					placeholder="http://127.0.0.1:7890"
					spellcheck="false"
				/>
			</div>

			{#if checkError}
				<p class="checkfail">{t('settings.update.fail')}: {checkError}</p>
			{/if}

			<Button variant="ghost" onclick={checkUpdate} disabled={checking}>
				{t('settings.update.check')}
			</Button>
		</div>

		<footer>
			<Button variant="ghost" onclick={onclose}>{t('common.cancel')}</Button>
			<Button variant="accent" onclick={save}>{t('common.save')}</Button>
		</footer>
	</div>
</div>

{#if showUpdateModal && checkResult}
	<UpdateModal update={checkResult} onclose={() => (showUpdateModal = false)} />
{/if}

<style>
	.overlay {
		position: fixed;
		inset: 0;
		z-index: 60;
		background: var(--color-overlay);
		backdrop-filter: blur(5px);
		-webkit-backdrop-filter: blur(5px);
		display: flex;
		align-items: center;
		justify-content: center;
	}
	.card {
		width: 420px;
		max-width: calc(100vw - 48px);
		background: var(--surface-100);
		border-radius: 14px;
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

	.divider {
		height: 1px;
		background: var(--border-subtle);
		margin: 2px 0;
	}

	.updatesec {
		display: flex;
		flex-direction: column;
		gap: 10px;
		align-items: flex-start;
	}

	.credgrid {
		display: grid;
		grid-template-columns: 1fr 1.4fr;
		gap: 10px;
		width: 100%;
	}
	.verrow {
		display: flex;
		align-items: center;
		gap: 10px;
		flex-wrap: wrap;
	}
	.vermono {
		font-family: var(--font-mono);
		font-size: 12px;
		background: var(--surface-300);
		padding: 4px 11px;
		border-radius: var(--radius-pill);
		color: var(--color-text);
	}
	.verstate {
		font-size: 12.5px;
		color: var(--text-secondary);
	}
	.verstate.new {
		color: var(--color-accent);
		font-weight: 600;
	}
	.verstate.ok {
		color: var(--color-success);
	}
	.checkrow.small {
		font-size: 13px;
	}
	.checkfail {
		margin: 0;
		font-family: var(--font-mono);
		font-size: 11.5px;
		line-height: 1.6;
		color: var(--color-error);
		word-break: break-all;
	}
</style>
