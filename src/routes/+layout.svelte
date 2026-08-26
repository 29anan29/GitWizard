<script lang="ts">
	import '../styles/app.css';
	import type { Snippet } from 'svelte';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import ConsoleDrawer from '$lib/components/ConsoleDrawer.svelte';
	import { attachConsole } from '$lib/state/console.svelte';
	import { initConfig, config } from '$lib/state/config.svelte';
	import { startupUpdateCheck } from '$lib/state/update.svelte';
	import { i18n } from '$lib/i18n/index.svelte';

	let { children }: { children: Snippet } = $props();

	let ready = $state(false);

	initConfig().then(() => (ready = true));
	attachConsole();

	$effect(() => {
		if (!ready) return;
		if (config.locale && config.locale !== i18n.locale) {
			i18n.locale = config.locale;
		}
		startupUpdateCheck(config.updateProxy ?? null, config.autoCheckUpdate);
	});
	$effect(() => {
		document.documentElement.lang = i18n.locale;
	});
</script>

<TitleBar />

<main>
	{@render children()}
</main>

<ConsoleDrawer />

<style>
	main {
		min-height: 100vh;
		padding-top: 40px;
		padding-bottom: 64px;
	}
</style>
