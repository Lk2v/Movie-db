mod console;
mod sql;

use sql::types::genre::SearchGenre;
use sql::types::movie::{Movie, MovieShort};
use sql::types::search_filter::{SearchFilter};
use sql::types::sql_user::{SqlUser, SqlUserCredentials};
use sql::types::stats::{CountStats, Stats};

use crate::sql::database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  console::state("App", "Starting...");

  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![
    login_user,
    logout_user,
    
    get_logged_username,
    get_current_user,
    
    get_all_movies,
    get_movie,

    delete_movie_lens_user,
    delete_movie_lens_tag,

    get_count_stats,

    create_sql_user,
    get_sql_users,

    delete_sql_user
  ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle()
        .plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command(async)]
async fn login_user(user: SqlUserCredentials) -> Result<bool, String> {
  // Initialise la connexion Oracle au démarrage de l'application
  match database::init_connection(user) {
    Ok(_) => Ok(true),
    Err(e) => Err(e.to_string()),
  }
}

#[tauri::command(async)]
async fn logout_user() -> Result<bool, String> {
  match database::close_connection() {
    Ok(_) => Ok(true),
    Err(e) => Err(e),
  }
}

#[tauri::command(async)]
async fn get_logged_username() -> Result<String, String> {
  let conn = database::get_connection()?;

  Ok(
    database::get_current_sql_username(&conn.as_ref().unwrap())
  )
}

#[tauri::command(async)]
async fn get_current_user() -> Result<SqlUser, String> {
  let conn = database::get_connection()?;
  database::get_current_user_statut(&conn.as_ref().unwrap()).map_err(|e| e.to_string())
}

/// Commande Tauri : Récupérer tous les films
#[tauri::command(async)]
async fn get_all_movies(genre: SearchGenre, query: String, filter: SearchFilter) -> Result<Vec<MovieShort>, String> {

    println!("Genre: {:?}", genre);
    println!("Query: {:?}", query);
    println!("Filter: {:?}", filter);
    
    let conn = database::get_connection()?;
    database::fetch_all_movies(&conn.as_ref().unwrap(), genre, query, filter).map_err(|e| e.to_string())
}

#[tauri::command(async)]
async fn get_movie(id: i32) -> Result<Option<Movie>, String> {
    let conn = database::get_connection()?;
    database::get_movie(&conn.as_ref().unwrap(), id).map_err(|e| e.to_string())
}

#[tauri::command(async)]
async fn get_count_stats() -> Result<Stats, String> {
    let conn = database::get_connection()?;
    database::get_stats(&conn.as_ref().unwrap()).map_err(|e| e.to_string())
}

// Admin Delete Command
#[tauri::command(async)]
async fn delete_movie_lens_user(id: i32) -> Result<(), String> {
  let conn = database::get_connection()?;
  database::delete_movie_lens_user(&conn.as_ref().unwrap(), id).map_err(|e| e.to_string())?;

  Ok(())
}

#[tauri::command(async)]
async fn delete_movie_lens_tag(movie_id: i32, user_id: i32, timestamp: i64) -> Result<(), String> {
  let conn = database::get_connection()?;
  database::delete_movie_lens_tag(&conn.as_ref().unwrap(), movie_id, user_id, timestamp).map_err(|e| e.to_string())?;

  Ok(())
}

// User
#[tauri::command(async)]
async fn create_sql_user(username: String, password: String, is_admin: bool) -> Result<(), String> {
  let conn = database::get_connection()?;
  database::create_sql_user(&conn.as_ref().unwrap(), &username, &password, is_admin).map_err(|e| e.to_string())?;

  Ok(())
}

#[tauri::command(async)]
async fn get_sql_users() -> Result<Vec<SqlUser>, String> {
  let conn = database::get_connection()?;
  database::get_sql_users(&conn.as_ref().unwrap()).map_err(|e| e.to_string())
}

#[tauri::command(async)]
async fn delete_sql_user(username: String) -> Result<(), String> {
  let conn = database::get_connection()?;
  database::delete_sql_user(&conn.as_ref().unwrap(), &username).map_err(|e| e.to_string())?;

  Ok(())
}