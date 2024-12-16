<script lang="ts">
    let {
        label,
        selected,
        onclick
    }: {
        label: string;
        selected: boolean;
        onclick: () => void;
    } = $props();

    function handleKeyDown(event: KeyboardEvent) {
        if (event.key === 'Enter') {
            onclick();
        }
    }
</script>

<button 
    class="search-filter-container"
    class:selected={selected}

    onclick={onclick}
    onkeydown={handleKeyDown}

>
    <div class="search-filter-label">
        {label}
    </div>
</button>

<style>
    .search-filter-container {
        background-color: var(--color-background-tertiary);
        border: none;
        outline: none;
        border-radius: 8px;
        padding: 8px 12px;

        display: flex;
        align-items: center;
        gap: 8px;

        user-select: none;
        cursor: pointer;

       
        transition: all .2s ease-in-out;
    }

    .search-filter-container.selected {
        background-color: var(--color-on-background);
    }

    .search-filter-container.selected * {
        color: var(--color-background-tertiary);
    }

    .search-filter-container:hover {
        /*box-shadow: 0 0 0 2px rgba(255, 255, 255, 0.1);*/
        transform: scale(1.05);
    }

    .search-filter-container:active {
        /*box-shadow: 0 0 0 5px rgba(255, 255, 255, 0.14);*/
        transform: scale(0.95);
    }

    .search-filter-label {
        font-size: 0.8rem;
    }
</style>