<script lang="ts">
	let {
		children,
		onclick,
	}: {
		children: () => any;
		onclick: () => void;
	} = $props();
</script>

<button 
    {onclick}
>
	{@render children()}
</button>

<style>
	button {
        font-weight: bold;

		background: none;
		border: none;
		border-radius: 50%;
		padding: 10px ;
		cursor: pointer;

        transition: transform 0.2s;
	}

    button:hover {
        background: rgba(225,225,225,0.1);
        transform: scale(1.05);
    }

    button:active {
        transform: scale(0.95);
    }
</style>
