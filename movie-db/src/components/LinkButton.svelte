<script lang="ts">
    let {
        label,
        onclick,
    } : {
        label: string;
        onclick: () => void;
    } = $props();

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === 'Enter') {
            onclick();
        }
    }
</script>

<div 
    class="link-button" 
    role="button"

    {onclick}

    tabindex="0"

    onkeydown={handleKeydown}
>
    <span class="button-label">{label}</span>
</div>

<style>
    .link-button {
        font-weight: bold;

        cursor: pointer;

        display: inline-block;
    }

    .button-label {
        color: var(--color-text-secondary);

        border-bottom: 1px solid var(--color-text-secondary);
    }


</style>



