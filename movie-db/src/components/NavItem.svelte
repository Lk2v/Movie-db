<script lang="ts">
    let {
        label,
        icon,
        href,
        current
    } : {
        label: string,
        icon: () => any,
        href: string,
        current: boolean,
    } = $props();

</script>

<a href={href}>
    <div class="nav-bar-item" class:current>
        {#if icon}
            {@render icon()}
        {/if}
        <span class="nav-item-name">{label}</span>
    </div>
</a>

<style>
    .nav-bar-item {
        border-radius: 8px;

        padding: 8px 16px;

        box-sizing: border-box;
        border-width: 2px;
        border-style: solid;
        border-color: transparent;

        align-items: center;
        justify-content: center;
        flex-direction: row;
        display: flex;

        transition: transform .3s ease-in-out;
    }

    .nav-bar-item.current {

        background-color: rgba(225, 225, 225, 0.14);
    }

    .nav-bar-item:hover {
        
        border-color: rgba(225, 225, 225, 0.2);
    }

    .nav-bar-item:active {
        transform: scale(0.95);
    }

    .nav-item-name {
        margin-left: 12px;
    }
</style>

