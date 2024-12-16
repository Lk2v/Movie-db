use std::string;

use oracle::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub count: CountStats,
    pub top_users: Vec<TopUserItem>,
    pub top_profits_movies: Vec<TopMovieProfit>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieGenreCountStats {
    pub genre_name: String,
    pub genre_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountStats {
    pub total_movies: u32,
    pub total_users: u32,
    pub total_ratings: u32,
    pub total_tags: u32,

    pub genre_count: Vec<MovieGenreCountStats>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopUserItem {
    pub user_id: i32,
    pub num_ratings: i32,
    pub num_tags: i32,
}

impl TopUserItem {
    pub fn from_row(row: &Row) -> Result<TopUserItem, oracle::Error> {
        Ok(TopUserItem {
            user_id: row.get(0)?,
            num_ratings: row.get(1)?,
            num_tags: row.get(2)?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopMovieProfit {
    pub movie_id: i32, 
    pub title: String, 
    pub poster_path: String, 
    pub profit: i64,
}

impl TopMovieProfit {
    pub fn from_row(row: &Row) -> Result<TopMovieProfit, oracle::Error> {
        Ok(TopMovieProfit {
            movie_id: row.get(0)?,
            title: row.get(1)?,
            poster_path: row.get(2)?,
            profit: row.get(3)?,
        })
    }
}