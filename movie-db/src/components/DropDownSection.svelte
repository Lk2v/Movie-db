<script lang="ts">
	import { ChevronDown, ChevronRight } from 'lucide-svelte';

	let { title, children } = $props();

	let open = $state(false);

	function toggleOpen() {
		open = !open;
	}

	function handleKeyDown(event: KeyboardEvent) {
		if (event.key === 'Enter' || event.key === ' ') {
			toggleOpen();
		}
	}
</script>

<div class="drop-down-section-container" class:open>
	<div
		role="button"
		class="drop-down-section-header"
		onclick={toggleOpen}
		onkeydown={handleKeyDown}
		tabindex="0"
	>
		{#if open}
			<ChevronDown />
		{:else}
			<ChevronRight />
		{/if}

		<span class="drop-down-section-title">{title}</span>
	</div>
	<div class="drop-down-section-content">
		{@render children()}

		<div class="mask"></div>
	</div>
</div>

<style>
	.drop-down-section-container {
		border: 2px solid rgba(255, 255, 255, 0.1);
		padding: 24px;
		border-radius: 12px;
	}

	.drop-down-section-header {
		align-items: center;
		flex-direction: row;
		display: flex;

		cursor: pointer;
	}

	.drop-down-section-title {
		margin-left: 12px;
	}

	.drop-down-section-content {
		position: relative;
		height: 200px;
		overflow: hidden;
	}

	.mask {
		position: absolute;
		background: linear-gradient(to bottom, rgba(0, 0, 0, 0), var(--color-background));
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		z-index: 1;
	}

	.drop-down-section-container.open .drop-down-section-content {
		height: auto;
	}

	.drop-down-section-container.open .mask {
		display: none;
	}
</style>
