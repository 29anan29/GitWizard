<script lang="ts">
	import { fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import WizardNav from '$lib/components/WizardNav.svelte';
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import folderOpen from '$lib/assets/icons/folder-open.svg?raw';
	import { repoStore, openDialog, refresh as refreshRepo } from '$lib/state/repo.svelte';
	import { config, persistConfig } from '$lib/state/config.svelte';
	import { git } from '$lib/services/git';
	import { goto } from '$app/navigation';
	import { t, type Key } from '$lib/i18n/index.svelte';
	import { consoleStore, toggleConsole } from '$lib/state/console.svelte';

	interface RecipeStep {
		label: string;
		run: () => Promise<void>;
	}

	interface Recipe {
		id: string;
		titleKey: Key;
		descKey: Key;
		steps: RecipeStep[];
	}

	let running = $state<string | null>(null);
	let results = $state<Record<string, { ok: boolean; msg: string }>>({});

	function pushLog(kind: 'cmd' | 'out' | 'err', line: string): void {
		consoleStore.lines.push({ kind, line });
	}

	function timestamp(): string {
		const d = new Date();
		const pad = (n: number) => String(n).padStart(2, '0');
		return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}:${pad(d.getSeconds())}`;
	}

	function getRecipes(): Recipe[] {
		const path = repoStore.info?.path ?? '';
		const branch = repoStore.info?.branch ?? 'main';
		const remote = 'origin';

		return [
			{
				id: 'quick-push',
				titleKey: 'recipes.quickPush.title',
				descKey: 'recipes.quickPush.desc',
				steps: [
					{
						label: t('recipes.quickPush.step1'),
						run: async () => {
							pushLog('cmd', 'git add -A');
							await git.stage(path, (await git.status(path)).map((f) => f.path));
							pushLog('out', 'All files staged');
						}
					},
					{
						label: t('recipes.quickPush.step2'),
						run: async () => {
							const msg = `chore: update ${timestamp()}`;
							pushLog('cmd', `git commit -m "${msg}"`);
							await git.commit(path, msg, config.userName, config.userEmail);
							pushLog('out', 'Committed');
						}
					},
					{
						label: t('recipes.quickPush.step3'),
						run: async () => {
							pushLog('cmd', `git push ${remote} ${branch}`);
							await git.push(path, remote, branch, null, config.credentialUsername, null);
							pushLog('out', 'Pushed');
						}
					}
				]
			},
			{
				id: 'sync-remote',
				titleKey: 'recipes.syncRemote.title',
				descKey: 'recipes.syncRemote.desc',
				steps: [
					{
						label: t('recipes.syncRemote.step1'),
						run: async () => {
							pushLog('cmd', `git pull ${remote} ${branch}`);
							const result = await git.pull(path, remote, branch, config.credentialUsername, null);
							if (result.status === 'conflict') {
								pushLog('err', 'Conflicts detected:');
								for (const f of result.conflicts) pushLog('err', `  ${f}`);
								throw new Error(t('recipes.syncRemote.conflict'));
							}
							pushLog('out', `Status: ${result.status}`);
						}
					}
				]
			},
			{
				id: 'clean-branches',
				titleKey: 'recipes.cleanBranches.title',
				descKey: 'recipes.cleanBranches.desc',
				steps: [
					{
						label: t('recipes.cleanBranches.step1'),
						run: async () => {
							const list = await git.listBranches(path);
							const current = repoStore.info?.branch ?? '';
							const merged: string[] = [];
							for (const b of list.local) {
								if (b === current || b === 'main' || b === 'master') continue;
								try {
									await git.deleteBranch(path, b, false);
									pushLog('out', `Deleted: ${b}`);
									merged.push(b);
								} catch {
									pushLog('out', `Skipped (unmerged): ${b}`);
								}
							}
							if (merged.length === 0) {
								pushLog('out', 'No merged branches to clean');
							}
						}
					}
				]
			},
			{
				id: 'new-feature',
				titleKey: 'recipes.newFeature.title',
				descKey: 'recipes.newFeature.desc',
				steps: [
					{
						label: t('recipes.newFeature.step1'),
						run: async () => {
							const base = (await git.listBranches(path)).local.includes('main') ? 'main' : 'master';
							const name = `feat/${timestamp().replace(/[: ]/g, '-')}`;
							pushLog('cmd', `git checkout -b ${name} ${base}`);
							await git.createBranch(path, name, true);
							pushLog('out', `Created and switched to: ${name}`);
							await refreshRepo();
						}
					}
				]
			}
		];
	}

	async function runRecipe(recipe: Recipe): Promise<void> {
		if (!repoStore.info || running) return;
		running = recipe.id;
		results = {};
		consoleStore.open = true;

		try {
			for (const step of recipe.steps) {
				pushLog('cmd', `# ${step.label}`);
				await step.run();
			}
			results[recipe.id] = { ok: true, msg: t('recipes.done') };
			await refreshRepo();
		} catch (e) {
			const msg = typeof e === 'string' ? e : String(e);
			results[recipe.id] = { ok: false, msg };
			pushLog('err', msg);
		} finally {
			running = null;
		}
	}

	async function browse(): Promise<void> {
		try {
			await openDialog();
		} catch { /* noop */ }
	}

	const steps = $derived([t('recipes.title')]);
