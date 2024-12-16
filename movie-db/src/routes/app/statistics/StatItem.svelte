<script lang="ts">
    let { 
        label,
        subtitle,
        value
    }: {
        label: string,
        subtitle: string,
        value: string | number | undefined
    } = $props();
</script>

<div class="stat-item">
    <div class="stat-item-content">
        <div class="stat-item-header">
            <h3 class="stat-label">{label}</h3>
            <subtitle>{subtitle}</subtitle>
        </div>
        
        <h1 class="stat-value">
            {value == undefined ? "--" : value}
        </h1>
    </div> 
</div>

<style>
    .stat-item {
        width: 100%;
        height: 100%;

        padding: var(--cell-padding);
        box-sizing: border-box;
        
        display: flex;
    }

    .stat-item-content {
        flex-grow: 1;
        flex-direction: column;
        display: flex;
    }

    .stat-item-header {
        flex-grow: 1;

        flex-direction: column;
        display: flex;
    }

    .stat-label {
        font-weight: 600;

        margin: 0;
    }

    .stat-value {
        font-size: 2rem;
        font-weight: bold;
        margin: 0;
    }
</style>