<script lang="ts">
	import Header from './Header.svelte';
	let { children } = $props();
</script>

<div class="app">
	<Header />

	<main>
		{@render children()}
	</main>
</div>

<style>
	.app {
		position: fixed;

		top: 0;
		left: 0;
		right: 0;
		bottom: 0;


		flex-direction: column;
		display: flex;
	}

	main {
		position: relative;
		flex-grow: 1;
		display: flex;

		overflow-y: auto;
		overflow-x: hidden;
	}
</style>
