<script lang="ts">
	import { fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { onMount, onDestroy } from 'svelte';
	import Icon from './Icon.svelte';
	import x from '$lib/assets/icons/x.svg?raw';
	import { t, type Key } from '$lib/i18n/index.svelte';

	interface Props {
		onclose: () => void;
	}
	let { onclose }: Props = $props();

	let open = $state(true);

	interface Shortcut {
		keys: string[];
		descKey: Key;
	}

	const shortcuts: Shortcut[] = [
		{ keys: ['Ctrl', 'Z'], descKey: 'shortcut.undo' },
		{ keys: ['Ctrl', 'Shift', 'Z'], descKey: 'shortcut.redo' },
		{ keys: ['Ctrl', 'K'], descKey: 'shortcut.commandPalette' },
		{ keys: ['Ctrl', 'Enter'], descKey: 'shortcut.execute' },
		{ keys: ['Esc'], descKey: 'shortcut.cancel' },
		{ keys: ['?'], descKey: 'shortcut.help' },
		{ keys: ['1'], descKey: 'shortcut.commit' },
		{ keys: ['2'], descKey: 'shortcut.pull' },
		{ keys: ['3'], descKey: 'shortcut.branch' }
	];

	function handleClose(): void {
		open = false;
		onclose();
	}

	function handleKeydown(e: KeyboardEvent): void {
		if (e.key === '?' && !e.ctrlKey && !e.metaKey && !e.altKey) {
			const target = e.target as HTMLElement;
			if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA') return;
			e.preventDefault();
			if (open) {
				handleClose();
			} else {
				open = true;
			}
		}
		if (e.key === 'Escape' && open) {
			handleClose();
		}
	}

	onMount(() => {
		window.addEventListener('keydown', handleKeydown);
	});

	onDestroy(() => {
		window.removeEventListener('keydown', handleKeydown);
	});
</script>

{#if open}
	<div class="backdrop" onclick={handleClose} role="presentation" in:fly={{ opacity: 0, duration: 150 }} out:fly={{ opacity: 0, duration: 100 }}></div>
	<div class="modal" in:fly={{ y: 14, duration: 240, easing: cubicOut }} out:fly={{ y: 10, duration: 150 }}>
		<div class="head">
			<h2>{t('shortcut.title')}</h2>
			<button class="close-btn" onclick={handleClose}>
				<Icon svg={x} size={15} />
			</button>
		</div>

		<ul class="list">
			{#each shortcuts as s (s.descKey)}
				<li>
					<span class="desc">{t(s.descKey)}</span>
					<span class="keys">
						{#each s.keys as k, i}
							{k}{#if i < s.keys.length - 1}<span class="sep">+</span>{/if}
						{/each}
					</span>
				</li>
			{/each}
		</ul>

		<p class="hint">{t('shortcut.hint')}</p>
	</div>
{/if}

<style>
	.backdrop {
		position: fixed;
		inset: 0;
		z-index: 60;
		background: rgba(0, 0, 0, 0.2);
	}

	.modal {
		position: fixed;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		z-index: 70;
		width: 400px;
		max-width: 90vw;
		background: var(--surface-400);
		box-shadow: var(--shadow-card), var(--color-card-border) 0 0 0 1px inset;
		border-radius: 14px;
		padding: 24px;
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.head { display: flex; align-items: center; justify-content: space-between; }
	h2 { margin: 0; font-size: 18px; font-weight: 400; letter-spacing: -0.2px; }
	.close-btn {
		background: transparent;
		border: none;
		color: var(--text-tertiary);
		cursor: pointer;
		padding: 4px;
		border-radius: var(--radius-small);
		display: flex;
		align-items: center;
	}
	.close-btn:hover { color: var(--color-error); }

	.list { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 6px; }
	li { display: flex; align-items: center; justify-content: space-between; padding: 8px 12px; background: var(--surface-100); border-radius: var(--radius-comfortable); }
	.desc { font-size: 13px; color: var(--text-secondary); }
	.keys { display: flex; align-items: center; gap: 2px; font-family: var(--font-mono); font-size: 12px; color: var(--color-text); }
	.sep { color: var(--text-tertiary); margin: 0 1px; }

	.hint { margin: 0; font-size: 11.5px; color: var(--text-tertiary); text-align: center; }
</style>