</script>

<div class="container page">
	<WizardNav
		title={t('recipes.title')}
		{steps}
		current={1}
		oncancel={() => goto('/')}
	/>

	{#if !repoStore.info}
		<section class="guard">
			<h2>{t('guard.noRepo.title')}</h2>
			<p>{t('guard.noRepo.desc')}</p>
			<div class="actions">
				<Button variant="accent" onclick={browse}>
					<span class="btnicon">{@html folderOpen}</span>
					{t('repo.openAction')}
				</Button>
				<Button variant="ghost" onclick={() => goto('/')}>{t('run.backHome')}</Button>
			</div>
		</section>
	{:else}
		<section class="body" in:fly={{ y: 16, duration: 280, delay: 60, easing: cubicOut }}>
			<p class="hint">{t('recipes.hint')}</p>
			<div class="grid">
				{#each getRecipes() as recipe (recipe.id)}
					<div class="recipe-card">
						<div class="recipe-info">
							<span class="recipe-title">{t(recipe.titleKey)}</span>
							<span class="recipe-desc">{t(recipe.descKey)}</span>
							{#if results[recipe.id]}
								<span class="result" class:ok={results[recipe.id].ok} class:err={!results[recipe.id].ok}>
									{results[recipe.id].msg}
								</span>
							{/if}
						</div>
						<Button
							variant={running === recipe.id ? 'surface' : 'accent'}
							disabled={running !== null}
							onclick={() => void runRecipe(recipe)}
						>
							{running === recipe.id ? t('recipes.running') : t('recipes.run')}
						</Button>
					</div>
				{/each}
			</div>
		</section>
	{/if}
</div>

<style>
	.page {
		padding-bottom: 80px;
	}

	.guard {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 10px;
		text-align: center;
		padding: 90px 24px;
	}
	.guard h2 {
		margin: 0;
		font-size: 26px;
		font-weight: 400;
		letter-spacing: -0.325px;
	}
	.guard p {
		margin: 0 0 10px;
		font-family: var(--font-serif);
		font-size: 15.5px;
		color: var(--text-secondary);
	}
	.actions {
		display: flex;
		gap: 10px;
	}
	.btnicon {
		display: inline-flex;
		width: 15px;
		height: 15px;
	}
	.btnicon :global(svg) {
		width: 100%;
		height: 100%;
		stroke: currentColor;
	}

	.body {
		max-width: 760px;
		margin: 28px auto 0;
		padding: 0 8px;
	}

	.hint {
		margin: 0 0 20px;
		font-family: var(--font-serif);
		font-size: 14.5px;
		line-height: 1.5;
		color: var(--text-secondary);
		text-align: center;
	}

	.grid {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.recipe-card {
		display: grid;
		grid-template-columns: 1fr auto;
		gap: 16px;
		align-items: center;
		background: var(--surface-400);
		box-shadow: var(--color-card-border) 0 0 0 1px inset;
		border-radius: var(--radius-featured);
		padding: 18px;
		transition:
			box-shadow 200ms ease,
			background-color 200ms ease;
	}
	.recipe-card:hover {
		background: var(--surface-300);
		box-shadow: var(--shadow-card);
	}

	.recipe-info {
		display: flex;
		flex-direction: column;
		gap: 4px;
		min-width: 0;
	}
	.recipe-title {
		font-size: 16px;
		font-weight: 500;
		letter-spacing: -0.11px;
		line-height: 1.3;
	}
	.recipe-desc {
		font-family: var(--font-serif);
		font-size: 13.5px;
		line-height: 1.35;
		color: var(--text-secondary);
	}
	.result {
		font-size: 12px;
		margin-top: 4px;
	}
	.result.ok {
		color: var(--color-success);
	}
	.result.err {
		color: var(--color-error);
	}
</style>
