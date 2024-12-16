import { invoke } from "@tauri-apps/api/core";
import type { Movie, MovieShort } from "./movie";
import { filter_to_string, type SearchFilter } from "./search";
import type { Stats } from "./stats";
import type { SqlUserList, SqlUserProps } from "./sql_user";


async function getAllMovies(genre: string, query: string, filter: SearchFilter) {
    console.log(`Search args: {
        genre: ${genre},
        query: ${query},
        filter: ${filter_to_string(filter)}
    }`);
    
    return await invoke<MovieShort[]>("get_all_movies", {
        genre,
        query,
        filter: filter_to_string(filter),
    });
}

async function getMovie(movie_id: string) {
    const movie_id_number = parseInt(movie_id);
    return await invoke<Movie>("get_movie", { id: movie_id_number });
}

async function getStats(): Promise<Stats> {
    return await invoke<Stats>("get_count_stats");
}


// Users
async function createSqlUser(user: SqlUserProps) {
    return await invoke("create_sql_user", {
        username: user.username,
        password: user.password,
        isAdmin: user.is_admin,
    });
}

async function getSqlUsers(){
    return await invoke<SqlUserList>("get_sql_users");
}

async function deleteSqlUser(username: string){
    return await invoke("delete_sql_user", { username });
}

async function deleteMovieLensUser(id: number){
    return await invoke("delete_movie_lens_user", { id });
}

async function deleteMovieLensTag(movieId: number, userId: number, timestamp: number){
    return await invoke("delete_movie_lens_tag", { movieId, userId, timestamp });
}

export {
    getAllMovies,
    getMovie,

    getStats,

    createSqlUser,
    getSqlUsers,
    deleteSqlUser,

    deleteMovieLensUser,
    deleteMovieLensTag,
}