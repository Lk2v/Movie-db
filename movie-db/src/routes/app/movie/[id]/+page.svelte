<script lang="ts">
	import { page } from "$app/stores";

    import { onMount } from "svelte";
    import { deleteMovieLensTag, deleteMovieLensUser, getMovie } from "$api/fetch";
	import { getBackdropPath, getPosterPath, type Movie, type MovieTag } from "$api/movie";
	import Tag from "./Tag.svelte";
	import { formatDuration, formatVote, formatWithSpace } from "$api/format";
	import InlineStat from "$components/InlineStat.svelte";
	import { Clock, Command, Star, StarHalf, ThumbsUp } from "lucide-svelte";
	import Mark from "$components/Mark.svelte";
	import DropDownSection from "$components/DropDownSection.svelte";

    // get movie id from page route
    let { id } = $page.params;

    let data: Movie | null = $state(null);
    
    onMount(init);

    function init() {
        getMovie(id).then((movie) => {
            console.log("ID:", id, "MOVIE:", movie);
            data = movie;
        });
    }

    let star_rating: number | null = $derived.by(() => {
        if (!data) return null;
        let sum = 0;

        for (let i = 0; i < data.ratings.length; i++) {
            sum += data.ratings[i].rating;
        }

        let rating = sum / data.ratings.length;

        // round to 1 decimal place
        return Math.round(rating * 10) / 10;
    });

    let full_star_count: number | null = $derived.by(() => {
        if (!star_rating) return null;
        return Math.trunc(star_rating) + (star_rating % 1 == 0 ? 0 : 1);
    });


    function onRemoveTag(tag: MovieTag) {
        console.log("Removing tag", tag);

        const movieId = data!.details.movie_id;

        deleteMovieLensTag(movieId, tag.user_id, tag.timestamp).then(() => {
            console.log("Tag removed");

            // remove tag from list
            if(data) data.tags = data.tags.filter((t) => t.user_id != tag.user_id || t.timestamp != tag.timestamp);
        });
    }

    function onRemoveUser(tag: MovieTag) {
        console.log("Removing user", tag);

        deleteMovieLensUser(tag.user_id).then(() => {
            console.log("User removed");

            // remove tag from list
            if(data) data.tags = data.tags.filter((t) => t.user_id != tag.user_id);
        });
    }

</script>

