use oracle::{sql_type::OracleType, Connection};
use std::sync::Mutex;

use crate::console;

use super::{
    queries::{
        CREATE_SQL_USER, DELETE_MOVIE_LENS_TAG, DELETE_MOVIE_LENS_USER, DELETE_SQL_USER, GET_CURRENT_SQL_USERNAME, GET_CURRENT_USER, GET_SQL_USERS, GET_STATS, SELECT_MOVIE_BY_ID, SELECT_MOVIE_RATINGS, SELECT_MOVIE_TAGS, SELECT_SHORT_MOVIES
    },
    types::{
        genre::SearchGenre,
        movie::{Movie, MovieDetails, MovieRating, MovieShort, MovieTag},
        search_filter::SearchFilter,
        sql_user::{SqlUser, SqlUserCredentials}, stats::{CountStats, MovieGenreCountStats, Stats, TopMovieProfit, TopUserItem},
    },
};

// Configuration de la base de données
const HOST: &str = "//localhost:1521/XEPDB1";

// Un wrapper Mutex pour gérer une connexion partagée
lazy_static::lazy_static! {
    static ref ORACLE_CONNECTION: Mutex<Option<Connection>> = Mutex::new(None);
}

/// Initialise la connexion Oracle
pub fn init_connection(user: SqlUserCredentials) -> Result<(), oracle::Error> {
    console::state("Db", "Connecting...");
    let mut conn = ORACLE_CONNECTION.lock().unwrap();
    if conn.is_none() {
        *conn = Some(Connection::connect(user.username, user.password, HOST)?);
        console::success("Db", "Connected");
    }
    Ok(())
}

pub fn close_connection() -> Result<(), String> {
    let mut conn = ORACLE_CONNECTION.lock().unwrap();
    if let Some(c) = conn.as_mut() {
        c.close().unwrap();
        *conn = None;
        console::success("Db", "Connection closed");
    }
    Ok(())
}

/// Obtenir une référence à la connexion Oracle existante
pub fn get_connection() -> Result<std::sync::MutexGuard<'static, Option<Connection>>, String> {
    let conn = ORACLE_CONNECTION
        .lock()
        .map_err(|_| "Erreur lors de l'accès à la connexion".to_string())?;
    if conn.is_none() {
        Err("Connexion non initialisée. Appelez `init_connection`.".to_string())
    } else {
        Ok(conn)
    }
}

pub fn get_current_sql_username(conn: &Connection) -> String {
    let mut stmt = conn.query(GET_CURRENT_SQL_USERNAME, &[]).unwrap();
    match stmt.next() {
        Some(row) => {
            // print row
            println!("{:?}", row);
            let user: String = row.unwrap().get(0).unwrap();
            user
        }
        None => "Unknown".to_string(),
    }
}

pub fn fetch_all_movies(
    conn: &Connection,
    genre: SearchGenre,
    query: String,
    filter: SearchFilter,
) -> Result<Vec<MovieShort>, oracle::Error> {
    let search_query = format!("%{}%", query.to_lowercase());

    // Préparer la clause genre conditionnellement
    let genre_filter = if genre != SearchGenre::All {
        "AND DBMS_LOB.INSTR(genres, :2) > 0"
    } else {
        ""
    };

    // Construire la requête complète
    let sql_query = SELECT_SHORT_MOVIES
        .replace("{GENRE_FILTER}", genre_filter)
        .replace("{VIEW_NAME}", filter.to_view_name());

    // Préparer les paramètres
    let mut params: Vec<&dyn oracle::sql_type::ToSql> = vec![&search_query];

    let genre_string;
    if let Some(g) = genre.as_str() {
        genre_string = g.to_string();

        params.push(&genre_string);
    }

    // Exécuter la requête
    let rows = conn.query(&sql_query, &params)?;

    let mut movies = Vec::new();
    for row in rows {
        movies.push(MovieShort::from_row(row?)?);
    }

    Ok(movies)
}

pub fn get_movie_details(
    conn: &Connection,
    movie_id: i32,
) -> Result<Option<MovieDetails>, oracle::Error> {
    let mut stmt = conn.query(SELECT_MOVIE_BY_ID, &[&movie_id])?;

    match stmt.next() {
        Some(row) => Ok(Some(MovieDetails::from_row(row?)?)),
        None => Ok(None),
    }
}

pub fn get_movie_ratings(
    conn: &Connection,
    movie_id: i32,
) -> Result<Vec<MovieRating>, oracle::Error> {
    let rows = conn.query(SELECT_MOVIE_RATINGS, &[&movie_id])?;
    let mut ratings = Vec::new();
    for row in rows {
        ratings.push(MovieRating::from_row(row?)?);
    }
    Ok(ratings)
}

pub fn get_movie_tags(conn: &Connection, movie_id: i32) -> Result<Vec<MovieTag>, oracle::Error> {
    let rows = conn.query(SELECT_MOVIE_TAGS, &[&movie_id])?;
    let mut tags = Vec::new();
    for row in rows {
        tags.push(MovieTag::from_row(row?)?);
    }
    Ok(tags)
}

