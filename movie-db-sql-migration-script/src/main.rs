mod tables;
mod user;

use std::{collections::HashSet, env, fs::File};
use csv::ReaderBuilder;

use oracle::Connection;
use tables::{link::Link, movie::Movie, rating::Rating, tag::Tag, user::User};
use user::{role::create_roles, tablespace::create_tablespace, users_service::{create_user, create_users_records_table, create_users_service}};

const DATASET_FOLDER: &str = "./dataset";

fn main() -> Result<(), oracle::Error> {
    // Charger les variables d'environnement
    dotenv::dotenv().ok();

    // Récupérer les variables d'environnement
    let username = env::var("USERNAME").expect("USERNAME is not set in .env file");
    let password = env::var("PASSWORD").expect("PASSWORD is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");

    // Connexion à la base Oracle
    println!("Connexion à la base de données avec l'utilisateur {}...", username);

    let conn = Connection::connect(username, password, host)
        .expect("Impossible de se connecter à la base de données");

    // Création du tablespace
    create_tablespace(&conn)?;

    // Creation des roles
    create_roles(&conn)?;

    // Creation records user
    create_users_records_table(&conn)?;
    create_users_service(&conn)?;

    // Création de l'utilisateur
    create_user(&conn, "admin", "pass", true)?;
    create_user(&conn, "spectator", "pass", false)?;
    

    // Création des tables
    let mut seen_users: HashSet<i32> = HashSet::new();

    // Movie table
    Movie::create_table(&conn)?;
    insert_movies(&conn)?;

    // Rating table
    Rating::create_table(&conn)?;
    insert_ratings(&conn, &mut seen_users)?;

    // Tag table
    Tag::create_table(&conn)?;
    insert_tags(&conn, &mut seen_users)?;

    // Users table
    User::create_table(&conn)?;
    insert_users(&conn, &seen_users);
    
    // Link table
    Link::create_table(&conn)?;
    insert_links(&conn)?;
    
    create_triggers(&conn)?;

    create_views(&conn)?;
    create_procedure(&conn)?;
    
    
    conn.close()?;

    println!("Les données ont été insérées avec succès !");
    Ok(())
}

fn insert_movies(conn: &Connection) -> Result<(), oracle::Error> {
    println!("Préparation de l'insertion des films...");
    // Insertion des données : TMDB_movie_dataset_v11.csv
    let file_path = format!("{}/TMDB_movie_dataset_v11.csv", DATASET_FOLDER);
    let file = File::open(file_path).expect("Impossible d'ouvrir le fichier CSV");
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    // Préparation du batch
    let sql_stmt = Movie::batch_insert_statement();
    let mut batch = conn.batch(&sql_stmt, 32768).build()?;

    let mut seen_ids = HashSet::new();

    println!("Début de l'insertion...");
    for (index, result) in rdr.deserialize().enumerate() {
        match result {
            Ok(movie) => {
                let movie: Movie = movie;

                if !seen_ids.insert(movie.id) {
                    println!("(Skip) Doublon détecté pour l'ID : {}", movie.id);
                    continue; // Ignorez les doublons
                }
                
                let res: Result<(), oracle::Error> = batch.append_row(&[
                    &movie.id,
                    &movie.title,
                    &movie.vote_average,
                    &movie.vote_count,
                    &movie.status,
                    &movie.release_date,
                    &movie.revenue,
                    &movie.runtime,
                    &if movie.adult { 1 } else { 0 },
                    &movie.backdrop_path,
                    &movie.budget,
                    &movie.homepage,
                    &movie.imdb_id,
                    &movie.original_language,
                    &movie.original_title,
                    &movie.overview,
                    &movie.popularity,
                    &movie.poster_path,
                    &movie.tagline,
                    &movie.genres,
                    &movie.production_companies,
                    &movie.production_countries,
                    &movie.spoken_languages,
                    &movie.keywords,
                ]);
                if res.is_err() {
                    eprintln!("Erreur lors de l'insertion du film {} : {}", movie.id, res.err().unwrap());
                    continue;
                }
            }
            Err(e) => {
                eprintln!("Erreur lors de la lecture du CSV : {:?}", e);
            },
        }

        // Afficher la progression
        if (index + 1) % 100000 == 0 {
            println!("Progression : {} films insérés...", index + 1);
        }
    }

    // Exécuter les lignes restantes dans le batch
    batch.execute()?;

    // Commit
    conn.commit()?;

    Ok(())
}

