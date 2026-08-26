<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import RepoHeader from '$lib/components/RepoHeader.svelte';
	import ActionCard from '$lib/components/ActionCard.svelte';
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import folderOpen from '$lib/assets/icons/folder-open.svg?raw';
	import send from '$lib/assets/icons/send.svg?raw';
	import gitBranch from '$lib/assets/icons/git-branch.svg?raw';
	import download from '$lib/assets/icons/download.svg?raw';
	import gitMerge from '$lib/assets/icons/git-merge.svg?raw';
	import history from '$lib/assets/icons/history.svg?raw';
	import bolt from '$lib/assets/icons/bolt.svg?raw';
	import wand from '$lib/assets/icons/wand.svg?raw';
	import { repoStore, openDialog, openPath } from '$lib/state/repo.svelte';
	import { config } from '$lib/state/config.svelte';
	import { getVersion } from '@tauri-apps/api/app';
	import { goto } from '$app/navigation';
	import { t, type Key } from '$lib/i18n/index.svelte';

	let appVersion = $state('');
	getVersion().then((v) => (appVersion = v)).catch(() => {});

	let opening = $state(false);
	let openError = $state('');

	async function browse(): Promise<void> {
		openError = '';
		opening = true;
		try {
			await openDialog();
		} catch {
			openError = t('repo.invalid');
		} finally {
			opening = false;
		}
	}

	async function pickRecent(path: string): Promise<void> {
		openError = '';
		opening = true;
		try {
			await openPath(path);
		} catch {
			openError = t('repo.invalid');
		} finally {
			opening = false;
		}
	}

	interface CardDef {
		icon: string;
		titleKey: Key;
		descKey: Key;
		enabled: boolean;
		badge: () => number;
		act?: () => void;
	}

	const cardDefs: CardDef[] = [
		{
			icon: send,
			titleKey: 'cards.commit.title',
			descKey: 'cards.commit.desc',
			enabled: true,
			badge: () => repoStore.info?.dirtyCount ?? 0,
			act: () => goto('/flow/commit')
		},
		{
			icon: download,
			titleKey: 'cards.pull.title',
			descKey: 'cards.pull.desc',
			enabled: true,
			badge: () => repoStore.info?.behind ?? 0,
			act: () => goto('/flow/pull')
		},
		{ icon: gitBranch, titleKey: 'cards.branch.title', descKey: 'cards.branch.desc', enabled: true, badge: () => 0, act: () => goto('/flow/branch') },
		{ icon: gitMerge, titleKey: 'cards.merge.title', descKey: 'cards.merge.desc', enabled: false, badge: () => 0 },
		{ icon: history, titleKey: 'cards.reset.title', descKey: 'cards.reset.desc', enabled: false, badge: () => 0 },
		{ icon: bolt, titleKey: 'cards.quick.title', descKey: 'cards.quick.desc', enabled: false, badge: () => 0 }
	];

	const info = $derived(repoStore.info);
</script>

