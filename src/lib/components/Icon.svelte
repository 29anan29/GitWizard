<script lang="ts">
	let { svg, size = 16 }: { svg: string; size?: number } = $props();
</script>

<span class="icon" style="--icon-size:{size}px">{@html svg}</span>

<style>
	.icon {
		display: inline-flex;
		width: var(--icon-size);
		height: var(--icon-size);
		flex-shrink: 0;
	}
	.icon :global(svg) {
		width: 100%;
		height: 100%;
		stroke-width: 1.7;
	}
</style>