fn insert_ratings(conn: &Connection, seen_users: &mut HashSet<i32>) -> Result<(), oracle::Error> {
    println!("Préparation de l'insertion des ratings...");
    // Charger les données des ratings
    let file_path = format!("{}/movie-lens/ratings.csv", DATASET_FOLDER);
    let file = File::open(file_path).expect("Impossible d'ouvrir le fichier ratings.csv");
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let sql_stmt = Rating::batch_insert_statement();
    let mut batch = conn.batch(&sql_stmt, 1000).build()?;

    println!("Insertion des ratings...");
    for (index, result) in rdr.deserialize().enumerate() {
        match result {
            Ok(rating) => {
                let rating: Rating = rating;
                let res = batch.append_row(&[
                    &rating.user_id,
                    &rating.movie_id,
                    &rating.rating,
                    &rating.timestamp,
                ]);
                if res.is_err() {
                    eprintln!("Erreur lors de l'insertion du rating {} : {}", rating.user_id, res.err().unwrap());
                    continue;
                }

                // Ajouter l'utilisateur à la liste des utilisateurs vus
                seen_users.insert(rating.user_id);

                // Afficher la progression
                if (index + 1) % 1000 == 0 {
                    println!("Progression : {} ratings insérés...", index + 1);
                }
            }
            Err(e) => eprintln!("Erreur lors de la lecture du rating : {:?}", e),
        }
    }

    batch.execute()?;

    conn.commit()?;
    Ok(())
}

fn insert_tags(conn: &Connection, seen_users: &mut HashSet<i32>) -> Result<(), oracle::Error> {
    println!("Préparation de l'insertion des tags...");
    // Charger les données des tags
    let file_path = format!("{}/movie-lens/tags.csv", DATASET_FOLDER);
    let file = File::open(file_path).expect("Impossible d'ouvrir le fichier tags.csv");
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let sql_stmt = Tag::batch_insert_statement();
    let mut batch = conn.batch(&sql_stmt, 1000).build()?;

    println!("Insertion des tags...");
    for (index, result) in rdr.deserialize().enumerate() {
        match result {
            Ok(tag) => {
                let tag: Tag = tag;

                let res = batch.append_row(&[
                    &tag.user_id,
                    &tag.movie_id,
                    &tag.tag,
                    &tag.timestamp,
                ]);

                if res.is_err() {
                    eprintln!("Erreur lors de l'insertion du tag {} : {}", tag.user_id, res.err().unwrap());
                    continue;
                }

                // Ajouter l'utilisateur à la liste des utilisateurs vus
                seen_users.insert(tag.user_id);

                // Afficher la progression
                if (index + 1) % 1000 == 0 {
                    println!("Progression : {} tags insérés...", index + 1);
                }
            }
            Err(e) => eprintln!("Erreur lors de la lecture du tag : {:?}", e),
        }
    }

    batch.execute()?;

    conn.commit()?;
    
    Ok(())
}

fn insert_users(conn: &Connection, seen_users: &HashSet<i32>) {
    println!("Préparation de l'insertion des utilisateurs...");
    let sql_stmt = User::batch_insert_statement();
    let mut batch = conn.batch(&sql_stmt, 1000).build().unwrap();

    println!("Insertion des utilisateurs...");
    for (index, user_id) in seen_users.iter().enumerate() {
        let res = batch.append_row(&[user_id]);
        if res.is_err() {
            eprintln!("Erreur lors de l'insertion de l'utilisateur {} : {}", user_id, res.err().unwrap());
            continue;
        }

        // Afficher la progression
        if (index + 1) % 1000 == 0 {
            println!("Progression : {} utilisateurs insérés...", index + 1);
        }
    }

    batch.execute().unwrap();
    conn.commit().unwrap();
}

