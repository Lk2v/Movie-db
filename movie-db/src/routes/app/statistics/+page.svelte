<script lang="ts">
	import { getStats } from "$api/fetch";
	import type { CountStats, Stats } from "$api/stats";
	import { onMount } from "svelte";
	import StatItem from "./StatItem.svelte";
	import TopUsersCell from "./TopUsersCell.svelte";
	import StatGenresGraph from "./StatGenresGraph.svelte";
	import Spinner from "$components/Spinner.svelte";
	import TopProfitsMoviesCell from "./TopProfitsMoviesCell.svelte";

    let data: Stats | null = $state(null);

    onMount(() => {
        init();
    });

    function init() {
        getStats().then((res) => {
            data = res;
            console.log(data);
        }).catch((error) => {
            console.error(error);
        });
    }
</script>


<section class="statistics-page">
    {#if data === null}
        <div class="loading-message">
            <Spinner/>
            <h1>Retrieving statistics...</h1>
            <subtitle>This might take a while, please wait</subtitle>
        </div>
    {:else}
        <div class="statistics-grid">

            <!-- COUNT STAT -->
            <div class="cell movie count">
                <StatItem label="Movies count" subtitle="Total distinct movies" value={data?.count.total_movies}/>
            </div>

            <div class="cell user count">
                <StatItem label="Users count" subtitle="Total distinct users" value={data?.count.total_users}/>
            </div>

            <div class="cell rating count">
                <StatItem label="Ratings count" subtitle="Total user ratings" value={data?.count.total_ratings}/>
            </div>

            <div class="cell tags count">
                <StatItem label="Tags count" subtitle="Total user tags" value={data?.count.total_tags}/>
            </div>

            <div class="cell genres count">
                {#if data === null}
                    <p>Loading...</p>
                {:else}
                    <StatGenresGraph data={data.count.genre_count}/>
                {/if}
            </div>

            <!-- TOP -->
            <div class="cell top user">
                <TopUsersCell top_users={data?.top_users}/>
            </div>

            <div class="cell top movie">
                <TopProfitsMoviesCell top_profits_movies={data?.top_profits_movies}/>
            </div>
        </div>
    {/if}
</section>

<style>
    section.statistics-page {
        width: 100%;
        height: 100%;

        box-sizing: border-box;
        
        align-items: center;
        display: flex;
        flex-direction: column;
    }

    .loading-message {
        flex-grow: 1;

        align-items: center;
        justify-content: center;
        flex-direction: column;
        display: flex;
    }
    .statistics-grid {
        --cell-width: 250px;
        --cell-height: calc(var(--cell-width) * 0.8);
        --cell-padding: 20px;

        padding: 2rem;
        display: grid;
        grid-template-columns: repeat(4, var(--cell-width));
        grid-template-rows: repeat(4, var(--cell-height));
        gap: 20px;
    }

    .cell {
        position: relative;

        background: rgba(255, 255, 255, 0.05);
        border-radius: 6px;
        overflow: hidden;
    }

    /* COUNT */
    .cell.movie.count {
        grid-row: 1;
        grid-column: 1;
    }

    .cell.user.count {
        grid-row: 1;
        grid-column: 2;
    }

    .cell.rating.count {
        grid-row: 2;
        grid-column: 1;
    }

    .cell.tags.count {
        grid-row: 2;
        grid-column: 2;
    }

    .cell.genres.count {
        grid-row: 1 / span 2;
        grid-column: 3 / span 2;
    }

    /* TOP */
    .cell.top.user {
        grid-row: 3 / span 2;
        grid-column: 1 / span 2;
    }

    /* BLANK */
    .cell.top.movie {
        grid-row: 3 / span 2;
        grid-column: 3 / span 2;
    }


    /* RESPONSIVE */
    @media (max-width: 1200px) {
        .statistics-grid {
            --cell-width: 200px;
        }
    }

    @media (max-width: 1000px) {
        .statistics-grid {
            grid-template-columns: repeat(2, var(--cell-width));
            grid-template-rows: repeat(8, var(--cell-height));
        }

        .cell.genres.count {
            grid-row: 3 / span 2;
            grid-column: 1 / span 2;
        }

        .cell.top.user {
            grid-row: 5 / span 2;
            grid-column: 1 / span 2;
        }

        .cell.top.movie {
            grid-row: 7 / span 2;
            grid-column: 1 / span 2;
        }
    }
</style>