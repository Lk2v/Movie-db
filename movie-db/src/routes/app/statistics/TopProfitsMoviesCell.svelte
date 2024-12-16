<script lang="ts">
	import { formatMoney } from '$api/format';
	import { getPosterPath } from '$api/movie';
	import type { TopMovieProfit } from '$api/stats';
	import Avatar from '$components/Avatar.svelte';
	import StatArticleCellFrame from './StatArticleCellFrame.svelte';

	let {
		top_profits_movies
	}: {
		top_profits_movies: TopMovieProfit[] | undefined;
	} = $props();
</script>

<StatArticleCellFrame title="Top Profits Movies" subtitle="Movies with the highest profit">
	{#if top_profits_movies == undefined}
		<p>Loading...</p>
	{:else}
		{#each top_profits_movies as movie, i}
			<div class="movie-profit-item">
                <h1 class="movie-rank">#{i+1}</h1>
                <img class="movie-poster" src={getPosterPath(movie.poster_path, 200)} alt={movie.title} />
                <div class="movie-detail">
                    <h5 class="movie-title">{movie.title}</h5>
                    
                    <h3 class="movie-profit">{formatMoney(movie.profit)}</h3>
                </div>
            </div>
		{/each}
	{/if}
</StatArticleCellFrame>

<style>
	.movie-profit-item {
        margin: 0.5rem 0;
        align-items: center;
        flex-direction: row;
        display: flex;
    }

    .movie-rank {
        font-size: 1.5rem;
        width: 3rem;
        margin: 0;
        margin-right: 1rem;
    }

    .movie-poster {
        width: 42px;

        box-shadow: 0 0 5px rgba(0, 0, 0, 0.5);

        border-radius: 3px;
        margin-right: 1rem;
    }

    .movie-detail {
        flex-grow: 1;

        align-items: center;
        flex-direction: row;
        display: flex;
    }

    .movie-title {
        margin-left: 1rem;
        margin-right: 1rem;
        flex-grow: 1;
    }

    .separator {
        background-color: rgba(225, 225, 225, 0.1);
        width: 2px;
        height: 10px;

        margin: 0 1rem;
    }
</style>
