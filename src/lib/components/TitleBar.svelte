<script lang="ts">
	import Icon from './Icon.svelte';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import appIcon from '$lib/assets/icons/wand.svg?raw';
	import minus from '$lib/assets/icons/minus.svg?raw';
	import square from '$lib/assets/icons/square.svg?raw';
	import x from '$lib/assets/icons/x.svg?raw';

	const win = getCurrentWindow();

	function minimize(): void {
		void win.minimize();
	}
	function toggleMaximize(): void {
		void win.toggleMaximize();
	}
	function close(): void {
		void win.close();
	}
</script>

<header class="titlebar" data-tauri-drag-region>
	<div class="brand" data-tauri-drag-region>
		<span class="logo">{@html appIcon}</span>
		<span class="name">GitWizard</span>
	</div>

	<div class="controls">
		<button class="ctrl" onclick={minimize} aria-label="minimize">
			<Icon svg={minus} size={14} />
		</button>
		<button class="ctrl" onclick={toggleMaximize} aria-label="maximize">
			<Icon svg={square} size={11} />
		</button>
		<button class="ctrl closebtn" onclick={close} aria-label="close">
			<Icon svg={x} size={14} />
		</button>
	</div>
</header>

<style>
	.titlebar {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		height: 40px;
		z-index: 90;
		display: flex;
		align-items: center;
		background: var(--color-titlebar-bg);
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
		border-bottom: 1px solid var(--border-subtle);
		user-select: none;
	}

	.brand {
		display: flex;
		align-items: center;
		gap: 8px;
		padding-left: 14px;
		height: 100%;
	}
	.logo {
		display: inline-flex;
		width: 16px;
		height: 16px;
	}
	.logo :global(svg) {
		width: 100%;
		height: 100%;
	}
	.name {
		font-size: 12.5px;
		letter-spacing: -0.11px;
		color: var(--text-tertiary);
	}

	.controls {
		margin-left: auto;
		display: flex;
		height: 100%;
	}
	.ctrl {
		width: 46px;
		height: 100%;
		border: none;
		background: transparent;
		color: var(--color-titlebar-text);
		display: inline-flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		transition:
			background-color 140ms ease,
			color 140ms ease;
	}
	.ctrl:hover {
		background: var(--color-titlebar-hover);
		color: var(--color-text);
	}
	.closebtn:hover {
		background: var(--color-error);
		color: #ffffff;
	}
	.ctrl:focus-visible {
		outline: none;
		box-shadow: var(--shadow-focus);
	}
</style>
