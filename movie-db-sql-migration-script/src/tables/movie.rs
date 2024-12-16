use oracle::{sql_type::ToSql, Connection};
use serde::Deserialize;

/// Fonction pour désérialiser un booléen à partir d'une chaîne
fn parse_bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.trim().to_lowercase().as_str() {
        "true" | "1" => Ok(true),
        "false" | "0" => Ok(false),
        _ => Ok(false), // Valeur par défaut si la donnée est incorrecte
    }
}

#[derive(Debug, Deserialize)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub vote_average: f64,
    pub vote_count: i32,
    pub status: String,
    pub release_date: String,
    pub revenue: i64,
    pub runtime: i32,

    #[serde(deserialize_with = "parse_bool_from_string")]
    pub adult: bool,
    pub backdrop_path: String,
    pub budget: i64,
    pub homepage: Option<String>,
    pub imdb_id: String,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: String,
    pub tagline: Option<String>,
    pub genres: String,
    pub production_companies: String,
    pub production_countries: String,
    pub spoken_languages: String,
    pub keywords: String,
}

impl Movie {
    pub fn create_table(conn: &Connection) -> Result<(), oracle::Error> {
        println!("Suppression de la table TMDB_movie_dataset...");
        
        conn.execute("DROP PUBLIC SYNONYM TMDB_movie_dataset", &[]).ok(); // Ignore l'erreur si le synonyme n'existe pas
        conn.execute("DROP TABLE TMDB_movie_dataset CASCADE CONSTRAINTS", &[]).ok(); // Ignore l'erreur si la table n'existe pas

        println!("Création de la table TMDB_movie_dataset...");
        conn.execute(
            "
        CREATE TABLE TMDB_movie_dataset (
            id NUMBER PRIMARY KEY,
            title VARCHAR2(768),
            vote_average NUMBER,
            vote_count NUMBER,
            status VARCHAR2(50),
            release_date DATE,
            revenue NUMBER,
            runtime NUMBER,
            adult NUMBER(1) CHECK (adult IN (0, 1)),
            backdrop_path VARCHAR2(255),
            budget NUMBER,
            homepage VARCHAR2(1024),
            imdb_id VARCHAR2(50),
            original_language VARCHAR2(10),
            original_title VARCHAR2(768),
            overview CLOB,
            popularity NUMBER,
            poster_path VARCHAR2(255),
            tagline VARCHAR2(510),
            genres CLOB,
            production_companies CLOB,
            production_countries CLOB,
            spoken_languages CLOB,
            keywords CLOB
        ) 
        TABLESPACE movie_db_tbs",
            &[],
        )?;
        
        // Création de l'index sur le titre
        conn.execute("CREATE INDEX TMDB_movie_dataset_title_idx ON TMDB_movie_dataset(title)", &[])?;

        // Role GRANT
        conn.execute("GRANT SELECT ON TMDB_movie_dataset TO movie_db_user", &[])?;
        conn.execute("GRANT SELECT, INSERT, UPDATE, DELETE ON TMDB_movie_dataset TO movie_db_admin", &[])?;

        // Synonyme
        conn.execute("CREATE PUBLIC SYNONYM TMDB_movie_dataset FOR SYSTEM.TMDB_movie_dataset", &[])?;

        Ok(())
    }

    /// Génère la oommande SQL pour l'insertion en batch
    pub fn batch_insert_statement() -> String {
        "
        INSERT INTO TMDB_movie_dataset (
            id, title, vote_average, vote_count, status, release_date, revenue,
            runtime, adult, backdrop_path, budget, homepage, imdb_id,
            original_language, original_title, overview, popularity,
            poster_path, tagline, genres, production_companies,
            production_countries, spoken_languages, keywords
        ) VALUES (
            :1, :2, :3, :4, :5, TO_DATE(:6, 'YYYY-MM-DD'), :7, :8, :9, :10,
            :11, :12, :13, :14, :15, :16, :17, :18, :19, :20, :21, :22, :23, :24
        )"
        .to_string()
    }
}
