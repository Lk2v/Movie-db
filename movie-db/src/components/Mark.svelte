<script lang="ts">
    let {
        label,
        opacity = 1
    }: {
        label: string;
        opacity?: number;
    } = $props();
</script>

<span class="mark" style="--opacity: {opacity};">
    {label}
</span>

<style>
    .mark {
        font-size: 0.6rem;
        font-weight: 600;
        background: var(--color-on-background);
        color: var(--color-background);
        padding: 4px 8px;
        border-radius: 4px;

        margin: 0 8px;

        opacity: var(--opacity);
    }
</style>