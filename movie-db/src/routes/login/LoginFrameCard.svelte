<script lang="ts">
	let {
		children,
		onclick
	}: {
		children: () => any;
		onclick: () => void;
	} = $props();

	function keydownHandler(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			onclick();
		}
	}
</script>

<div role="button" class="login-choice" {onclick} tabindex="0" onkeydown={keydownHandler}>
	{@render children()}
</div>

<style>
	.login-choice {
		width: 150px;
		height: 200px;

		border: 2px solid rgba(255, 255, 255, 0.1);
		border-radius: 8px;

		overflow: hidden;
        cursor: pointer;

		transition: all 0.4s;
	}

	.login-choice:hover {
		border-color: rgba(255, 255, 255, 0.2);
		transform: scale(1.05);
	}
</style>