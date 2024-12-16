<script lang="ts">
	import { getCurrentUser, getLoggedUsername, logout } from '$api/login';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import Avatar from '$components/Avatar.svelte';
	import NavItem from '$components/NavItem.svelte';
	import { ChartColumn, Database, Home, Users } from 'lucide-svelte';
	import { onMount } from 'svelte';

	interface NavRoute {
		label: string;
		href: string;
		icon: any;
	}

	const BASE_URL = "/app";

	let nav: NavRoute[]  = [
		{
			label: "Home",
			href: "/movie",
			icon: Home,
		},
		{
			label: "Statistics",
			href: "/statistics",
			icon: ChartColumn,
		},
		{
			label: "Access",
			href: "/admin",
			icon: Database,
		}
	]

	function getAbsolutePath(path: string) {

		console.log("Path: ", `${BASE_URL}${path}`);

		return `${BASE_URL}${path}`;
	}
	
	let username: string | null = $state("");

	onMount(() => {
		init();
	});

	function init() {
		getLoggedUsername().then((res) => {
			console.log(res);
			username = res as string;
		}).catch((err) => {
			console.error("ERROR:", err);
		});

		getCurrentUser().then((user) => {
			console.log("User: ", user);
		}).catch((err) => {
			console.error("ERROR:", err);
		});
	}

	function profilClick() {
		logout().then(() => {
			goto("/login");
		});
	}
</script>

<header>
	<div class="nav-bar">
		{#each nav as ni}
			{@const path = getAbsolutePath(ni.href)}
			<NavItem label={ni.label} href={path} current={$page.url.pathname == path}>
				{#snippet icon()}
					<ni.icon/>
				{/snippet}
			</NavItem>
		{/each}
	</div>

	<div 
		class="user-profil" 
		role="button"
		tabindex="0"

		onclick={profilClick}
		onkeydown="{(e) => e.key == 'Enter' && profilClick()}"
	>
		<div class="user-details">
			<h5 class="username">{username}</h5>
			<!--<h5 class="user-status">Status</h5>-->
		</div>
		<Avatar/>
	</div>
</header>

<style>
	header {
		position: relative;
		background: var(--color-background-secondary);
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);

		padding-left: 16px;
		padding-right: 16px;

		min-height: 62px;

		box-shadow: 0 0 10px rgba(0, 0, 0, 0.5);

		flex-shrink: 0;

		align-items: center;
		justify-content: space-between;
		display: flex;
	}

	.nav-bar {
		padding-left: 12px;
		padding-right: 12px;


        gap: 12px;
		
		display: flex;
		align-items: center;

		z-index: 1;
	}

	.user-profil {
		flex-direction: row;
		display: flex;

		cursor: pointer;
	}

	.user-details {
		margin-right: 16px;

		justify-content: center;
		align-items: end;
		flex-direction: column;
		display: flex;
	}

	.username {
		margin: 0;
	}

	.user-status {
		font-weight: normal;
		color: var(--color-text-secondary);
		margin: 0;
	}
</style>
