<script lang="ts">
	import '../styles/app.css';
	import type { Snippet } from 'svelte';
	import ConsoleDrawer from '$lib/components/ConsoleDrawer.svelte';
	import { attachConsole } from '$lib/state/console.svelte';
	import { initConfig, config } from '$lib/state/config.svelte';
	import { i18n } from '$lib/i18n/index.svelte';

	let { children }: { children: Snippet } = $props();

	initConfig();
	attachConsole();

	$effect(() => {
		if (config.locale && config.locale !== i18n.locale) {
			i18n.locale = config.locale;
		}
	});
	$effect(() => {
		document.documentElement.lang = i18n.locale;
	});
</script>

<main>
	{@render children()}
</main>

<ConsoleDrawer />

<style>
	main {
		min-height: 100vh;
		padding-bottom: 64px;
	}
</style>
