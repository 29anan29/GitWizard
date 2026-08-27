<script lang="ts">
	import { fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { goto } from '$app/navigation';
	import WizardNav from '$lib/components/WizardNav.svelte';
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import alertTriangle from '$lib/assets/icons/alert-triangle.svg?raw';
	import { repoStore, openDialog, refresh as refreshRepo } from '$lib/state/repo.svelte';
	import { git } from '$lib/services/git';
	import type { BranchList } from '$lib/services/git';
	import { friendlyErrorKey } from '$lib/flows/commit/errors';
	import { t } from '$lib/i18n/index.svelte';

	let branches = $state<BranchList>({ local: [], remote: [] });
	let newName = $state('');
	let switchAfter = $state(true);
	let busy = $state(false);
	let formError = $state('');
	let renaming = $state('');
	let renameValue = $state('');
	let armedDelete = $state('');
	let rowError = $state<{ name: string; msg: string } | null>(null);

	const current = $derived(repoStore.info?.branch ?? '');
	const dirty = $derived(repoStore.entries.length > 0);

	async function reload(): Promise<void> {
		if (!repoStore.info) return;
		try {
			const [b] = await Promise.all([git.listBranches(repoStore.info.path), refreshRepo()]);
			branches = b;
		} catch {
			/* noop */
		}
	}

	function errText(e: unknown): string {
		return typeof e === 'string' ? e : e instanceof Error ? e.message : String(e);
	}

	async function create(): Promise<void> {
		if (!repoStore.info || !newName.trim()) return;
		busy = true;
		formError = '';
		try {
			await git.createBranch(repoStore.info.path, newName.trim(), switchAfter);
			newName = '';
			await reload();
		} catch (e) {
			formError = errText(e);
		} finally {
			busy = false;
		}
	}

	async function switchTo(name: string): Promise<void> {
		if (!repoStore.info || name === current) return;
		busy = true;
		rowError = null;
		try {
			await git.checkoutBranch(repoStore.info.path, name);
			await reload();
		} catch (e) {
			rowError = { name, msg: errText(e) };
		} finally {
			busy = false;
		}
	}

	async function doDelete(name: string, force: boolean): Promise<void> {
		if (!repoStore.info) return;
		busy = true;
		rowError = null;
		try {
			await git.deleteBranch(repoStore.info.path, name, force);
			if (armedDelete === name) armedDelete = '';
			await reload();
		} catch (e) {
			const msg = errText(e);
			if (msg.includes('BRANCH_UNMERGED')) {
				armedDelete = name;
			} else {
				rowError = { name, msg };
			}
		} finally {
			busy = false;
		}
	}

	async function submitRename(name: string): Promise<void> {
		if (!repoStore.info || !renameValue.trim()) return;
		busy = true;
		rowError = null;
		try {
			await git.renameBranch(repoStore.info.path, name, renameValue.trim());
			renaming = '';
			renameValue = '';
			await reload();
		} catch (e) {
			rowError = { name, msg: errText(e) };
		} finally {
			busy = false;
		}
	}

	function startRename(name: string): void {
		renaming = name;
		renameValue = name;
		armedDelete = '';
	}

	async function browse(): Promise<void> {
		try {
			await openDialog();
		} catch {
		}
	}

	reload();
</script>

<div class="container page">
	<WizardNav title={t('branch.title')} steps={[t('branch.title')]} current={1} {busy} oncancel={() => goto('/')} />

	{#if !repoStore.info}
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
			<div class="newcard">
				<span class="label">{t('branch.new.label')}</span>
				<form
					class="newrow"
					onsubmit={(e) => {
						e.preventDefault();
						void create();
					}}
				>
					<input
						type="text"
						bind:value={newName}
						placeholder={t('branch.new.placeholder')}
						spellcheck="false"
					/>
					<label class="switchcheck">
						<input type="checkbox" bind:checked={switchAfter} />
						<span class="box" aria-hidden="true"></span>
						{t('branch.new.switch')}
					</label>
					<Button variant="accent" disabled={!newName.trim() || busy} onclick={create}>
						{t('branch.new.create')}
					</Button>
				</form>
				{#if dirty}
					<p class="warnline">
						<Icon svg={alertTriangle} size={12} />
						{t('pull.dirtyWarn')}
						<button class="golink" onclick={() => goto('/flow/commit')}>{t('pull.goCommit')}</button>
					</p>
				{/if}
				{#if formError}
					<p class="errline">{t(friendlyErrorKey(formError))}</p>
				{/if}
				<p class="hint">{t('branch.checkoutRemoteHint', { name: '…' })}</p>
			</div>

			<ul class="rows">
				{#each branches.local as b (b)}
					<li class:current={b === current}>
						{#if renaming === b}
							<div class="renamewrap">
								<input type="text" bind:value={renameValue} spellcheck="false" />
								<button class="mini ok" onclick={() => void submitRename(b)} disabled={busy}>✓</button>
								<button class="mini" onclick={() => (renaming = '')}>✕</button>
							</div>
						{:else}
							<span class="name">{b}</span>
							{#if b === current}
								<em class="tag curtag">{t('branch.current')}</em>
							{/if}
							<div class="ops">
								{#if b !== current}
									<button class="op" onclick={() => void switchTo(b)} disabled={busy}>
										{t('branch.switch')}
									</button>
								{/if}
								<button class="op" onclick={() => startRename(b)} disabled={busy}>
									{t('branch.rename')}
								</button>
								<button
									class="op danger"
									class:armed={armedDelete === b}
									onclick={() =>
										armedDelete === b ? void doDelete(b, true) : (armedDelete = b)}
									disabled={busy}
								>
									{armedDelete === b ? t('branch.confirmDel') : t('branch.del')}
								</button>
							</div>
						{/if}

						{#if armedDelete === b}
							<div class="forcepanel">
								<span>{t('branch.forceTitle')}</span>
								<Button variant="accent" onclick={() => void doDelete(b, true)} disabled={busy}>
									{t('branch.forceDelete')}
								</Button>
							</div>
						{/if}
						{#if rowError?.name === b}
							<p class="rowerr">{t(friendlyErrorKey(rowError.msg))}</p>
						{/if}
					</li>
				{/each}
			</ul>

			<footer class="foot">
				<Button variant="ghost" onclick={() => goto('/')}>{t('run.backHome')}</Button>
			</footer>
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
	}
	.guard p {
		margin: 0 0 10px;
		font-family: var(--font-serif);
		color: var(--text-secondary);
	}
	.actions {
		display: flex;
		gap: 10px;
	}

	.body {
		max-width: 720px;
		margin: 28px auto 0;
		padding: 0 8px;
	}

	.newcard {
		background: var(--surface-400);
		box-shadow: rgba(38, 37, 30, 0.1) 0 0 0 1px inset;
		border-radius: var(--radius-featured);
		padding: 16px 20px 14px;
		display: flex;
		flex-direction: column;
		gap: 10px;
		margin-bottom: 20px;
	}
	.label {
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.048px;
		color: var(--text-secondary);
	}
	.newrow {
		display: flex;
		align-items: center;
		gap: 10px;
		flex-wrap: wrap;
	}
	input[type='text'] {
		flex: 1;
		min-width: 220px;
		background: var(--surface-100);
		border: 1px solid var(--border-subtle);
		border-radius: var(--radius-standard);
		padding: 8px 11px;
		font-family: var(--font-mono);
		font-size: 13px;
		outline: none;
		transition: border-color 150ms ease;
	}
	input:focus {
		border-color: var(--color-accent);
	}
	.switchcheck {
		display: inline-flex;
		align-items: center;
		gap: 7px;
		font-size: 13px;
		cursor: pointer;
		user-select: none;
	}
	.switchcheck input {
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
	.switchcheck input:checked + .box {
		background: var(--color-text);
		border-color: var(--color-text);
	}
	.switchcheck input:checked + .box::after {
		content: '';
		width: 4px;
		height: 8px;
		border-right: 2px solid #fff;
		border-bottom: 2px solid #fff;
		transform: rotate(45deg) translate(-1px, -1px);
	}
	.warnline,
	.errline {
		margin: 0;
		font-size: 12.5px;
		display: flex;
		align-items: center;
		gap: 6px;
		flex-wrap: wrap;
	}
	.warnline {
		color: var(--color-gold);
	}
	.errline {
		color: var(--color-error);
		white-space: pre-wrap;
	}
	.golink {
		border: none;
		background: transparent;
		color: var(--color-accent);
		font-size: 12.5px;
		cursor: pointer;
		padding: 0;
		text-decoration: underline;
	}
	.hint {
		margin: 0;
		font-size: 11px;
		color: var(--text-secondary);
	}

	.rows {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 8px;
	}
	li {
		background: var(--surface-100);
		box-shadow: rgba(38, 37, 30, 0.1) 0 0 0 1px inset;
		border-radius: var(--radius-comfortable);
		padding: 10px 16px;
		display: flex;
		align-items: center;
		gap: 10px;
		flex-wrap: wrap;
		transition:
			box-shadow 180ms ease,
			background-color 150ms ease;
	}
	li.current {
		background: var(--surface-300);
		box-shadow: rgba(38, 37, 30, 0.28) 0 0 0 1px inset;
	}
	.name {
		font-family: var(--font-mono);
		font-size: 13px;
		user-select: text;
	}
	.tag {
		font-style: normal;
		font-size: 10.5px;
		line-height: 1;
		padding: 3px 8px;
		border-radius: var(--radius-pill);
	}
	.curtag {
		color: var(--color-success);
		background: rgba(31, 138, 101, 0.12);
	}
	.ops {
		margin-left: auto;
		display: flex;
		gap: 6px;
	}
	.op {
		background: transparent;
		border: none;
		color: var(--text-tertiary);
		font-size: 12px;
		padding: 4px 9px;
		border-radius: var(--radius-small);
		cursor: pointer;
		transition:
			color 140ms ease,
			background-color 140ms ease;
	}
	.op:hover:not(:disabled) {
		color: var(--color-error);
		background: rgba(38, 37, 30, 0.06);
	}
	.op.danger.armed {
		color: #ffffff;
		background: var(--color-error);
	}
	.op:disabled {
		opacity: 0.45;
		cursor: not-allowed;
	}

	.renamewrap {
		display: flex;
		align-items: center;
		gap: 8px;
		flex: 1;
	}
	.renamewrap input {
		flex: 1;
	}
	.mini {
		width: 26px;
		height: 26px;
		border-radius: var(--radius-small);
		border: none;
		background: rgba(38, 37, 30, 0.06);
		color: var(--text-secondary);
		cursor: pointer;
		font-size: 12px;
	}
	.mini.ok {
		color: var(--color-success);
	}
	.mini:hover {
		background: rgba(38, 37, 30, 0.12);
	}

	.forcepanel {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
		padding: 10px 14px;
		background: rgba(207, 45, 86, 0.07);
		box-shadow: rgba(207, 45, 86, 0.3) 0 0 0 1px inset;
		border-radius: var(--radius-comfortable);
		font-size: 12.5px;
		color: var(--color-error);
	}

	.rowerr {
		width: 100%;
		margin: 0;
		font-size: 12px;
		color: var(--color-error);
		white-space: pre-wrap;
	}

	.foot {
		display: flex;
		justify-content: center;
		margin-top: 24px;
	}
</style>
