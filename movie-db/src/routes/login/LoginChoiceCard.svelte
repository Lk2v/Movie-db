<script lang="ts">
	import { type SqlUserChoice } from '$api/login';
	import { User } from 'lucide-svelte';
	import LoginFrameCard from './LoginFrameCard.svelte';
	let {
		choice,
		onclick
	}: {
		choice: SqlUserChoice;
		onclick: () => void;
	} = $props();
</script>

<LoginFrameCard {onclick}>
	<div class="login-choice">
		<div class="login-header">
			<div class="avatar">
				<User size={48}/>
			</div>
		</div>
		<div class="login-body">
			<span class="user-name">{choice.title}</span>
		</div>
	</div>
</LoginFrameCard>

<style>
	.login-choice {
		width: 100%;
		height: 100%;

		flex-direction: column;
		display: flex;
	}

	.login-header {
		flex-grow: 1;

		align-items: center;
		justify-content: center;
		display: flex;
	}

	.login-body {
		margin: 10px;
	}

	.user-name {
		text-align: center;
	}
</style>