<script lang="ts">
	import type { TopUserItem } from '$api/stats';
	import Avatar from '$components/Avatar.svelte';
	import StatArticleCellFrame from './StatArticleCellFrame.svelte';

	let {
		top_users
	}: {
		top_users: TopUserItem[] | undefined;
	} = $props();
</script>

<StatArticleCellFrame title="Top Users" subtitle="Most active users">
	{#if top_users == undefined}
		<p>Loading...</p>
	{:else}
		{#each top_users as user, i}
            {@const contributionCount = user.num_tags + user.num_ratings}
			<div class="user-rank-item">
                <h1 class="user-rank">#{i+1}</h1>
                <div class="avatar">
                    <Avatar size={16}/>
                </div>
                <div class="user-detail">
                    <div class="user-conribution-count">{contributionCount} contributions</div>
                    <div class="user-count-detail">
                        <subtitle class="tags-count">{user.num_tags} tags</subtitle>
                        <div class="separator"></div>
                        <subtitle class="ratings-count">{user.num_ratings} ratings</subtitle>
                    </div>
                </div>
            </div>
		{/each}
	{/if}
</StatArticleCellFrame>

<style>
	.user-rank-item {
        margin: 0.5rem 0;
        align-items: center;
        flex-direction: row;
        display: flex;
    }

    .user-rank {
        margin: 0;
        margin-right: 1rem;
    }

    .user-detail {
        flex-direction: column;
        display: flex;
    }

    .user-conribution-count {
        font-weight: bold;
    }

    .user-count-detail {
        align-items: center;
        flex-direction: row;
        display: flex;
    }

    .avatar {
        margin-right: 1rem;
    }
    .separator {
        background-color: rgba(225, 225, 225, 0.1);
        width: 2px;
        height: 10px;

        margin: 0 1rem;
    }
</style>
