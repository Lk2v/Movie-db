use oracle::Connection;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Tag {
    pub user_id: i32,
    pub movie_id: i32,
    pub tag: String,
    pub timestamp: i64,
}

impl Tag {
    /// Crée la table `MovieLens_Tags` dans la base de données
    pub fn create_table(conn: &Connection) -> Result<(), oracle::Error> {
        println!("Suppression de la table MovieLens_Tags...");
        conn.execute("DROP PUBLIC SYNONYM MovieLens_Tags", &[]).ok(); // Ignore l'erreur si le synonyme n'existe pas
        conn.execute("DROP TABLE MovieLens_Tags CASCADE CONSTRAINTS", &[]).ok();

        println!("Création de la table MovieLens_Tags...");
        conn.execute(
            "CREATE TABLE MovieLens_Tags (
                user_id NUMBER,
                movie_id NUMBER,
                tag VARCHAR2(255),
                timestamp NUMBER
            )
            TABLESPACE movie_db_tbs",
            &[],
        )?;

        // Index
        conn.execute("CREATE INDEX MovieLens_Tags_user_id_idx ON MovieLens_Tags(user_id)", &[])?;
        conn.execute("CREATE INDEX MovieLens_Tags_movie_id_idx ON MovieLens_Tags(movie_id)", &[])?;

        // Role GRANT
        conn.execute("GRANT SELECT ON MovieLens_Tags TO movie_db_user", &[])?;
        conn.execute("GRANT SELECT, INSERT, UPDATE, DELETE ON MovieLens_Tags TO movie_db_admin", &[])?;

        // Synonym
        conn.execute("CREATE PUBLIC SYNONYM MovieLens_Tags FOR SYSTEM.MovieLens_Tags", &[])?;
        Ok(())
    }

    /// Génère la commande SQL pour l'insertion en batch
    pub fn batch_insert_statement() -> String {
        "
        INSERT INTO MovieLens_Tags (user_id, movie_id, tag, timestamp) VALUES (:1, :2, :3, :4)"
        .to_string()
    }
}