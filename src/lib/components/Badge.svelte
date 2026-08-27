<script lang="ts">
	let { n }: { n: number } = $props();
</script>

<span class="badge">{n > 99 ? '99+' : n}</span>

<style>
	.badge {
		background: var(--color-error);
		color: var(--color-bg);
		font-family: var(--font-mono);
		font-size: 11px;
		line-height: 1;
		padding: 3px 7px;
		border-radius: var(--radius-pill);
		min-width: 14px;
		text-align: center;
	}
</style>
