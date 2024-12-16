use oracle::Connection;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rating {
    pub user_id: i32,
    pub movie_id: i32,
    pub rating: f32,
    pub timestamp: i64,
}

impl Rating {
    pub fn create_table(conn: &Connection) -> Result<(), oracle::Error> {
        println!("Suppression de la table MovieLens_Ratings...");
        conn.execute("DROP PUBLIC SYNONYM MovieLens_Ratings", &[]).ok(); // Ignore l'erreur si le synonyme n'existe pas
        conn.execute("DROP TABLE MovieLens_Ratings CASCADE CONSTRAINTS", &[]).ok();

        println!("Création de la table MovieLens_Ratings...");
        conn.execute(
            "CREATE TABLE MovieLens_Ratings (
                user_id NUMBER,
                movie_id NUMBER,
                rating NUMBER(2, 1),
                timestamp NUMBER
            )
            TABLESPACE movie_db_tbs",
            &[],
        )?;

        // Index
        conn.execute("CREATE INDEX MovieLens_Ratings_user_id_idx ON MovieLens_Ratings(user_id)", &[])?;
        conn.execute("CREATE INDEX MovieLens_Ratings_movie_id_idx ON MovieLens_Ratings(movie_id)", &[])?;

        // Role GRANT
        conn.execute("GRANT SELECT ON MovieLens_Ratings TO movie_db_user", &[])?;
        conn.execute("GRANT SELECT, INSERT, UPDATE, DELETE ON MovieLens_Ratings TO movie_db_admin", &[])?;

        // Synonym
        conn.execute("CREATE PUBLIC SYNONYM MovieLens_Ratings FOR SYSTEM.MovieLens_Ratings", &[])?;

        Ok(())
    }

    /// Génère la commande SQL pour l'insertion en batch
    pub fn batch_insert_statement() -> String {
        "
        INSERT INTO MovieLens_Ratings (user_id, movie_id, rating, timestamp) VALUES (:1, :2, :3, :4)"
        .to_string()
    }
}
