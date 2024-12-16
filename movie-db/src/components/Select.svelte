<script lang="ts">
	import { ChevronDown } from "lucide-svelte";

    let {
        value = $bindable(''),
        children,
        onchange
    }: {
        value: string | number,
        children: () => any,
        onchange?: (event: Event) => void;
    } = $props();

    let selectElement: HTMLSelectElement;

    function focus() {
        selectElement.focus();
    }
</script>

<div 
    class="select-container" 
    role="button"

    onclick={focus}
    onkeydown={focus}

    tabindex="0"
>
    <select bind:this={selectElement} {onchange} bind:value={value}>
        {@render children()}
    </select>
    <div class="chevron">
        <ChevronDown/>
    </div>
</div>

<style>
    .select-container {
        position: relative;
        align-items: center;
        justify-content: center;
        flex-direction: row;
        display: flex;
        z-index: 1;
    }

    select {
        background-color: transparent;
        border: none;
        outline: none;

        font-weight: bold;

        padding: 5px; 

        padding-right: 26px;

        appearance: none; 
        -webkit-appearance: none; 
        -moz-appearance: none;

        cursor: pointer;
    }

    .chevron {
        position: absolute;
        right: 0;
        top: 0;
        bottom: 0;

        align-items: center;
        justify-content: center;
        display: flex;

        z-index: -1;
    }
</style>