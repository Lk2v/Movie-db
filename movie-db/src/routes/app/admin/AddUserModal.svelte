<script lang="ts">
	import type { SqlUser, SqlUserProps } from "$api/sql_user";
	import Button from "$components/Button.svelte";
	import Modal from "$components/Modal.svelte";
import Select from "$components/Select.svelte";
    import TextField from "$components/TextField.svelte";

    let {
        onsubmit,
        onclose
    } : {
        onsubmit: (user: SqlUserProps) => void;
        onclose: () => void;
    } = $props();
    
    let user: SqlUserProps = $state({
        username: "",
        password: "",
        is_admin: false
    });
    
    let role = $state(0);

    $effect(() => {
        user.is_admin = role === 1;
    });

    function submit() {
        onsubmit(user);
    }
</script>

<Modal title="Add user" desciption="Add a new user to the system">
    {#snippet body()}
        <TextField label="Username" bind:value={user.username} />
        <TextField label="Password" bind:value={user.password} password/>

        <div class="role-select">
            <Select bind:value={role}>
                <option value={0}>Spectator</option>
                <option value={1}>Admin</option>
            </Select>
        </div>
    {/snippet}

    {#snippet footer()}
        <Button mode="outlined" onclick={onclose}>Close</Button>
        <Button onclick={submit}>Submit</Button>
    {/snippet}
</Modal>

<style>
    .role-select {
        margin-top: 1rem;

        justify-content: flex-start;
        flex-direction: row;
        display: flex;
    }
</style>