pub fn get_movie(conn: &Connection, movie_id: i32) -> Result<Option<Movie>, oracle::Error> {
    let details = match get_movie_details(conn, movie_id) {
        Ok(Some(details)) => details,
        Err(e) => {
            println!("Error fetching movie details: {}", e);
            return Ok(None);
        }
        _ => return Ok(None),
    };

    let ratings = get_movie_ratings(conn, movie_id)?;
    let tags = get_movie_tags(conn, movie_id)?;

    Ok(Some(Movie {
        details,
        ratings,
        tags,
    }))
}

/* MOVIE-LENS ADMIN DELETE */
pub fn delete_movie_lens_user(conn: &Connection, user_id: i32) -> Result<(), oracle::Error> {
    let _ = conn.execute(DELETE_MOVIE_LENS_USER,
        &[&user_id],
    )?;

    Ok(())
}

pub fn delete_movie_lens_tag(conn: &Connection, movie_id: i32, user_id: i32, timestamp: i64) -> Result<(), oracle::Error> {
    let _ = conn.execute(DELETE_MOVIE_LENS_TAG,
        &[
            &movie_id,
            &user_id,
            &timestamp,
        ],
    )?;

    Ok(())
}

/* STATS */
pub fn get_stats(conn: &Connection) -> Result<Stats, oracle::Error> {
    let mut stmt = conn.statement(GET_STATS).build()?; 

    stmt.execute(&[
        // number
        &OracleType::Number(0, 0),
        &OracleType::Number(0, 0),
        &OracleType::Number(0, 0),
        &OracleType::Number(0, 0),
    ])?;

    let total_movies: u32 = stmt.returned_values("1")?[0];
    let total_ratings: u32 = stmt.returned_values("2")?[0];
    let total_tags: u32 = stmt.returned_values("3")?[0];
    let distinct_users: u32 = stmt.returned_values("4")?[0]; 
    
    // Get genre count
    let mut genre_count: Vec<MovieGenreCountStats> = Vec::new();
    if let Some(mut cursor) = stmt.implicit_result()? {
        let mut res = cursor.query()?;

        while let Some(row) = res.next() {
            if let Ok(row) = row {
                let genre_name: String = match row.get(0) {
                    Ok(genre) => genre,
                    Err(_) => {
                        continue;
                    }
                };

                let genre_movie_count: u32 = row.get(1)?;

                genre_count.push(MovieGenreCountStats {
                    genre_name,
                    genre_count: genre_movie_count,
                });
            }
        }
    }

    // Get top users
    let mut top_users: Vec<TopUserItem> = Vec::new();

    if let Some(mut cursor) = stmt.implicit_result()? {
        let mut res = cursor.query()?;

        while let Some(row) = res.next() {
            if let Ok(row) = row {
                top_users.push(TopUserItem::from_row(&row)?);
            }
        }
    } else {
        println!("No implicit result");
    }

    // Get top profits films
    let mut top_profits_movies: Vec<TopMovieProfit> = Vec::new();

    if let Some(mut cursor) = stmt.implicit_result()? {
        let mut res = cursor.query()?;

        while let Some(row) = res.next() {
            if let Ok(row) = row {
                top_profits_movies.push(TopMovieProfit::from_row(&row)?);
            }
        }
    } else {
        println!("No implicit result");
    }

    // Retourner les résultats sous forme d'un tuple
    Ok(Stats {
        count: CountStats {
            total_movies,
            total_ratings,
            total_tags,
            total_users: distinct_users,
            genre_count,
        },
        top_users,
        top_profits_movies,
    })
}


/* USER */
pub fn get_current_user_statut(conn: &Connection) -> Result<SqlUser, oracle::Error> {
    let mut stmt = conn.statement(GET_CURRENT_USER).build()?; 

    stmt.execute(&[
        // number
        &OracleType::Varchar2(255),
        &OracleType::Number(0, 0),
    ])?;
    
    let username: String = stmt.returned_values::<_, String>("1")?[0].to_string();
    let isAdmin: u8 = stmt.returned_values("2")?[0];

    Ok(SqlUser {
        username,
        is_admin: isAdmin == 1,
        created_at: "".to_string(),
    })
}

pub fn create_sql_user(conn: &Connection, username: &str, password: &str, is_admin: bool) -> Result<(), oracle::Error> {
    let _ = conn.execute(
        CREATE_SQL_USER,
        &[
            &username, 
            &password, 
            if is_admin { &1 } else { &0 },
        ],
    )?;

    Ok(())
}

pub fn delete_sql_user(conn: &Connection, username: &str) -> Result<(), oracle::Error> {
    let _ = conn.execute(DELETE_SQL_USER,
        &[&username]
    )?;

    Ok(())
}

pub fn get_sql_users(conn: &Connection) -> Result<Vec<SqlUser>, oracle::Error> {
    let rows = conn.query(GET_SQL_USERS, &[])?;

    let mut users: Vec<SqlUser> = Vec::new();

    for row in rows {
        users.push(SqlUser::from_row(&row?)?);
    }

    Ok(users)
}