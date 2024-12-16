<script lang="ts">
    let {
        value,
        details,
        icon
    }: {
        value: string | number;
        details?: () => any;
        icon: () => any;
    } = $props();
</script>

<div class="inline-stat">

    {@render icon?.()}

    <span class="inline-stat-value">{value}</span>
    {#if details}
        <span class="inline-stat-details">
            {@render details()}
        </span>
    {/if}
</div>

<style>
    .inline-stat {
        margin: 1rem 0;

        align-items: center;
        flex-direction: row;
        display: flex;
    }

    .inline-stat-value {
        font-weight: bold;
        margin-left: 8px;
    }

    .inline-stat-details {
        margin-left: 1em;

        justify-content: center;
        flex-direction: row;
        display: flex;

        opacity: var(--secondary-opacity);
    }
</style>

