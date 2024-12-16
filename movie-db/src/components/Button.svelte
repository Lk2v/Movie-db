<script lang="ts">
	let {
		children,
		onclick,
        mode = 'filled',
	}: {
		children: () => any;
		onclick: () => void;

        mode?: 'filled' | 'outlined';
	} = $props();
</script>

<button 
    {onclick}

    class:filled={mode === 'filled'}
    class:outlined={mode === 'outlined'}
>
	{@render children()}
</button>

<style>
	button {
        font-weight: bold;

		background: none;
		border: none;
		border-radius: 8px;
		padding: 10px 24px;
		cursor: pointer;

        transition: transform 0.2s;
	}

    button.filled {
        background: var(--color-on-background);
		color: var(--color-background);
    }

    button.outlined {
        background: transparent;
        color: var(--color-on-background);
    }

    button:hover {
        transform: scale(1.05);
    }

    button:active {
        transform: scale(0.95);
    }
</style>
