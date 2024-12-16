<script lang="ts">

    let { 
        size = 32,
    }: {
        size?: number;
    } = $props();
    const widthStyle = $derived(size != undefined ? `width:${size}px;` : '');
</script>


<div class="spin" style="{widthStyle}">
    <svg viewBox="0 0 50 50">
        <defs>
            <linearGradient id="spinner-gradient" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color: currentcolor; stop-opacity: 1;"></stop>
                <stop offset="100%" style="stop-color: currentcolor; stop-opacity: 0;"></stop>
            </linearGradient>
        </defs>
        <circle cx="25" cy="25" r="20" stroke="url(#spinner-gradient)" stroke-width="8" stroke-linecap="round" fill="none" stroke-dasharray="75,150" stroke-dashoffset="0"></circle>
    </svg>
</div>

<style>
    .spin {
        
        aspect-ratio: 1;

        /*border-color: var(--color-on-background);
        border-radius: 50%;
        border-width: 3px;
        border-style: solid;
        border-top-color: transparent !important;*/

        /*animation: spin .8s ease-in-out infinite;*/
        animation: spin .8s linear infinite;
    }

    @keyframes spin {
        100% {
            transform: rotate(360deg);
        }
    }
</style>