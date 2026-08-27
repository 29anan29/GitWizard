<script lang="ts">
	import '../styles/app.css';
	import type { Snippet } from 'svelte';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import ConsoleDrawer from '$lib/components/ConsoleDrawer.svelte';
	import CredentialPrompt from '$lib/components/CredentialPrompt.svelte';
	import UndoSidebar from '$lib/components/UndoSidebar.svelte';
	import DarkToggle from '$lib/components/DarkToggle.svelte';
	import ShortcutHelp from '$lib/components/ShortcutHelp.svelte';
	import { attachConsole } from '$lib/state/console.svelte';
	import { initConfig, config } from '$lib/state/config.svelte';
	import { startupUpdateCheck } from '$lib/state/update.svelte';
	import { i18n } from '$lib/i18n/index.svelte';
	import { repoStore } from '$lib/state/repo.svelte';

	let { children }: { children: Snippet } = $props();

	let ready = $state(false);
	let showShortcuts = $state(false);

	initConfig().then(() => {
		ready = true;
		applyTheme();
	});
	attachConsole();

	function applyTheme(): void {
		const root = document.documentElement;
		if (config.theme === 'dark') {
			root.setAttribute('data-theme', 'dark');
		} else if (config.theme === 'light') {
			root.removeAttribute('data-theme');
		} else {
			// system preference
			if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
				root.setAttribute('data-theme', 'dark');
			} else {
				root.removeAttribute('data-theme');
			}
		}
	}

	$effect(() => {
		if (!ready) return;
		if (config.locale && config.locale !== i18n.locale) {
			i18n.locale = config.locale;
		}
		applyTheme();
		startupUpdateCheck(config.updateProxy ?? null, config.autoCheckUpdate);
	});
	$effect(() => {
		document.documentElement.lang = i18n.locale;
	});

	function handleKeydown(e: KeyboardEvent): void {
		if (e.key === '?' && !e.ctrlKey && !e.metaKey && !e.altKey) {
			const tag = (e.target as HTMLElement)?.tagName;
			if (tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT') return;
			e.preventDefault();
			showShortcuts = !showShortcuts;
		}
		if (e.key === 'Escape' && showShortcuts) {
			showShortcuts = false;
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<TitleBar />

<main>
	{#if repoStore.info}
		<UndoSidebar />
	{/if}
	{@render children()}
	<DarkToggle />
</main>

<ConsoleDrawer />

<CredentialPrompt />

{#if showShortcuts}
	<ShortcutHelp onclose={() => (showShortcuts = false)} />
{/if}

<style>
	main {
		min-height: 100vh;
		padding-top: 40px;
		padding-bottom: 64px;
	}
</style>
