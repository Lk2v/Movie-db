<script lang="ts">
	import type { SqlUser } from "$api/sql_user";
	import Avatar from "$components/Avatar.svelte";
	import IconButton from "$components/IconButton.svelte";
	import { Crown, Trash } from "lucide-svelte";

    let { 
        user,
        ondelete
    } : { 
        user: SqlUser,
        ondelete: () => void
    } = $props();
</script>


<div class="user-item-container">
    <div class="user-item-avatar">
        <Avatar/>
    </div>

    <div class="user-item-content">
        <h5 class="username">{user.username}</h5>
        {#if user.is_admin}

        <div class="user-admin-role">
            <Crown size={18} color={"#ffb343"}/>
        </div>
        {/if}
    </div>
    <div class="user-actions">
        <IconButton onclick={ondelete}>
            <Trash size={18}/>
        </IconButton>
    </div>
</div>

<style>
    .user-item-container {
        border-bottom: 1px solid rgba(225,225,225,0.2);
        padding: 16px 18px;

        align-items: center;
        flex-direction: row;
        display: flex;

        cursor: pointer;
    }

    .user-item-container:last-child {
        border-bottom: none;
    }

    .user-item-avatar {
        margin-right: 16px;
        flex-shrink: 0;
    }

    .user-item-content {
        flex-grow: 1;

        align-items: center;
        flex-direction: row;
        display: flex;
    }

    .user-admin-role {
        margin: 0 12px;
    }

    .username {
        margin: 0;
    }
</style>