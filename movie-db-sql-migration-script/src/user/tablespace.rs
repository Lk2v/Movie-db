use oracle::Connection;
use rand::Rng;


pub fn create_tablespace(conn: &Connection) -> Result<(), oracle::Error> {

    println!("Suppression du tablespace movie_db_tbs...");
    match conn.execute("DROP TABLESPACE movie_db_tbs INCLUDING CONTENTS AND DATAFILES CASCADE CONSTRAINTS", &[]) {
        Ok(_) => {
            println!("Tablespace movie_db_tbs supprimé avec succès.");
        }
        Err(err) => {
            println!("Erreur lors de la suppression du tablespace movie_db_tbs: {:?}", err);
        }
    }

    let random_int: u32 = rand::thread_rng().gen_range(1..10000);
    let tablespace_name = format!("movie_db_tbs_{}", random_int);

    println!("Création du tablespace {}...", tablespace_name);

    conn.execute(&format!("
        CREATE TABLESPACE movie_db_tbs
            DATAFILE '/opt/oracle/oradata/{}.dbf'
            SIZE 100M
            AUTOEXTEND ON
            NEXT 10M
            MAXSIZE UNLIMITED
    ", tablespace_name), &[])?;

    conn.commit()?;

    Ok(())
}