pub const SELECT_SHORT_MOVIES: &str = "
    SELECT movie_id, tmdb_id, title, vote_average, poster_path
    FROM {VIEW_NAME}
    WHERE LOWER(title) LIKE :1
    {GENRE_FILTER}
    FETCH FIRST 100 ROWS ONLY
";

pub const GET_CURRENT_SQL_USERNAME: &str = "
    select user from dual
";

// Récupérer les détails d'un film par son ID MovieLens
pub const SELECT_MOVIE_BY_ID: &str = "SELECT 
    movie_id,
    tmdb_id,
    title,
    vote_average,
    vote_count,
    status,
    TO_CHAR(release_date, 'YYYY-MM-DD') AS release_date,
    runtime,
    adult,
    backdrop_path,
    overview,
    poster_path,
    genres,
    keywords
FROM 
    MovieDetailsView
WHERE 
    movie_id = :1";

pub const SELECT_MOVIE_RATINGS: &str = "SELECT 
    user_id, 
    rating, 
    timestamp
FROM 
    MovieLens_Ratings
WHERE 
    movie_id = :1";

pub const SELECT_MOVIE_TAGS: &str = "SELECT 
    user_id, 
    tag, 
    timestamp
FROM 
    MovieLens_Tags
WHERE 
    movie_id = :1";

/* MOVIE-LENS ADMIN DELETE */
pub const DELETE_MOVIE_LENS_USER: &str = "
    BEGIN
        -- Arguments: user_id
        DeleteMovieLensUser(:1);
    END;
";

pub const DELETE_MOVIE_LENS_TAG: &str = "
    BEGIN
        -- Arguments: movie_id, user_id, timestamp
        DeleteMovieLensTag(:1, :2, :3);
    END;
";

/* STATS */
pub const GET_STATS: &str = "
    DECLARE
        genre_count SYS_REFCURSOR;
        top_users SYS_REFCURSOR;
        top_profits_films SYS_REFCURSOR;
    BEGIN
        GetStats(:1, :2, :3, :4, genre_count, top_users, top_profits_films);
        DBMS_SQL.RETURN_RESULT(genre_count);
        DBMS_SQL.RETURN_RESULT(top_users);
        DBMS_SQL.RETURN_RESULT(top_profits_films);
    END;
";


/* USER */
pub const GET_CURRENT_USER : &str = "
    BEGIN
        GetCurrentUserInfo(:1, :2);
    END;
";

pub const CREATE_SQL_USER: &str = "
    BEGIN
        /*
        * Arguments:
        * 1: username
        * 2: password
        * 3: is_admin (0 or 1)
        */
        CreateUser(:1, :2, :3);
    END;
";

pub const DELETE_SQL_USER: &str = "
    BEGIN
        /*
        * Arguments:
        * 1: username
        */
        DeleteUser(:1);
    END;
";

pub const GET_SQL_USERS: &str = "
    SELECT
        username, 
        is_admin,
        created_at
    FROM 
        USERS_RECORDS
    ORDER BY 
        created_at DESC";