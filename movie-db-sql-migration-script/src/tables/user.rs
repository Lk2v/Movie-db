use oracle::Connection;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub user_id: i32,
}

impl User {
    pub fn create_table(conn: &Connection) -> Result<(), oracle::Error> {
        println!("Suppression de la table MovieLens_Users...");
        conn.execute("DROP PUBLIC SYNONYM MovieLens_Users", &[]).ok(); // Ignore l'erreur si le synonyme n'existe pas
        conn.execute("DROP TABLE MovieLens_Users CASCADE CONSTRAINTS", &[]).ok();

        println!("Création de la table MovieLens_Users...");
        conn.execute(
            "CREATE TABLE MovieLens_Users (
                user_id NUMBER PRIMARY KEY
            )
            TABLESPACE movie_db_tbs",
            &[],
        )?;

        // Role GRANT
        conn.execute("GRANT SELECT ON MovieLens_Users TO movie_db_user", &[])?;
        conn.execute("GRANT DELETE ON MovieLens_Users TO movie_db_admin", &[])?;

        // Synonym
        conn.execute("CREATE PUBLIC SYNONYM MovieLens_Users FOR SYSTEM.MovieLens_Users", &[])?;

        Ok(())
    }

    /// Génère la commande SQL pour l'insertion en batch
    pub fn batch_insert_statement() -> String {
        "
        INSERT INTO MovieLens_Users (user_id) VALUES (:1)"
        .to_string()
    }
}
