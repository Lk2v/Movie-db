
interface MovieGenreCount {
    genre_name: string,
    genre_count: number,
}

type MovieGenreCountStats = MovieGenreCount[];

interface CountStats {
    total_movies: number;
    total_users: number;
    total_ratings: number;
    total_tags: number;

    genre_count: MovieGenreCountStats;
}


interface TopUserItem {
    user_id: number;
    num_ratings: number;
    num_tags: number;
}

interface TopMovieProfit {
    movie_id: number, 
    title: string, 
    poster_path: string, 
    profit: number,
}

interface Stats {
    count: CountStats,
    top_users: Array<TopUserItem>,
    top_profits_movies: Array<TopMovieProfit>,
}

export {
    type MovieGenreCount,
    type MovieGenreCountStats,
    type CountStats,
    type TopUserItem,
    type TopMovieProfit,
    type Stats,
}