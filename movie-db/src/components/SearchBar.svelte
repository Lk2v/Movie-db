<script lang="ts">
    import { Search } from "lucide-svelte";

    let {
        value = $bindable(''),
        trailing,
        onenter,
    }: {
        value: string;
        trailing: () => any,
        onenter: () => void;
    } = $props();

    function handleKeyDown(event: KeyboardEvent) {
        if (event.key === 'Enter') {
            onenter();
        }
    }
</script>

<div class="search-bar-container">
    <div class="search-bar-content">
        <div class="search-bar-icon">
            <Search />
        </div>
        {#if trailing}
            <div class="trailing">
                {@render trailing()}
            </div>
        {/if}
        <div class="search-bar-input">
            <input type="text" bind:value={value} placeholder="Search for a movie" onkeydown={handleKeyDown} />
        </div>
    </div>
</div>

<style>
    .trailing {
        padding-right: 12px;
        margin-right: 12px;
        border-right: 1px solid rgba(225, 225, 225, 0.14);
    }

    .search-bar-container:focus-within {
        box-shadow: 0 9px 9px 0 rgba(0, 0, 0, 0.14);
    }

    .search-bar-container {
        background-color: var(--color-background-tertiary);
        border-radius: 8px;
        padding: 12px 16px;

        transition: all .2s ease-in-out;
    }

    .search-bar-content {

        flex-direction: row;
        display: flex;
    }

    .search-bar-icon {
        margin-right: 0.5rem;
    }

    .search-bar-input {
        flex-grow: 1;
        display: flex;
    }

    .search-bar-input input {
        font-size: 0.8rem;
        height: 100%;
        width: 100%;
        border: none;
        background: none;
        outline: none;
    }
</style>