<section class="movie-page">
    {#if data}

    {#if data.details.backdrop_path}
        <div class="movie-page-backdrop" style={`background-image: url(${getBackdropPath(data.details.backdrop_path)})`} ></div>
    {/if}
    
    <div class="movie-page-content">
        <div class="movie-page-header">
            <div class="movie-page-poster">
                <img class="movie-poster-image" src={getPosterPath(data.details.poster_path)} alt={data.details.title} />
            </div>
            <div class="movie-page-details">
                <div class="movie-status-container">
                    <div class="movie-status">{data.details.status} - {data.details.release_date}</div>
                </div>
                <h1 class="movie-title">
                    <span>{data.details.title}</span>
                </h1>

                {#if data.details.adult}
                    <span class="movie-status-adult">18+</span>
                {/if}
                    <p>{data.details.overview}</p>

                <div class="movie-genres">
                    {#each data.details.genres.split(',') as genre}
                        <span class="movie-genre-item">{genre}</span>
                    {/each}
                </div>

                <InlineStat value={formatDuration(data.details.runtime)}>
                    {#snippet icon()}
                        <Clock />
                    {/snippet}
                </InlineStat>

                <InlineStat value={`${formatVote(data.details.vote_average)} %`}>
                    {#snippet icon()}
                        <ThumbsUp />
                    {/snippet}

                    {#snippet details()}
                        {formatWithSpace(data!.details.vote_count)} votes <Mark label="TMDB"/>
                    {/snippet}
                </InlineStat>

                
                {#if star_rating && full_star_count}
                    <div class="movie-star-rating">
                        <div class="star-container">
                            {#each {length: full_star_count} as _, i}
                                {@const is_last = i+1 == full_star_count}
                            <div class="star-item" class:last={is_last} style={"--star-cut: " + ((full_star_count - star_rating) * 100) + "%;"}>
                                <Star />
                                </div>
                            {/each}
                        </div>
                        <h4 class="movie-star-rating-value">{star_rating}</h4>

                        <span class="movie-star-rating-count">{data.ratings.length} ratings <Mark label="MovieLens"/></span>
                    </div>
                {/if}
            </div>
        </div>
        
        <div class="movie-page-body">
            <div class="movie-page-tags">
                {#each data.tags as tag, i}
                    <Tag 
                        index={i}
                        tag={tag}
                        
                        onRemoveTag={() => onRemoveTag(tag)}
                        onRemoveUser={() => onRemoveUser(tag)}
                    />
                {/each}
            </div>
        </div>

        <div class="movie-page-footer">
            <DropDownSection title="Debug - Query Data">
                <pre class="pre-debug">{JSON.stringify(data, null, 2)}</pre>
            </DropDownSection>

            <div class="movie-debug-id">
                Movie ID: {id} | TMDB ID : {data.details.tmdb_id}
            </div>
        </div>
    </div>
    {/if}
</section>

<style>
    section.movie-page {
        width: 100%;
        height: 100%;

        margin: var(--page-padding);

        box-sizing: border-box;

        display: flex;
        flex-direction: column;
    }

    .movie-page-content {
        flex-grow: 1;

        flex-direction: column;
        display: flex;
    }

    .movie-page-header {
        position: relative;

        display: flex;
        flex-direction: row;
    }

    .movie-poster-image {
        height: 350px;
    
        box-shadow: 0 14px 33px 0 rgba(0, 0, 0, 0.5);
        border-radius: 8px;

        aspect-ratio: 2 / 3;

        display: block;
    }

    .movie-page-poster {
        animation-name: scale;
        animation-duration: 1s;
    }

    .movie-page-details {
        padding: 0 2rem;

        animation-name: slideUp;
        animation-duration: 1s;
        animation-fill-mode: backwards;
        animation-delay: 200ms;
    }

    .movie-status-container {
        margin-bottom: 1rem;

        align-items: center;
        flex-direction: row;
        display: flex;
    }

    .movie-status {
        font-size: 1rem;
        color: var(--color-text-secondary);
        font-weight: 600;
    }

    .movie-status-adult {
        font-size: 0.8rem;
        background: var(--orange-color);
        border-radius: 6px;
        padding: 4px 8px;
    }

    .movie-genres {
        flex-wrap: wrap;
        flex-direction: row;
        display: flex;
    }

    .movie-genre-item {
        font-size: 0.8rem;
        border: 2px solid rgba(255, 255, 255, 0.2);
        border-radius: 100px;
        padding: 4px 8px;

        margin-right: 8px;
        margin-bottom: 8px;

        text-align: center;
    }

    .movie-star-rating {
        margin: 1rem 0;

        align-items: center;
        flex-direction: row;
        display: flex;
    }

    .star-container {
        flex-direction: row;
        display: flex;
    }

    .star-item {
        display: flex;
    }
    .star-item.last {
        mask-image: linear-gradient(to left, transparent var(--star-cut), black var(--star-cut));
        -webkit-mask-image: linear-gradient(to left, transparent var(--star-cut), black var(--star-cut));
    }

    .movie-star-rating-value {
        margin: 0 12px;
    }

    .movie-star-rating-count {
        margin-left: 12px;
        opacity: var(--secondary-opacity);
    }

    .movie-page-backdrop {
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;

        background-size: cover;
        background-position: center;

        -webkit-mask-image: linear-gradient(18deg, transparent 30%, black 100%);
        mask-image: linear-gradient(18deg, transparent 30%, black 100%);

        opacity: 0.5;
        z-index: -1;
    }
    
    .movie-page-body {
        padding: 2rem 0;
    }

    .movie-page-tags {
        align-items: start;
        flex-direction: column;
        display: flex;
    }

    .movie-page-footer {
        
    }

    .movie-debug-id {
        margin: 40px;

        justify-content: center;
        align-items: center;
        display: flex;

        opacity: var(--secondary-opacity);
    }

    .pre-debug {
        text-wrap: wrap;
    } 

</style>