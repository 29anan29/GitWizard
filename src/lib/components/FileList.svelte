<script lang="ts">
	import type { FileEntry } from '$lib/services/git';
	import { t } from '$lib/i18n/index.svelte';
	import type { SvelteSet } from 'svelte/reactivity';

	interface Props {
		entries: FileEntry[];
		selected: SvelteSet<string>;
		toggle: (path: string) => void;
		setGroup: (paths: string[], on: boolean) => void;
	}

	let { entries, selected, toggle, setGroup }: Props = $props();

	type Kind = 'added' | 'modified' | 'deleted';
	type Item = { path: string; kind: Kind; staged: boolean; conflicted: boolean };

	const GROUP_DEFS: { key: Kind; dot: string; labelKey: 'files.group.added' | 'files.group.modified' | 'files.group.deleted' }[] = [
		{ key: 'added', dot: 'var(--timeline-grep)', labelKey: 'files.group.added' },
		{ key: 'modified', dot: 'var(--timeline-read)', labelKey: 'files.group.modified' },
		{ key: 'deleted', dot: 'var(--color-error)', labelKey: 'files.group.deleted' }
	];

	const groups = $derived.by(() => {
		const items: Item[] = entries
			.filter((e) => e.worktree || e.staged)
			.map((e) => ({
				path: e.path,
				kind: (e.worktree ?? e.staged) as Kind,
				staged: !!e.staged,
				conflicted: e.conflicted
			}));
		return GROUP_DEFS.map((g) => ({
			...g,
			label: t(g.labelKey),
			items: items.filter((i) => i.kind === g.key).sort((a, b) => a.path.localeCompare(b.path))
		})).filter((g) => g.items.length > 0);
	});
</script>

<div class="list">
	{#each groups as g}
		<section>
			<header class="grouphead">
				<span class="dot" style="background:{g.dot}"></span>
				<span class="grouplabel">{g.label}</span>
				<span class="count">{g.items.length}</span>
				<button
					class="allbtn"
					onclick={() =>
						setGroup(
							g.items.map((i) => i.path),
							!g.items.every((i) => selected.has(i.path))
						)}
				>
					{t('files.selectAll')}
				</button>
			</header>
			<ul>
				{#each g.items as item (item.path)}
					<li>
						<label class:checked={selected.has(item.path)}>
							<input type="checkbox" checked={selected.has(item.path)} onchange={() => toggle(item.path)} />
							<span class="checkmark" aria-hidden="true"></span>
							<span class="path">{item.path}</span>
							<span class="tags">
								{#if item.conflicted}<em class="tag conflict">{t('files.conflictTag')}</em>{/if}
								{#if item.staged}<em class="tag staged">{t('files.stagedTag')}</em>{/if}
							</span>
						</label>
					</li>
				{/each}
			</ul>
		</section>
	{/each}
</div>

<style>
	.list {
		display: flex;
		flex-direction: column;
		gap: 22px;
	}
	.grouphead {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 6px;
	}
	.dot {
		width: 8px;
		height: 8px;
		border-radius: var(--radius-pill);
	}
	.grouplabel {
		font-size: 13px;
		font-weight: 600;
	}
	.count {
		font-family: var(--font-mono);
		font-size: 11px;
		color: var(--text-secondary);
		background: var(--surface-400);
		border-radius: var(--radius-pill);
		padding: 1px 7px;
	}
	.allbtn {
		margin-left: auto;
		background: transparent;
		border: none;
		font-size: 12px;
		color: var(--text-tertiary);
		cursor: pointer;
		padding: 3px 8px;
		border-radius: var(--radius-medium);
		transition: color 150ms ease;
	}
	.allbtn:hover {
		color: var(--color-error);
	}

	ul {
		list-style: none;
		margin: 0;
		padding: 0;
		border: 1px solid var(--border-subtle);
		border-radius: var(--radius-comfortable);
		overflow: hidden;
	}
	label {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 9px 14px;
		cursor: pointer;
		background: var(--surface-100);
		position: relative;
		transition:
			background-color 120ms ease,
			box-shadow 160ms ease;
	}
	label::before {
		content: '';
		position: absolute;
		left: 0;
		top: 6px;
		bottom: 6px;
		width: 3px;
		border-radius: var(--radius-pill);
		background: var(--color-accent);
		opacity: 0;
		transform: scaleY(0.4);
		transition:
			opacity 150ms ease,
			transform 180ms cubic-bezier(0.22, 1, 0.36, 1);
	}
	label.checked {
		background: var(--surface-200);
	}
	label.checked::before {
		opacity: 1;
		transform: scaleY(1);
	}
	li + li label {
		border-top: 1px solid var(--border-subtle);
	}
	label:hover {
		background: var(--surface-300);
	}

	input[type='checkbox'] {
		position: absolute;
		opacity: 0;
		width: 0;
		height: 0;
	}
	.checkmark {
		width: 15px;
		height: 15px;
		flex-shrink: 0;
		border-radius: var(--radius-medium);
		border: 1px solid var(--border-strong);
		display: inline-flex;
		align-items: center;
		justify-content: center;
		transition:
			background-color 120ms ease,
			border-color 120ms ease;
	}
	input:checked + .checkmark {
		background: var(--color-text);
		border-color: var(--color-text);
		animation: pop 0.22s cubic-bezier(0.22, 1, 0.36, 1);
	}
	@keyframes pop {
		0% {
			transform: scale(0.7);
		}
		60% {
			transform: scale(1.12);
		}
		100% {
			transform: scale(1);
		}
	}
	input:checked + .checkmark::after {
		content: '';
		width: 4px;
		height: 8px;
		border-right: 2px solid #ffffff;
		border-bottom: 2px solid #ffffff;
		transform: rotate(45deg) translate(-1px, -1px);
	}
	input:focus-visible + .checkmark {
		box-shadow: var(--shadow-focus);
	}

	.path {
		font-family: var(--font-mono);
		font-size: 12px;
		line-height: 1.5;
		word-break: break-all;
		user-select: text;
	}
	.tags {
		margin-left: auto;
		display: flex;
		gap: 5px;
	}
	.tag {
		font-style: normal;
		font-size: 10.5px;
		line-height: 1;
		padding: 3px 7px;
		border-radius: var(--radius-pill);
	}
	.tag.staged {
		color: var(--color-success);
		background: rgba(31, 138, 101, 0.12);
	}
	.tag.conflict {
		color: var(--color-error);
		background: rgba(207, 45, 86, 0.12);
	}
</style>
