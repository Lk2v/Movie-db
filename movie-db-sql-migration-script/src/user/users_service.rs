use oracle::Connection;

pub fn create_users_service(conn: &Connection) -> Result<(), oracle::Error> {

    // CREATE USER SERVICE
    println!("Création du service de création d'utilisateur...");

    // Service de création d'utilisateur
    conn.execute("CREATE OR REPLACE PROCEDURE CreateUser(
        username IN VARCHAR2,
        password IN VARCHAR2,
        is_admin IN NUMBER
    ) AUTHID CURRENT_USER AS
    BEGIN
        -- Créer l'utilisateur avec son mot de passe et son tablespace par défaut
        EXECUTE IMMEDIATE 'CREATE USER ' || username || 
                        ' IDENTIFIED BY ' || password ||
                        ' DEFAULT TABLESPACE movie_db_tbs ' ||
                        ' QUOTA UNLIMITED ON movie_db_tbs';

        -- Corriger le bug Oracle si nécessaire
        EXECUTE IMMEDIATE 'alter session set container=XEPDB1';

        -- Attribuer le rôle de base à tous les utilisateurs
        EXECUTE IMMEDIATE 'GRANT movie_db_user TO ' || username;

        -- Si l'utilisateur est admin, attribuer également le rôle admin
        IF is_admin = 1 THEN
            EXECUTE IMMEDIATE 'GRANT CREATE USER, ALTER USER, DROP USER, GRANT ANY ROLE, GRANT ANY PRIVILEGE TO ' || username || ' WITH ADMIN OPTION';

            EXECUTE IMMEDIATE 'GRANT movie_db_admin TO ' || username;

            -- Définir les deux rôles comme rôles par défaut
            EXECUTE IMMEDIATE 'ALTER USER ' || username || ' DEFAULT ROLE movie_db_user, movie_db_admin';
        ELSE
            -- Sinon, ne définir que le rôle de base
            EXECUTE IMMEDIATE 'ALTER USER ' || username || ' DEFAULT ROLE movie_db_user';
        END IF;
        
        -- Attribuer les permissions de base
        EXECUTE IMMEDIATE 'GRANT CONNECT, CREATE SESSION TO ' || username;
        
        -- Enregistrer l'utilisateur dans la table UserAudit
        INSERT INTO USERS_RECORDS (username, is_admin)
            VALUES (username, is_admin);

        -- Commit pour sauvegarder l'enregistrement
        COMMIT;
                
        -- Afficher un message pour indiquer que l'utilisateur a été créé
        DBMS_OUTPUT.PUT_LINE('Utilisateur ' || username || ' créé avec succès.');
    END;", &[])?;

    // Role GRANT
    conn.execute("GRANT EXECUTE ON CreateUser TO movie_db_admin", &[])?;

    // Synonym
    conn.execute("DROP PUBLIC SYNONYM CreateUser", &[]).ok(); // Ignore l'erreur si le synonyme n'existe pas
    conn.execute("CREATE PUBLIC SYNONYM CreateUser FOR SYSTEM.CreateUser", &[])?;
    

    // DELETE USER SERVICE
    println!("Création du service de suppression d'utilisateur...");

    // Service de suppression d'utilisateur
    conn.execute("CREATE OR REPLACE PROCEDURE DeleteUser(
        u_name IN VARCHAR2
    ) AUTHID CURRENT_USER AS
    BEGIN
        BEGIN
            -- Supprimer l'utilisateur
            EXECUTE IMMEDIATE 'DROP USER ' || u_name || ' CASCADE';
            DBMS_OUTPUT.PUT_LINE('Utilisateur ' || u_name || ' supprimé avec succès.');
        EXCEPTION
            WHEN OTHERS THEN
                -- Lever une exception en cas d'échec
                RAISE_APPLICATION_ERROR(
                    -20001,
                    'Erreur lors de la suppression de l''utilisateur ' || u_name || ': ' || SQLERRM
                );
        END;

        -- Supprimer l'utilisateur de la table Users_Records
        DELETE FROM USERS_RECORDS WHERE UPPER(username) = UPPER(u_name);

        -- Commit des changements
        COMMIT;
    END;", &[])?;

    // Role GRANT
    conn.execute("GRANT EXECUTE ON DeleteUser TO movie_db_admin", &[])?;

    // Synonym
    conn.execute("DROP PUBLIC SYNONYM DeleteUser", &[]).ok(); // Ignore l'erreur si le synonyme n'existe pas
    conn.execute("CREATE PUBLIC SYNONYM DeleteUser FOR SYSTEM.DeleteUser", &[])?;

    conn.commit()?;

    Ok(())
}

pub fn create_users_records_table(conn: &Connection) -> Result<(), oracle::Error> {
    //println!("Suppression de la table Users_Records...");
    conn.execute("DROP PUBLIC SYNONYM Users_Records", &[]).ok(); // Ignore l'erreur si le synonyme n'existe pas

    conn.execute("DROP TABLE Users_Records CASCADE CONSTRAINTS", &[]).ok(); // Ignore l'erreur si la table n'existe pas

    println!("Création de la table Users_Records...");

    conn.execute(
        "CREATE TABLE Users_Records (
                username VARCHAR2(128) PRIMARY KEY,
                is_admin NUMBER(1) NOT NULL CHECK (is_admin IN (0, 1)),
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
            TABLESPACE movie_db_tbs",
        &[],
    ).ok(); // Ignore l'erreur si la table existe déjà

    // Role GRANT
    conn.execute("GRANT SELECT, INSERT, DELETE ON Users_Records TO movie_db_admin", &[])?; // Ignore l'erreur si le rôle a déjà été attribué

    // Synonym
    conn.execute(
        "CREATE PUBLIC SYNONYM Users_Records FOR SYSTEM.Users_Records",
        &[],
    ).ok(); // Ignore l'erreur si le synonyme existe déjà

    conn.commit()?;
    Ok(())
}


pub fn create_user(conn: &Connection, username: &str, password: &str, is_admin: bool) -> Result<(), oracle::Error> {
    println!("Suppression de l'utilisateur {}...", username);
    match conn.execute(&format!("DROP USER {} CASCADE", username), &[]) {
        Ok(_) => {
            println!("Utilisateur {} supprimé avec succès.", username);
        }
        Err(err) => {
            eprintln!("Erreur lors de la suppression de l'utilisateur {}: {}", username, err);
        }
    }
    
    println!("Création de l'utilisateur {}...", username);
    match conn.execute(
        "BEGIN
            CreateUser(:username, :password, :is_admin);
        END;",
        &[
            &username,
            &password,
            &(if is_admin { 1 } else { 0 }),
        ],
    ) {
        Ok(_) => {
            println!("Utilisateur {} créé avec succès.", username);
        }
        Err(err) => {
            eprintln!("Erreur lors de la création de l'utilisateur {}: {}", username, err);
        }
    }

    conn.commit()?;
    Ok(())
}