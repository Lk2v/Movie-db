<script lang="ts">
	import { User } from "lucide-svelte";

    let {
        size = 18
    } : {
        size?: number
    } = $props();
</script>


<div class="avatar-container">
    <div class="avatar-icon">
        <User size={size}/>
    </div>
</div>


<style>
    .avatar-container {
        background-color: rgba(225, 225, 225, 0.1);
        border-radius: 50%;
        padding: 10px;

        transition: all .2s ease-in-out;
    }

    .avatar-icon {
    }
</style>