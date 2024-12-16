<script lang="ts">
    import { login, type SqlUserChoice, type SqlUserCredentials } from "$api/login";
	import { goto } from "$app/navigation";
	import Spinner from "$components/Spinner.svelte";
	import LoginAddCard from "./LoginAddCard.svelte";
	import LoginBox from "./LoginBox.svelte";
    import LoginChoiceCard from "./LoginChoiceCard.svelte";

    const CHOICE_LIST: SqlUserChoice[] = [
        {
            title: "Spectator",
            username: "spectator",
        },
        {
            title: "Administrator",
            username: "admin",
        },
    ];

    let choice: SqlUserChoice | null = $state(null);

    function onChoiceClick(c: SqlUserChoice = {}) {
        choice = c;
    }

    function back() {
        choice = null;
    }

    let isLoading = $state(false);

    function submit(credentials: SqlUserCredentials) {
        console.log("LOGIN", "CREDENTIALS:", credentials);
        isLoading = true;
        login(credentials).then((res) => {
            console.log("LOGIN RESPONSE:", res);
            goto("/app/movie");
        }).catch((err) => {
            isLoading = false;
            console.error("LOGIN ERROR:", err);
        });
    }
</script>


<section class="login-container">
    <div class="content">
        {#if choice === null}
            <div class="row-choice">
                {#each CHOICE_LIST as item}
                    <LoginChoiceCard choice={item} onclick={() => onChoiceClick(item)}/>
                {/each}

                <LoginAddCard onclick={onChoiceClick}/>
            </div>
        {:else}
            {#if isLoading}
                <Spinner/>
            {:else}
                <LoginBox choice={choice} onsubmit={submit} oncancel={back}/>
            {/if}
        {/if}
    </div>
</section>

<style>
    section.login-container {
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;

        display: flex;
    }

    .login-container .content {
        justify-content: center;
        align-items: center;
        flex-grow: 1;
        display: flex;
    }

    .row-choice {
        gap: 20px;
        flex-direction: row;
        display: flex;
    }
</style>