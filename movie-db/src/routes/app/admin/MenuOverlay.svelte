<script lang="ts">
    let {
        children,
    } : {
        children: () => any;
    } = $props();
</script>


<div class="menu-overlay-container">
    <div class="menu-overlay">
        {@render children()}
    </div>
</div>

<style>
    .menu-overlay-container {
        position: fixed;
        background: rgba(0,0,0,0.5);

        top: 0;
        left: 0;
        right: 0;
        bottom: 0;   
        
        align-items: center;
        justify-content: center;
        display: flex;
    }
</style>