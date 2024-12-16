<script lang="ts">
	import { formatVote } from "$api/format";
	import { ThumbsUp } from "lucide-svelte";
	import { getPosterPath, type MovieShort } from "../api/movie";

    let {
        movie
    } : {
        movie: MovieShort
    } = $props();
</script>

<a href={`/app/movie/${movie.movie_id}`}>
    <div class="movie-card-container">
        <div class="movie-card">
            <div class="movie-thumbnail">
                <span class="movie-vote-average">
                    <ThumbsUp size={18}/>
                    <span class="movie-vote-average-value">{formatVote(movie.vote_average)} %</span>
                </span>
                <img src={getPosterPath(movie.poster_path, 300)} alt={movie.title} />
            </div>
            <div class="movie-details">
                <h5 class="movie-title">{movie.title}</h5>
            </div>
        </div>
    </div>
</a>


<style>
    .movie-card-container {
        margin: 12px;
        
        display: flex;
        justify-content: center;
        align-items: center;

        cursor: pointer;


        transition: transform .5s;
    }

    .movie-card-container:hover {
        transform: scale(1.05);
    }

    .movie-card-container:active {
        transform: scale(0.95);
    }

    .movie-card {
        
        overflow: hidden;

        flex-direction: column;
        display: flex;
    }

    .movie-thumbnail {
        position: relative;
        flex-shrink: 0;
        border-radius: 5px;
        width: 100%;
        overflow: hidden;
    }

    .movie-vote-average {
        position: absolute;
        top: 5px;
        left: 5px;
        padding: 5px;

        background-color: rgba(0, 0, 0, 0.5);
        backdrop-filter: blur(10px);
        border-radius: 5px;

        justify-content: center;
        align-items: center;
        flex-direction: row;
        display: flex;
    }

    .movie-vote-average-value {
        margin-left: 5px;
    }
    .movie-thumbnail img {
        width: 100%;
        object-fit: cover;
        object-position: center;

        aspect-ratio: 2 / 3;

        display: block;
    }

    .movie-details {
        margin-top: 10px;
        text-align: center;

        flex-grow: 1;
    }

    .movie-title {
        margin: 0;
    }
</style>
