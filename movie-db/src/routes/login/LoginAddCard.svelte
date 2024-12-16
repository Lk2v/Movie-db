<script lang="ts">
	import { Plus } from 'lucide-svelte';
	import LoginFrameCard from './LoginFrameCard.svelte';
	let {
		onclick
	}: {
		onclick: () => void;
	} = $props();
</script>

<LoginFrameCard {onclick}>
	<div class="content">
		<div class="add">

		<Plus/>
		</div>
	</div>
</LoginFrameCard>

<style>
	.content {
		width: 100%;
		height: 100%;

		background: rgba(255, 255, 255, 0.02);

		transition: background 0.2s;

		justify-content: center;
		align-items: center;
		display: flex;
	}

	.content:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.content:hover .add {
		background: #0056b3;
		transform: scale(1.1);
	}

	.add {
		/* ios blue */
		background: #007aff;
		border-radius: 50%;
		padding: 4px;

		transition: all 0.2s;
	}
</style>