<div class="container">
	<RepoHeader />

	<section class="hero" in:fade={{ duration: 300 }}>
		<h1>{t('home.hero.title')}</h1>
		<p>{t('home.hero.sub')}</p>

		{#if info}
			<div class="statusrow" in:fly={{ y: 10, duration: 320, delay: 120 }}>
				<span class="spill branch">
					<Icon svg={gitBranch} size={12} />
					{info.branch ?? 'HEAD'}
				</span>
				{#if info.dirtyCount > 0}
					<span class="spill warn">{t('status.dirty', { n: info.dirtyCount })}</span>
				{:else}
					<span class="spill ok">✓</span>
				{/if}
				{#if info.ahead > 0}
					<span class="spill gold">{t('status.ahead', { n: info.ahead })}</span>
				{/if}
				{#if info.behind > 0}
					<span class="spill bad">{t('status.behind', { n: info.behind })}</span>
				{/if}
				{#if info.remoteUrl}
					<span class="spill remote">{info.remoteUrl.replace(/^https?:\/\//, '')}</span>
				{/if}
			</div>

			{#if info.ahead > 0}
				<div class="pushbanner" in:fly={{ y: 10, duration: 320, delay: 180 }}>
					<div class="pushtexts">
						<strong>{t('home.ahead.title', { n: info.ahead })}</strong>
						<span>{t('home.ahead.desc')}</span>
					</div>
					<Button variant="accent" onclick={() => goto('/flow/commit')}>
						{t('home.ahead.action')}
					</Button>
				</div>
			{/if}
		{/if}
	</section>

	{#if !info}
		<section class="openpanel" in:fly={{ y: 14, duration: 340, delay: 80 }}>
			<div class="art">
				<span class="tile main"><Icon svg={folderOpen} size={30} /></span>
				<span class="tile mini"><Icon svg={send} size={16} /></span>
				<span class="spark"><Icon svg={wand} size={15} /></span>
			</div>
			<h2>{t('repo.empty.title')}</h2>
			<p>{t('repo.empty.desc')}</p>

			{#if opening}
				<span class="opening">…</span>
			{:else}
				<Button variant="accent" onclick={browse}>{t('repo.openAction')}</Button>
			{/if}

			{#if openError}
				<p class="err">{openError}</p>
			{/if}

			{#if config.recentRepos.length > 0}
				<div class="recents">
					<span class="micro">{t('repo.recent')}</span>
					<div class="chips">
						{#each config.recentRepos as p (p)}
							<button class="recent" onclick={() => pickRecent(p)} disabled={opening}>
								{p.split('/').pop() || p}
							</button>
						{/each}
					</div>
				</div>
			{/if}
		</section>
	{:else}
		<section class="cards">
			{#each cardDefs as c, i (i)}
				<div in:fly={{ y: 16, duration: 320, delay: 60 + i * 45 }}>
				<ActionCard
					icon={c.icon}
					titleKey={c.titleKey}
					descKey={c.descKey}
					badge={c.badge()}
					enabled={c.enabled}
					onclick={c.act}
				/>
				</div>
			{/each}
		</section>
	{/if}

	<footer class="foot">
		<span>{t('version.line', { v: appVersion || '…' })}</span>
	</footer>
</div>

<style>
	.container {
		padding-bottom: 24px;
	}

	.hero {
		padding: 58px 0 38px;
		max-width: 680px;
	}
	h1 {
		margin: 0 0 12px;
		font-size: clamp(34px, 4.6vw, 50px);
		line-height: 1.1;
		letter-spacing: -0.033em;
		font-weight: 400;
		background: linear-gradient(105deg, var(--color-text) 55%, rgba(245, 78, 0, 0.85));
		-webkit-background-clip: text;
		background-clip: text;
	}
	.hero p {
		margin: 0;
		font-family: var(--font-serif);
		font-size: 17.28px;
		line-height: 1.6;
		color: var(--text-secondary);
		max-width: 520px;
	}

	.statusrow {
		display: flex;
		align-items: center;
		gap: 8px;
		flex-wrap: wrap;
		margin-top: 20px;
	}
	.spill {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		font-size: 12px;
		line-height: 1;
		padding: 7px 13px;
		border-radius: var(--radius-pill);
		background: var(--surface-400);
		box-shadow: rgba(38, 37, 30, 0.08) 0 0 0 1px inset;
		color: var(--text-tertiary);
	}
	.spill.branch {
		font-family: var(--font-mono);
		font-size: 11px;
		color: var(--color-text);
		background: var(--surface-300);
	}
	.spill.warn {
		color: var(--color-error);
		background: rgba(207, 45, 86, 0.09);
		box-shadow: rgba(207, 45, 86, 0.25) 0 0 0 1px inset;
	}
	.spill.ok {
		color: var(--color-success);
		background: rgba(31, 138, 101, 0.09);
		box-shadow: rgba(31, 138, 101, 0.25) 0 0 0 1px inset;
	}
	.spill.gold {
		color: var(--color-gold);
		background: rgba(192, 133, 50, 0.1);
		box-shadow: rgba(192, 133, 50, 0.3) 0 0 0 1px inset;
	}
	.spill.bad {
		color: var(--color-error);
		background: rgba(207, 45, 86, 0.09);
		box-shadow: rgba(207, 45, 86, 0.25) 0 0 0 1px inset;
	}
	.spill.remote {
		font-family: var(--font-mono);
		font-size: 11px;
		max-width: 260px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.pushbanner {
		margin-top: 18px;
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 16px 20px;
		background: rgba(245, 78, 0, 0.06);
		box-shadow: rgba(245, 78, 0, 0.28) 0 0 0 1px inset;
		border-radius: var(--radius-comfortable);
		max-width: 620px;
	}
	.pushtexts {
		display: flex;
		flex-direction: column;
		gap: 3px;
		flex: 1;
	}
	.pushtexts strong {
		font-size: 14.5px;
		font-weight: 600;
		color: var(--color-accent);
	}
	.pushtexts span {
		font-family: var(--font-serif);
		font-size: 13.5px;
		line-height: 1.5;
		color: var(--text-secondary);
	}

	.openpanel {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 12px;
		text-align: center;
		background:
			radial-gradient(420px 180px at 50% 0%, rgba(245, 78, 0, 0.05), transparent 70%),
			var(--surface-400);
		box-shadow: rgba(38, 37, 30, 0.1) 0 0 0 1px inset;
		border-radius: var(--radius-featured);
		padding: 52px 32px 56px;
		margin-bottom: 40px;
	}

	.art {
		position: relative;
		width: 104px;
		height: 88px;
		margin-bottom: 10px;
		animation: float-y 5s ease-in-out infinite;
	}
	.tile {
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: var(--radius-featured);
	}
	.tile.main {
		position: absolute;
		left: 4px;
		top: 8px;
		width: 68px;
		height: 68px;
		background: linear-gradient(140deg, var(--surface-100), var(--surface-500));
		box-shadow:
			rgba(38, 37, 30, 0.12) 0 0 0 1px inset,
			var(--shadow-card);
		color: var(--color-text);
	}
	.tile.mini {
		position: absolute;
		right: 8px;
		bottom: 2px;
		width: 42px;
		height: 42px;
		background: var(--color-bg);
		box-shadow:
			rgba(38, 37, 30, 0.14) 0 0 0 1px inset,
			var(--shadow-ambient);
		transform: rotate(-7deg);
		color: var(--color-accent);
	}
	.spark {
		position: absolute;
		right: 6px;
		top: -2px;
		color: var(--color-gold);
		animation: pop-in 0.6s cubic-bezier(0.22, 1, 0.36, 1) both;
	}

	.openpanel h2 {
		margin: 0;
		font-size: 22px;
		font-weight: 400;
		letter-spacing: -0.11px;
	}
	.openpanel p {
		margin: 0 0 10px;
		font-family: var(--font-serif);
		font-size: 15.5px;
		color: var(--text-secondary);
		max-width: 420px;
	}
	.opening {
		font-size: 14px;
		color: var(--text-secondary);
	}
	.err {
		margin: 0;
		font-size: 13px;
		color: var(--color-error);
	}

	.recents {
		display: flex;
		flex-direction: column;
		gap: 8px;
		margin-top: 18px;
	}
	.micro {
		font-size: 11px;
		text-transform: uppercase;
		letter-spacing: 0.048px;
		color: var(--text-secondary);
	}
	.chips {
		display: flex;
		gap: 7px;
		flex-wrap: wrap;
		justify-content: center;
		max-width: 560px;
	}
	.recent {
		background: var(--surface-300);
		border: none;
		color: var(--text-tertiary);
		font-family: var(--font-mono);
		font-size: 11px;
		padding: 5px 13px;
		border-radius: var(--radius-pill);
		cursor: pointer;
		transition:
			color 150ms ease,
			transform 150ms ease;
	}
	.recent:hover:not(:disabled) {
		color: var(--color-error);
		transform: translateY(-1px);
	}

	.cards {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(290px, 1fr));
		gap: 14px;
		margin-bottom: 48px;
	}

	.foot {
		display: flex;
		justify-content: center;
		padding-top: 8px;
	}
	.foot span {
		font-size: 11px;
		color: rgba(38, 37, 30, 0.35);
	}
</style>
