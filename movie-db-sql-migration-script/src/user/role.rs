use oracle::Connection;

pub fn create_roles(conn: &Connection) -> Result<(), oracle::Error> {
    println!("Suppression des rôles...");

    // Suppression des rôles si ils existent
    conn.execute("DROP ROLE movie_db_user", &[]).ok();
    conn.execute("DROP ROLE movie_db_admin", &[]).ok();

    
    println!("Création des rôles...");

    // Création du rôle pour les utilisateurs ayant accès en lecture
    conn.execute("CREATE ROLE movie_db_user", &[])?;

    // Création du rôle pour les utilisateurs ayant accès en écriture
    conn.execute("CREATE ROLE movie_db_admin", &[])?;
    conn.execute("GRANT CREATE USER, ALTER USER, DROP USER, GRANT ANY ROLE TO movie_db_admin", &[])?;
    

    conn.commit()?;
    Ok(())
}