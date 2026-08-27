<script lang="ts">
	import { fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { goto } from '$app/navigation';
	import WizardNav from '$lib/components/WizardNav.svelte';
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import circleCheck from '$lib/assets/icons/circle-check.svg?raw';
	import loader from '$lib/assets/icons/loader-2.svg?raw';
	import shieldLock from '$lib/assets/icons/shield-lock.svg?raw';
	import { repoStore, openDialog } from '$lib/state/repo.svelte';
	import { git } from '$lib/services/git';
	import { t } from '$lib/i18n/index.svelte';

	interface SshKey {
		name: string;
		type: string;
		path: string;
		isDefault: boolean;
	}

	let keys = $state<SshKey[]>([]);
	let busy = $state(false);
	let loadError = $state('');
	let showGenerate = $state(false);
	let genName = $state('');
	let genType = $state<'ed25519' | 'rsa' | 'ecdsa'>('ed25519');
	let genComment = $state('');
	let generatedPub = $state('');
	let copied = $state(false);

	const info = $derived(repoStore.info);

	async function loadKeys(): Promise<void> {
		busy = true;
		loadError = '';
		try {
			keys = await git.listSshKeys();
		} catch (e) {
			loadError = typeof e === 'string' ? e : String(e);
		} finally {
			busy = false;
		}
	}

	async function generateKey(): Promise<void> {
		if (!genName.trim()) return;
		busy = true;
		loadError = '';
		generatedPub = '';
		try {
			const pub = await git.generateSshKey(genName.trim(), genType, genComment.trim() || undefined);
			generatedPub = pub;
			await loadKeys();
		} catch (e) {
			loadError = typeof e === 'string' ? e : String(e);
		} finally {
			busy = false;
		}
	}

	async function copyPub(): Promise<void> {
		try {
			await navigator.clipboard.writeText(generatedPub);
			copied = true;
			setTimeout(() => (copied = false), 2000);
		} catch {
			/* noop */
		}
	}

	async function browse(): Promise<void> {
		try { await openDialog(); } catch { /* noop */ }
	}

	loadKeys();
</script>

<div class="container page">
	<WizardNav title={t('ssh.title')} steps={[t('ssh.title')]} current={1} {busy} oncancel={() => goto('/')} />

	{#if !info}
		<section class="guard">
			<h2>{t('guard.noRepo.title')}</h2>
			<p>{t('guard.noRepo.desc')}</p>
			<div class="actions">
				<Button variant="accent" onclick={browse}>{t('repo.openAction')}</Button>
				<Button variant="ghost" onclick={() => goto('/')}>{t('run.backHome')}</Button>
			</div>
		</section>
	{:else}
		<section class="body" in:fly={{ y: 14, duration: 240, easing: cubicOut }}>
			{#if busy && !showGenerate}
				<section class="center"><Icon svg={loader} size={28} /></section>
			{:else}
				<div class="keylist-card">
					<span class="label">{t('ssh.existingKeys')}</span>
					{#if keys.length === 0}
						<p class="empty">{t('ssh.noKeys')}</p>
					{:else}
						<ul class="keylist">
							{#each keys as k (k.path)}
								<li>
									<span class="key-icon"><Icon svg={shieldLock} size={14} /></span>
									<div class="key-info">
										<span class="key-name">{k.name}</span>
										<span class="key-meta">{k.type} · {k.path}</span>
									</div>
									{#if k.isDefault}
										<span class="default-tag">{t('ssh.default')}</span>
									{/if}
								</li>
							{/each}
						</ul>
					{/if}
				</div>

				{#if !showGenerate}
					<Button variant="accent" onclick={() => (showGenerate = true)}>
						{t('ssh.generate')}
					</Button>
				{:else}
					<div class="gen-card" in:fly={{ y: 10, duration: 200, easing: cubicOut }}>
						<span class="label">{t('ssh.genTitle')}</span>

						<div class="form-row">
							<span class="form-label">{t('ssh.genName')}</span>
							<input type="text" bind:value={genName} placeholder="id_ed25519_work" spellcheck="false" />
						</div>

						<div class="form-row">
							<span class="form-label">{t('ssh.genType')}</span>
							<div class="type-btns">
								{#each (['ed25519', 'rsa', 'ecdsa'] as const) as tp}
									<button class="type-btn" class:active={genType === tp} onclick={() => (genType = tp)}>
										{tp}
									</button>
								{/each}
							</div>
						</div>

						<div class="form-row">
							<span class="form-label">{t('ssh.genComment')}</span>
							<input type="text" bind:value={genComment} placeholder={t('ssh.genCommentPh')} spellcheck="false" />
						</div>

						<div class="gen-actions">
							<Button variant="ghost" onclick={() => { showGenerate = false; generatedPub = ''; }}>{t('common.cancel')}</Button>
							<Button variant="accent" disabled={!genName.trim() || busy} onclick={() => void generateKey()}>
								{t('ssh.genExecute')}
							</Button>
						</div>
					</div>
				{/if}

				{#if generatedPub}
					<div class="pub-card" in:fly={{ y: 10, duration: 200, easing: cubicOut }}>
						<span class="label">{t('ssh.pubKey')}</span>
						<pre class="pub-key">{generatedPub}</pre>
						<Button variant="accent" onclick={() => void copyPub()}>
							{copied ? t('ssh.copied') : t('ssh.copy')}
						</Button>
					</div>
				{/if}

				{#if loadError}
					<p class="errline">{loadError}</p>
				{/if}

				<footer class="foot">
					<Button variant="ghost" onclick={() => goto('/')}>{t('run.backHome')}</Button>
				</footer>
			{/if}
		</section>
	{/if}
</div>

<style>
	.page { padding-bottom: 80px; }
	.guard { display: flex; flex-direction: column; align-items: center; gap: 10px; text-align: center; padding: 90px 24px; }
	.guard h2 { margin: 0; font-size: 26px; font-weight: 400; }
	.guard p { margin: 0 0 10px; font-family: var(--font-serif); color: var(--text-secondary); }
	.actions { display: flex; gap: 10px; }

	.center { display: flex; flex-direction: column; align-items: center; gap: 14px; padding: 90px 24px; color: var(--text-secondary); }
	.center :global(.icon) { animation: spin 0.9s linear infinite; color: var(--color-accent); }
	@keyframes spin { to { transform: rotate(360deg); } }

	.body { max-width: 640px; margin: 28px auto 0; padding: 0 8px; }

	.keylist-card { background: var(--surface-400); box-shadow: rgba(38, 37, 30, 0.1) 0 0 0 1px inset; border-radius: var(--radius-featured); padding: 16px 20px 14px; display: flex; flex-direction: column; gap: 10px; margin-bottom: 16px; }
	.label { font-size: 11px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.048px; color: var(--text-secondary); }
	.empty { margin: 0; font-size: 13px; color: var(--text-secondary); font-family: var(--font-serif); }

	.keylist { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 4px; }
	.keylist li { display: flex; align-items: center; gap: 10px; padding: 8px 12px; background: var(--surface-100); border-radius: var(--radius-comfortable); }
	.key-icon { color: var(--color-accent); flex-shrink: 0; display: flex; }
	.key-info { display: flex; flex-direction: column; gap: 1px; flex: 1; min-width: 0; }
	.key-name { font-family: var(--font-mono); font-size: 13px; }
	.key-meta { font-size: 11px; color: var(--text-tertiary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
	.default-tag { font-size: 10.5px; padding: 2px 8px; border-radius: var(--radius-pill); color: var(--color-success); background: rgba(31, 138, 101, 0.1); flex-shrink: 0; }

	.gen-card { background: var(--surface-400); box-shadow: rgba(38, 37, 30, 0.1) 0 0 0 1px inset; border-radius: var(--radius-featured); padding: 16px 20px 14px; display: flex; flex-direction: column; gap: 12px; margin-bottom: 16px; }
	.form-row { display: flex; align-items: center; gap: 12px; }
	.form-label { font-size: 12.5px; color: var(--text-secondary); min-width: 80px; flex-shrink: 0; }
	input[type='text'] { flex: 1; background: var(--surface-100); border: 1px solid var(--border-subtle); border-radius: var(--radius-standard); padding: 8px 11px; font-family: var(--font-mono); font-size: 13px; outline: none; transition: border-color 150ms ease; }
	input:focus { border-color: var(--color-accent); }
	.type-btns { display: flex; gap: 6px; }
	.type-btn { background: var(--surface-100); border: 1px solid var(--border-subtle); border-radius: var(--radius-standard); padding: 6px 14px; font-size: 12.5px; cursor: pointer; transition: all 120ms ease; }
	.type-btn.active { border-color: var(--color-accent); color: var(--color-accent); background: rgba(245, 78, 0, 0.04); }
	.gen-actions { display: flex; gap: 10px; justify-content: flex-end; margin-top: 4px; }

	.pub-card { background: var(--surface-400); box-shadow: rgba(38, 37, 30, 0.1) 0 0 0 1px inset; border-radius: var(--radius-featured); padding: 16px 20px 14px; display: flex; flex-direction: column; gap: 10px; margin-bottom: 16px; }
	.pub-key { margin: 0; padding: 12px 16px; background: var(--surface-100); border-radius: var(--radius-comfortable); font-family: var(--font-mono); font-size: 11.5px; line-height: 1.6; white-space: pre-wrap; word-break: break-all; user-select: text; color: var(--text-secondary); overflow-x: auto; }

	.errline { margin: 0 0 12px; font-size: 12.5px; color: var(--color-error); white-space: pre-wrap; }
	.foot { display: flex; justify-content: center; margin-top: 24px; }
</style>
