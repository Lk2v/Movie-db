use oracle::{Error, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieShort {
    pub movie_id: i32,
    pub tmdb_id: i32,
    pub title: String,
    pub vote_average: f64,

    pub poster_path: Option<String>,
}

impl MovieShort {
    /// Parse une ligne de résultat SQL en une instance de MovieShort
    pub fn from_row(row: Row) -> Result<Self, Error> {
        Ok(Self {
            movie_id: row.get(0)?, // Colonne 1 : movie_id
            tmdb_id: row.get(1)?, // Colonne 2 : tmdb_id
            title: row.get(2)?, // Colonne 3 : title
            vote_average: row.get(3)?, // Colonne 4 : vote_average
            poster_path: match row.get(4) {
                Ok(path) => Some(path),
                Err(_) => None
            } // Colonne 5 : poster_path
        })
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MovieDetails {
    pub movie_id: i32,
    pub tmdb_id: i32,
    pub title: String,
    pub vote_average: f64,
    pub vote_count: i32,
    pub status: String,
    pub release_date: String,
    pub runtime: i32,
    pub adult: bool,
    pub backdrop_path: Option<String>,

    pub overview: String,
    pub poster_path: String,
    pub genres: String,

    pub keywords: String,
}

impl MovieDetails {
    /// Parse une ligne de résultat SQL en une instance de `Movie`
    pub fn from_row(row: Row) -> Result<Self, Error> {
        Ok(Self {
            movie_id: row.get(0)?, // Colonne 1 : movie_id
            tmdb_id: row.get(1)?, // Colonne 2 : tmdb_id
            title: row.get(2)?, // Colonne 3 : title
            vote_average: row.get(3)?, // Colonne 4 : vote_average
            vote_count: row.get(4)?, // Colonne 5 : vote_count
            status: row.get(5)?, // Colonne 6 : status
            release_date: row.get(6)?, // Colonne 7 : release_date
            runtime: row.get(7)?, // Colonne 8 : runtime    
            adult: match row.get(8)? {
                1 => true,
                0 => false,
                _ => false,
            }, // Colonne 9 : adult
            backdrop_path: match row.get(9) {
                Ok(path) => Some(path),
                Err(_) => None,
            }, // Colonne 10 : backdrop_path
            overview: row.get(10)?, // Colonne 11 : overview
            poster_path: row.get(11)?, // Colonne 12 : poster_path
            genres: row.get(12)?, // Colonne 13 : genres
            keywords: row.get(13)?, // Colonne 14 : keywords
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieRating {
    pub user_id: i32,
    pub rating: f32,
    pub timestamp: i64,
}

impl MovieRating {
    pub fn from_row(row: Row) -> Result<Self, Error> {
        Ok(Self {
            user_id: row.get(0)?,
            rating: row.get(1)?,
            timestamp: row.get(2)?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieTag {
    pub user_id: i32,
    pub tag: String,
    pub timestamp: i64,
}

impl MovieTag {
    pub fn from_row(row: Row) -> Result<Self, Error> {
        Ok(Self {
            user_id: row.get(0)?,
            tag: row.get(1)?,
            timestamp: row.get(2)?,
        })
    }
}


// struct pour encapsuler les données d'un film
#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    pub details: MovieDetails,
    pub ratings: Vec<MovieRating>,
    pub tags: Vec<MovieTag>,
}