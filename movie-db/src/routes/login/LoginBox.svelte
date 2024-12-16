<script lang="ts">
	import type { SqlUserChoice, SqlUserCredentials } from "$api/login";
	import Button from "$components/Button.svelte";
	import LinkButton from "$components/LinkButton.svelte";
	import TextField from "$components/TextField.svelte";

    let {
        choice,
        onsubmit,
        oncancel,
    }: {
        choice : SqlUserChoice,
        onsubmit: (credentials: SqlUserCredentials) => void,
        oncancel: () => void,
    } = $props();

    let username = $state(choice.username ?? "");
    let password = $state("");

    function submit() {
        onsubmit({ username, password });
    }
</script>


<div class="login-box-container">
    <div class="login-box">
        <div class="login-box-header">
            <h3>{choice.username}</h3>
        </div>
        <div class="login-box-body">
            <TextField bind:value={username} label="Username" placeholder="Username" autocapitalize="off" disabled={choice.username != undefined}/>
            <TextField bind:value={password} label="Password" placeholder="Password" password/>
        </div>
        <div class="login-box-footer">
            <Button mode="outlined" onclick={oncancel}>Back</Button>
            <Button onclick={submit}>Submit</Button>
        </div>
    </div>
</div>

<style>
    .login-box-container {
        align-items: center;
        flex-direction: column;
        display: flex;
    }

    .login-box {
        width: 300px;
        height: 350px;
        background: var(--color-background-tertiary);

        border-radius: 16px;

        padding: 25px;
        margin: 10px 0;

        flex-direction: column;
        display: flex;
    }

    .login-box-body {
        flex-grow: 1;
        flex-direction: column;
        display: flex;
    }

    .login-box-footer {
        padding-top: 15px;

        justify-content: flex-end;
        display: flex;
    }
</style>