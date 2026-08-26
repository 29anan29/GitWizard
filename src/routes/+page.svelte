<script lang="ts">
	import RepoHeader from '$lib/components/RepoHeader.svelte';
	import ActionCard from '$lib/components/ActionCard.svelte';
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import folderOpen from '$lib/assets/icons/folder-open.svg?raw';
	import send from '$lib/assets/icons/send.svg?raw';
	import download from '$lib/assets/icons/download.svg?raw';
	import gitBranch from '$lib/assets/icons/git-branch.svg?raw';
	import gitMerge from '$lib/assets/icons/git-merge.svg?raw';
	import history from '$lib/assets/icons/history.svg?raw';
	import bolt from '$lib/assets/icons/bolt.svg?raw';
	import { repoStore, openDialog, openPath } from '$lib/state/repo.svelte';
import { config } from '$lib/state/config.svelte';
	import { goto } from '$app/navigation';
	import { t, type Key } from '$lib/i18n/index.svelte';

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
		act?: () => void;
	}

	const cardDefs: CardDef[] = [
		{
			icon: send,
			titleKey: 'cards.commit.title',
			descKey: 'cards.commit.desc',
			enabled: true,
			act: () => goto('/flow/commit')
		},
		{ icon: download, titleKey: 'cards.pull.title', descKey: 'cards.pull.desc', enabled: false },
		{ icon: gitBranch, titleKey: 'cards.branch.title', descKey: 'cards.branch.desc', enabled: false },
		{ icon: gitMerge, titleKey: 'cards.merge.title', descKey: 'cards.merge.desc', enabled: false },
		{ icon: history, titleKey: 'cards.reset.title', descKey: 'cards.reset.desc', enabled: false },
		{ icon: bolt, titleKey: 'cards.quick.title', descKey: 'cards.quick.desc', enabled: false }
	];
</script>

<div class="container">
	<RepoHeader />

	<section class="hero">
		<h1>{t('home.hero.title')}</h1>
		<p>{t('home.hero.sub')}</p>
	</section>

	{#if !repoStore.info}
		<section class="openpanel">
			<span class="bigicon"><Icon svg={folderOpen} size={38} /></span>
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
				<ActionCard
					icon={c.icon}
					titleKey={c.titleKey}
					descKey={c.descKey}
					badge={c.enabled ? repoStore.info?.dirtyCount ?? 0 : 0}
					enabled={c.enabled}
					onclick={c.act}
				/>
			{/each}
		</section>
	{/if}

	<footer class="foot">
		<span>{t('version.line')}</span>
	</footer>
</div>

<style>
	.container {
		padding-bottom: 24px;
	}

	.hero {
		padding: 56px 0 36px;
		max-width: 640px;
	}
	h1 {
		margin: 0 0 10px;
		font-size: clamp(32px, 4.4vw, 46px);
		line-height: 1.12;
		letter-spacing: -0.03em;
		font-weight: 400;
	}
	.hero p {
		margin: 0;
		font-family: var(--font-serif);
		font-size: 17.28px;
		line-height: 1.5;
		color: var(--text-secondary);
	}

	.openpanel {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 12px;
		text-align: center;
		background: var(--surface-400);
		box-shadow: rgba(38, 37, 30, 0.1) 0 0 0 1px inset;
		border-radius: var(--radius-featured);
		padding: 56px 32px;
		margin-bottom: 40px;
	}
	.bigicon {
		width: 68px;
		height: 68px;
		border-radius: var(--radius-featured);
		background: var(--surface-500);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-text);
		margin-bottom: 6px;
	}
	.openpanel h2 {
		margin: 0;
		font-size: 22px;
		font-weight: 400;
		letter-spacing: -0.11px;
	}
	.openpanel p {
		margin: 0 0 8px;
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
		transition: color 150ms ease;
	}
	.recent:hover:not(:disabled) {
		color: var(--color-error);
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
