interface MovieShort {
    movie_id: number;
    tmdb_id: number;
    title: string;
    vote_average: number;
    poster_path: string;
    popularity: number;
}

interface MovieDetails extends MovieShort {
    vote_average: number,
    vote_count: number,
    status: string,
    release_date: string,
    runtime: number,
    adult: boolean,
    backdrop_path: string,
    overview: string,
    genres: string,
    keywords: string,
}

interface MovieRating {
    user_id: number;
    rating: number;
    timestamp: string;
}

interface MovieTag {
    user_id: number;
    tag: string;
    timestamp: number;
}

interface Movie {
    details: MovieDetails;
    ratings: MovieRating[];
    tags: MovieTag[];
}

function getPosterPath(path: string) {
    return `https://image.tmdb.org/t/p/w500/${path}`;
}

function getBackdropPath(path: string) {
    return `https://image.tmdb.org/t/p/w1280/${path}`;
}

export {
    type MovieDetails,
    type MovieShort,
    type MovieRating,
    type MovieTag,
    type Movie,

    getPosterPath,
    getBackdropPath,
}