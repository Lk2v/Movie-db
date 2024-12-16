use oracle::Connection;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Link {
    pub movie_id: i32,
    pub imdb_id: String,
    pub tmdb_id: Option<i32>, // Certains tmdbId peuvent être null
}

impl Link {
    pub fn create_table(conn: &Connection) -> Result<(), oracle::Error> {
        println!("Suppression de la table MovieLens_Links...");
        conn.execute("DROP PUBLIC SYNONYM MovieLens_Links", &[]).ok(); // Ignore l'erreur si le synonyme n'existe pas
        conn.execute("DROP TABLE MovieLens_Links CASCADE CONSTRAINTS", &[]).ok();

        println!("Création de la table MovieLens_Links...");
        conn.execute(
            "CREATE TABLE MovieLens_Links (
                movie_id NUMBER PRIMARY KEY,
                imdb_id VARCHAR2(20),
                tmdb_id NUMBER
            ) 
            TABLESPACE movie_db_tbs",
            // CONSTRAINT fk_tmdb_id FOREIGN KEY (tmdb_id) REFERENCES TMDB_movie_dataset (id) ON DELETE SET NULL
            &[],
        )?;

        // Index
        conn.execute("CREATE INDEX MovieLens_Links_tmdb_id_idx ON MovieLens_Links(tmdb_id)", &[])?;

        // Role GRANT
        conn.execute("GRANT SELECT ON MovieLens_Links TO movie_db_user", &[])?;
        conn.execute("GRANT SELECT, INSERT, UPDATE, DELETE ON MovieLens_Links TO movie_db_admin", &[])?;

        // Synonym
        conn.execute("CREATE PUBLIC SYNONYM MovieLens_Links FOR SYSTEM.MovieLens_Links", &[])?;

        Ok(())
    }

    /// Génère la commande SQL pour l'insertion en batch
    pub fn batch_insert_statement() -> String {
        "
        INSERT INTO MovieLens_Links (movie_id, imdb_id, tmdb_id) VALUES (:1, :2, :3)"
        .to_string()
    }
}
