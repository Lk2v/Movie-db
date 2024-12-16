<script lang="ts">
	import Button from "$components/Button.svelte";
    import UserItem from "./UserItem.svelte";
	import MenuOverlay from "./MenuOverlay.svelte";
	import AddUserModal from "./AddUserModal.svelte";
	import type { SqlUser, SqlUserList, SqlUserProps } from "$api/sql_user";
	import { onMount } from "svelte";
	import { createSqlUser, deleteSqlUser, getSqlUsers } from "$api/fetch";

    let users: SqlUserList | null = $state(null);

    let addUserModal = $state(false);



    let hasError: boolean = $state(false);

    onMount(() => {
        load();
    });
    
    function load() {
        hasError = false;

        getSqlUsers().then((data) => {
            users = data;
        }).catch((err) => {
            console.error(err);
            hasError = true;
        });
    }

    function openUserModal() {
        addUserModal = true;
    }

    function closeAddUserModal() {
        addUserModal = false;
    }

    function addUser(props: SqlUserProps) {
        console.log(props);

        closeAddUserModal();
        createSqlUser(props).then(() => {
            load();
        });
    }

    function deleteUser(user : SqlUser) {
        console.log("delete user", user);

        deleteSqlUser(user.username).then(() => {
            load();
        });
    }
</script>

<section class="admin-page">
    <div class="admin-header">
        <h1 class="header-title">Users</h1>
        <div class="header-actions">
            <Button onclick={openUserModal}>Add user</Button>
        </div>
    </div>
    <div class="users-collection">
        {#if hasError}
            <p>Error loading users : only admin can manage users</p>
        {:else }
            {#if users === null}
                <p>Loading...</p>
            {:else}
                {#if users.length === 0}
                    <p>No users found</p>
                {:else}
                    {#each users as user}
                        <UserItem user={user} ondelete={() => deleteUser(user)}/>
                    {/each}
                {/if}
            
            {/if}
        {/if}
    </div>
</section>

{#if addUserModal}
    <MenuOverlay>
        <AddUserModal 
            onsubmit={addUser}
            onclose={closeAddUserModal}
        />
    </MenuOverlay>
{/if}


<style>
    section.admin-page {
        width: 100%;
        height: 100%;

        margin: 2rem;

        box-sizing: border-box;

        display: flex;
        flex-direction: column;
    }

    .admin-header {
        margin-bottom: 1rem;
        flex-direction: row;
        display: flex;
    }

    .header-title {
        margin: 0;

        flex-grow: 1;
    }
</style>

