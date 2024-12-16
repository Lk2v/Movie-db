<script lang="ts">
	import MovieCard from '$components/MovieCard.svelte';
	import { type MovieShort } from '$api/movie';
	import { getAllMovies } from '$api/fetch';
	import { onMount } from 'svelte';
	import SearchBar from '$components/SearchBar.svelte';
	import SearchTag from '$components/SearchFilter.svelte';
	import { genresList , SearchFilter } from '$api/search';
	import Spinner from '$components/Spinner.svelte';
	import Select from '$components/Select.svelte';
	import type { ScrollEvents } from 'lucide-svelte/icons/scroll';

	interface SearchFilterItem {
		label: string;
		type: SearchFilter;
	}

	let searchTags: SearchFilterItem[] = [
		{
			label: 'None',
			type: SearchFilter.None
		},
		{
			label: 'Alphabetical',
			type: SearchFilter.Alphabetical
		},
		{
			label: 'Popular',
			type: SearchFilter.Popular
		},
		{
			label: 'Latest',
			type: SearchFilter.Latest
		},
		{
			label: 'Top Rated',
			type: SearchFilter.TopRated
		}
	];

	onMount(() => {
		getMovies();
	});

	let moviesList: MovieShort[] = $state([]);

	let searchValue = $state('');
	let searchGenre = $state(-1);

	let searchFilter = $state(SearchFilter.None);

	let isLoading: boolean = $state(false);
	function changeTagFilter(tag: SearchFilterItem): void {
		searchFilter = tag.type;

		getMovies();
	}

	function getMovies() {
		const genre = searchGenre != -1 ? genresList[searchGenre] : "All";

		getAllMovies(
			genre,
			searchValue,
			searchFilter,
		).then((movies) => {
			console.log(movies);
			moviesList = movies;
			isLoading = false;
		}).catch((err) => {
			console.error("ERROR:", err);
			isLoading = false;
		});
		
		isLoading = true;
	}

	let scrollContainer: HTMLDivElement | undefined = $state();

	function scrollUpdate(e: ScrollEvents) {
		if(!scrollContainer) return;

		const scrollPosition = scrollContainer.scrollTop + scrollContainer.offsetHeight;
		const scrollHeight = scrollContainer.scrollHeight;
		if (scrollPosition >= scrollHeight - 50) {
			// 50px near end of scroll
			console.log("near end");
		}
	}
</script>

<section class="home-page">
	<div class="movie-header">
		<div class="movie-header-title">
			<!---->
		</div>
		<div class="movie-header-search">
			<div class="movie-header-search-content">
				<SearchBar bind:value={searchValue} onenter={getMovies}>
					{#snippet trailing()}
						<Select bind:value={searchGenre} onchange={getMovies}>
							<option value={-1} selected>All</option>
			
							{#each genresList as g, i}
								<option value={i}>{g}</option>
							{/each}

						</Select>
					{/snippet}
				</SearchBar>
				<div class="movie-header-search-tags">
					{#each searchTags as tag}
						<SearchTag label={tag.label} onclick={() => changeTagFilter(tag)} selected={searchFilter === tag.type}></SearchTag>
					{/each}
				</div>
			</div>
		</div>
	</div>
	<div class="movie-collection">
		{#if isLoading}
			<div class="message-container">
				<Spinner/>
			</div>
		{:else}
			{#if moviesList.length > 0}
				<div class="movie-collection-scroll" bind:this={scrollContainer}>
					{#each moviesList as movie}
						<MovieCard movie={movie} />
					{/each}
				</div>
			{:else}
				<div class="message-container">
					<subtitle>Nothing to display</subtitle>
				</div>
			{/if}
		{/if}
	</div>

</section>

<style>
	section.home-page {
		width: 100%;
		height: 100%;
		
		box-sizing: border-box;
		flex-direction: column;
		display: flex;
	}

	.movie-header {
		margin-bottom: 26px;
		flex-direction: column;
		display: flex;
	}

	.movie-header-title {
		font-size: 2rem;
		text-align: center;
		margin-bottom: 1rem;
	}

	.movie-header-search {
		justify-content: center;
		display: flex;
	}

	.movie-header-search-content {
		width: 60vw;
		min-width: 300px;
		max-width: 800px;

		flex-direction: column;
		display: flex;
	}

	.movie-header-search-tags {
		margin-top: 12px;

		gap: 8px;
		flex-grow: 1;

		justify-content: center;
		align-items: center;
		flex-direction: row;
		display: flex;
	}

	.movie-collection {
		position: relative;
		flex-grow: 1;

	}

	.movie-collection-scroll {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;

		overflow: auto;

		gap: 16px;
		grid-template-columns: repeat(auto-fill, minmax(175px, max-content));
		justify-content: center;
		padding: 16px;

		display: grid;
	}


	.message-container {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;

		justify-content: center;
		align-items: center;
		display: flex;
	}
</style>