fn insert_links(conn: &Connection) -> Result<(), oracle::Error> {
    println!("Préparation de l'insertion des liens...");
    // Charger les données des liens
    let file_path = format!("{}/movie-lens/links.csv", DATASET_FOLDER);
    let file = File::open(file_path).expect("Impossible d'ouvrir le fichier links.csv");
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let sql_stmt = Link::batch_insert_statement();
    let mut batch = conn.batch(&sql_stmt, 1000).build()?;

    println!("Insertion des movies data links...");
    for (index, result) in rdr.deserialize().enumerate() {
        match result {
            Ok(link) => {
                let link: Link = link;
                let res = batch.append_row(&[
                    &link.movie_id,
                    &link.imdb_id,
                    &link.tmdb_id,
                ]);

                if res.is_err() {
                    eprintln!("Erreur lors de l'insertion du lien {} : {}", link.movie_id, res.err().unwrap());
                    continue;
                }

                // Afficher la progression
                if (index + 1) % 1000 == 0 {
                    println!("Progression : {} links insérés...", index + 1);
                }
            }
            Err(e) => eprintln!("Erreur lors de la lecture du lien : {:?}", e),
        }
    }

    batch.execute()?;

    conn.commit()?;
    Ok(())
}


fn create_views(conn: &Connection) -> Result<(), oracle::Error> {

    println!("Création des vues...");

    println!("Création de la vue MovieDetailsView...");
    // Création de la vue MovieDetailsView
    conn.execute("CREATE OR REPLACE VIEW MovieDetailsView AS
        SELECT 
            l.movie_id AS movie_id,
            tmd.id AS tmdb_id,
            tmd.title,
            tmd.vote_average,
            tmd.vote_count,
            tmd.status,
            tmd.release_date,
            tmd.runtime,
            tmd.adult,
            tmd.backdrop_path,
            tmd.overview,
            tmd.poster_path,
            tmd.genres,
            tmd.keywords,
            tmd.budget,
            tmd.revenue
        FROM 
            TMDB_movie_dataset tmd
        JOIN 
    MovieLens_Links l ON l.tmdb_id = tmd.id", &[])?;

    conn.execute("GRANT SELECT ON MovieDetailsView TO movie_db_user", &[])?;
    // Synonym
    conn.execute("DROP PUBLIC SYNONYM MovieDetailsView", &[]).ok();
    conn.execute("CREATE PUBLIC SYNONYM MovieDetailsView FOR SYSTEM.MovieDetailsView", &[])?;


    println!("Création de la vue MovieShortView...");
    // Création de la vue MovieShortView
    // Contient les informations essentielles des films
    conn.execute("CREATE OR REPLACE VIEW MovieShortView AS
        SELECT 
            l.movie_id AS movie_id,
            tmd.id AS tmdb_id,
            tmd.title,
            tmd.vote_average,
            tmd.poster_path,
            tmd.genres,
            tmd.popularity,
            tmd.release_date
        FROM 
            TMDB_movie_dataset tmd
        JOIN 
            MovieLens_Links l ON l.tmdb_id = tmd.id", &[])?;

    conn.execute("GRANT SELECT ON MovieShortView TO movie_db_user", &[])?;
    // Synonym
    conn.execute("DROP PUBLIC SYNONYM MovieShortView", &[]).ok();
    conn.execute("CREATE PUBLIC SYNONYM MovieShortView FOR SYSTEM.MovieShortView", &[])?;

    println!("Création de la vue MovieShortView_Alphabetical...");
    // Création de la vue MovieShortView_Alphabetical
    conn.execute(
        "CREATE OR REPLACE VIEW MovieShortView_Alphabetical AS
        SELECT *
        FROM MovieShortView
        ORDER BY title ASC", 
        &[]
    )?;

    conn.execute("GRANT SELECT ON MovieShortView_Alphabetical TO movie_db_user", &[])?;

    conn.execute("DROP PUBLIC SYNONYM MovieShortView_Alphabetical", &[]).ok();
    conn.execute("CREATE PUBLIC SYNONYM MovieShortView_Alphabetical FOR SYSTEM.MovieShortView_Alphabetical", &[])?;
    

    println!("Création de la vue MovieShortView_ByPopularity...");
    // Création de la vue MovieShortView_ByPopularity
    conn.execute(
        "CREATE OR REPLACE VIEW MovieShortView_ByPopularity AS
        SELECT *
        FROM MovieShortView
        ORDER BY popularity DESC", 
        &[]
    )?;

    conn.execute("GRANT SELECT ON MovieShortView_ByPopularity TO movie_db_user", &[])?;

    conn.execute("DROP PUBLIC SYNONYM MovieShortView_ByPopularity", &[]).ok();
    conn.execute("CREATE PUBLIC SYNONYM MovieShortView_ByPopularity FOR SYSTEM.MovieShortView_ByPopularity", &[])?;

    println!("Création de la vue MovieShortView_ByReleaseDate...");
    // Création de la vue MovieShortView_ByReleaseDate
    conn.execute(
        "CREATE OR REPLACE VIEW MovieShortView_ByReleaseDate AS
        SELECT *
        FROM MovieShortView
        ORDER BY release_date DESC", 
        &[]
    )?;

    conn.execute("GRANT SELECT ON MovieShortView_ByReleaseDate TO movie_db_user", &[])?;

    conn.execute("DROP PUBLIC SYNONYM MovieShortView_ByReleaseDate", &[]).ok();
    conn.execute("CREATE PUBLIC SYNONYM MovieShortView_ByReleaseDate FOR SYSTEM.MovieShortView_ByReleaseDate", &[])?;

    println!("Création de la vue MovieShortView_ByRating...");
    // Création de la vue MovieShortView_ByRating
    conn.execute(
        "CREATE OR REPLACE VIEW MovieShortView_ByRating AS
        SELECT *
        FROM MovieShortView
        ORDER BY vote_average DESC", 
        &[]
    )?;

    conn.execute("GRANT SELECT ON MovieShortView_ByRating TO movie_db_user", &[])?;

    conn.execute("DROP PUBLIC SYNONYM MovieShortView_ByRating", &[]).ok();
    conn.execute("CREATE PUBLIC SYNONYM MovieShortView_ByRating FOR SYSTEM.MovieShortView_ByRating", &[])?;

    println!("Création de la vue TopUsers...");
    // Creation de la vue TopUsersByRatings
    // Contient le nombre de ratings par utilisateur
    conn.execute(
    "CREATE OR REPLACE VIEW TopUsers AS
        WITH RatingsCount AS (
            SELECT 
                user_id, 
                COUNT(*) AS num_ratings
            FROM 
                MovieLens_Ratings
            GROUP BY 
                user_id
        ),
        TagsCount AS (
            SELECT 
                user_id, 
                COUNT(*) AS num_tags
            FROM 
                MovieLens_Tags
            GROUP BY 
                user_id
        )
        SELECT 
            COALESCE(r.user_id, t.user_id) AS user_id,
            COALESCE(r.num_ratings, 0) AS num_ratings,
            COALESCE(t.num_tags, 0) AS num_tags,
            COALESCE(r.num_ratings, 0) + COALESCE(t.num_tags, 0) AS total_contributions
        FROM 
            RatingsCount r
        FULL OUTER JOIN 
            TagsCount t ON r.user_id = t.user_id
        ORDER BY 
    total_contributions DESC", &[])?;

    conn.execute("GRANT SELECT ON TopUsers TO movie_db_user", &[])?;

    conn.execute("DROP PUBLIC SYNONYM TopUsersByRatings", &[]).ok();
    conn.execute("CREATE PUBLIC SYNONYM TopUsersByRatings FOR SYSTEM.TopUsersByRatings", &[])?;
    

    conn.commit()?;
    Ok(())
}

fn create_triggers(conn: &Connection) -> Result<(), oracle::Error> {
    println!("Création des triggers...");

    println!("Création du trigger trg_delete_user_cleanup...");
    // Trigger pour la suppression en cascade des ratings et tags lors de la suppression d'un utilisateur
    conn.execute("CREATE OR REPLACE TRIGGER trg_delete_user_cleanup
        AFTER DELETE ON MovieLens_Users
        FOR EACH ROW
        BEGIN
            -- Supprimer les ratings associés à l'utilisateur
            DELETE FROM MovieLens_Ratings WHERE user_id = :OLD.user_id;
            
            -- Supprimer les tags associés à l'utilisateur
            DELETE FROM MovieLens_Tags WHERE user_id = :OLD.user_id;
        END;
        ",
        &[],
    )?;

    println!("Création du trigger trg_delete_movie_cleanup...");
    // Trigger pour la suppression en cascade des ratings et tags lors de la suppression de la relation film movieLens <-> TMDB
    conn.execute("
        CREATE OR REPLACE TRIGGER trg_delete_movie_cleanup
        AFTER DELETE ON MovieLens_Links
        FOR EACH ROW
        BEGIN
            -- Supprimer les ratings associés au film
            DELETE FROM MovieLens_Ratings WHERE movie_id = :OLD.movie_id;
            
            -- Supprimer les tags associés au film
            DELETE FROM MovieLens_Tags WHERE movie_id = :OLD.movie_id;
        END;
        ",
        &[],
    )?;

    conn.commit()?;
    Ok(())
}

fn create_procedure(conn: &Connection) -> Result<(), oracle::Error> {
    println!("Création des procédures...");

    // Création de la procédure GetStats
    // Retourne les statistiques : total_movies, total_ratings, total_tags, distinct_users...
    println!("Création de la procédure GetStats...");

    conn.execute(
        "CREATE OR REPLACE PROCEDURE GetStats(
        total_movies OUT NUMBER,
        total_ratings OUT NUMBER,
        total_tags OUT NUMBER,
        distinct_users OUT NUMBER,
        
        genre_count OUT SYS_REFCURSOR,
        
        top_users OUT SYS_REFCURSOR,
        top_profits_films OUT SYS_REFCURSOR
    ) AS
    BEGIN
        -- Nombre total distinct de films enregistrés (TMDB join Rating)
        SELECT COUNT(DISTINCT l.movie_id)
        INTO total_movies
        FROM MovieLens_Links l
        JOIN TMDB_movie_dataset tmd ON l.tmdb_id = tmd.id;

        -- Nombre total d'avis (Rating)
        SELECT COUNT(*)
        INTO total_ratings
        FROM MovieLens_Ratings;

        -- Nombre total de tags
        SELECT COUNT(*)
        INTO total_tags
        FROM MovieLens_Tags;

        -- Nombre distinct d'utilisateurs
        SELECT COUNT(DISTINCT user_id)
        INTO distinct_users
        FROM (
            SELECT user_id FROM MovieLens_Ratings
            UNION
            SELECT user_id FROM MovieLens_Tags
        );
        
        -- Genres count
        OPEN genre_count FOR
            WITH GenreSplit AS (
            SELECT 
                TRIM(REGEXP_SUBSTR(TO_CHAR(genres), '[^,]+', 1, LEVEL)) AS genre,
                movie_id
            FROM 
                MovieDetailsView
            CONNECT BY REGEXP_SUBSTR(TO_CHAR(genres), '[^,]+', 1, LEVEL) IS NOT NULL
            AND PRIOR movie_id = movie_id
            AND PRIOR SYS_GUID() IS NOT NULL
        )
        SELECT 
            genre,
            COUNT(*) AS num_movies
        FROM 
            GenreSplit
        GROUP BY 
            genre
        ORDER BY 
            num_movies DESC;
            
            
        -- get top users
        OPEN top_users FOR
        SELECT user_id, num_ratings, num_tags
        FROM TopUsers
        FETCH FIRST 5 ROWS ONLY;
        
        -- get top profit movie
        OPEN top_profits_films FOR
        SELECT 
            movie_id, 
            title, 
            poster_path, 
            (revenue - budget) AS profit
        FROM 
            MovieDetailsView
        ORDER BY 
            profit DESC
        FETCH FIRST 5 ROWS ONLY;
    END;", &[])?;
    
    // Grant
    conn.execute("GRANT EXECUTE ON GetStats TO movie_db_user", &[])?;

    // Synonym
    conn.execute("DROP PUBLIC SYNONYM GetStats", &[]).ok();
    conn.execute("CREATE PUBLIC SYNONYM GetStats FOR SYSTEM.GetStats", &[])?;

    // Création de la procédure DeleteMovieLensUser
    // Supprime un utilisateur movieLens et donc ses ratings et tags associés
    println!("Création de la procédure DeleteMovieLensUser...");

    conn.execute("CREATE OR REPLACE PROCEDURE DeleteMovieLensUser(
            u_id IN NUMBER
        ) AS
        BEGIN
            -- Supprimer l'utilisateur dans la table MovieLens_Users
            DELETE FROM MovieLens_Users
            WHERE user_id = u_id;

            -- Effectuer un commit pour enregistrer les modifications
            COMMIT;

            -- Afficher un message pour indiquer que l'utilisateur a été supprimé avec succès
            DBMS_OUTPUT.PUT_LINE('Utilisateur avec user_id=' || u_id || ' supprimé avec succès.');
        EXCEPTION
            WHEN OTHERS THEN
                -- Gérer les erreurs éventuelles
                RAISE_APPLICATION_ERROR(
                    -20002,
                    'Erreur lors de la suppression de l''utilisateur ' || u_id || ': ' || SQLERRM
                );
    END;", &[])?;

    // Grant
    conn.execute("GRANT EXECUTE ON DeleteMovieLensUser TO movie_db_admin", &[])?;

    // Synonym
    conn.execute("DROP PUBLIC SYNONYM DeleteMovieLensUser", &[]).ok();
    conn.execute("CREATE PUBLIC SYNONYM DeleteMovieLensUser FOR SYSTEM.DeleteMovieLensUser", &[])?;

    // Création de la procédure DeleteMovieLensTag
    // Supprime un tag movieLens

    println!("Création de la procédure DeleteMovieLensTag...");
    conn.execute("CREATE OR REPLACE PROCEDURE DeleteMovieLensTag(
        m_id IN NUMBER,
        u_id IN NUMBER,
        tag_timestamp IN NUMBER
    ) AS
    BEGIN
        -- Supprimer le tag correspondant dans la table MovieLens_Tags
        DELETE FROM MovieLens_Tags
        WHERE user_id = u_id
        AND movie_id = m_id
        AND timestamp = tag_timestamp;

        -- Effectuer un commit pour enregistrer les modifications
        COMMIT;

        -- Afficher un message pour indiquer que le tag a été supprimé avec succès
        DBMS_OUTPUT.PUT_LINE('Tag supprimé avec succès pour user_id=' || u_id || ', movie_id=' || m_id || ', timestamp=' || tag_timestamp);
    EXCEPTION
        WHEN OTHERS THEN
            -- Gérer les erreurs éventuelles
            RAISE_APPLICATION_ERROR(
                -20003,
                'Erreur lors de la suppression du tag pour user_id=' || u_id || ', movie_id=' || m_id || ', timestamp=' || tag_timestamp || ': ' || SQLERRM
            );
    END;", &[])?;

    // Grant
    conn.execute("GRANT EXECUTE ON DeleteMovieLensTag TO movie_db_admin", &[])?;

    // Synonym
    conn.execute("DROP PUBLIC SYNONYM DeleteMovieLensTag", &[]).ok();
    conn.execute("CREATE PUBLIC SYNONYM DeleteMovieLensTag FOR SYSTEM.DeleteMovieLensTag", &[])?;

    // Commit
    conn.commit()?;

    Ok(())
}