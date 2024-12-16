<script lang="ts">
    let {
        title,
        subtitle,
        children
    } : {
        title: string,
        subtitle: string,
        children: () => any
    } = $props();
</script>

<div class="article-cell">
    <div class="stat-item-content">
        <div class="stat-item-header">
            <h3 class="cell-title">{title}</h3>
            <subtitle>{subtitle}</subtitle>
        </div>
        <div class="stat-item-body">
            <div class="stat-item-body-scroll">
                {@render children()}
            </div>
        </div>
    </div>
</div>

<style>
    .article-cell {
        width: 100%;
        height: 100%;

        
        padding: calc(var(--cell-padding) * 0.2);
        box-sizing: border-box;
       

        flex-direction: column;
        display: flex;
    }

    .stat-item-content {
        flex-grow: 1;
        flex-direction: column;
        display: flex;
    }

    .stat-item-header {
    }

    .cell-title {
        font-weight: 600;

        margin: 0;
    }

    .stat-item-body {
        position: relative;
        flex-grow: 1;
    }

    .stat-item-header, .stat-item-body-scroll {
        padding: calc(var(--cell-padding) * 0.8);
    }

    .stat-item-body-scroll {
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;

        overflow-y: auto;

        flex-direction: column;
        display: flex;
    }
</style>