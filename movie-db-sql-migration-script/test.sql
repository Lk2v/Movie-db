SELECT count(*) FROM TMDB_movie_dataset;
SELECT * FROM movielens_ratings;
SELECT * FROM movielens_links;
SELECT * FROM movielens_tags;

CREATE PUBLIC SYNONYM m FOR TMDB_movie_dataset;

-- obtenir nom utilisateur actuel
select user from dual;

-- privilege de l'utilisateur actuel
SELECT * FROM USER_SYS_PRIVS; 
SELECT * FROM USER_TAB_PRIVS;
SELECT * FROM USER_ROLE_PRIVS;

-- recuperer 100 premier film
SELECT *
    FROM MovieShortView
    FETCH FIRST 100 ROWS ONLY;

GRANT SELECT ON MovieShortView_Alphabetical TO PUBLIC;

-- recupere film genre Action
SELECT *
FROM MovieShortView
WHERE DBMS_LOB.INSTR(genres, 'Action') > 0;

-- recupere film avec le plus de rating
SELECT 
    tmd.title,
    COUNT(ml.rating) AS num_ratings
FROM 
    MovieLens_Ratings ml
JOIN 
    MovieLens_Links l ON ml.movie_id = l.movie_id
JOIN 
    TMDB_movie_dataset tmd ON l.tmdb_id = tmd.id
GROUP BY 
    tmd.title
ORDER BY 
    num_ratings DESC
FETCH FIRST 1 ROW ONLY;


-- recupere le film avec le plus de tag
SELECT 
    tmd.title,
    COUNT(mt.tag) AS num_tags
FROM 
    MovieLens_Tags mt
JOIN 
    MovieLens_Links l ON mt.movie_id = l.movie_id
JOIN 
    TMDB_movie_dataset tmd ON l.tmdb_id = tmd.id
GROUP BY 
    tmd.title
ORDER BY 
    num_tags DESC
FETCH FIRST 1 ROW ONLY;

SELECT count(*)
    FROM MovieShortView
    WHERE DBMS_LOB.INSTR(genres, 'Mystery') > 0;
    --FETCH FIRST 100 ROWS ONLY;

-- Test obtention film
SELECT 
    l.movie_id AS movie_id,
    tmd.id AS tmdb_id,
    tmd.title,
    tmd.vote_average,
    tmd.vote_count,
    tmd.status,
    TO_CHAR(tmd.release_date, 'YYYY-MM-DD') AS release_date,
    tmd.runtime,
    tmd.adult,
    tmd.backdrop_path,
    tmd.homepage,
    tmd.overview,
    tmd.poster_path,
    tmd.genres,
    tmd.keywords
FROM 
    TMDB_movie_dataset tmd
JOIN 
    MovieLens_Links l ON l.tmdb_id = tmd.id
WHERE 
    l.movie_id = 1840;


-- Film present dans TMDB_movie_dataset mais pas MovieLens_Links
SELECT t.id, t.title
FROM TMDB_movie_dataset t
WHERE NOT EXISTS (
    SELECT 1
    FROM MovieLens_Links l
    WHERE t.id = l.tmdb_id
);

-- Film interdit au moins de 18 ans
SELECT DISTINCT t.id, t.title, t.vote_average, t.release_date
FROM TMDB_movie_dataset t
JOIN MovieLens_Links l ON t.id = l.tmdb_id
JOIN MovieLens_Ratings r ON l.movie_id = r.movie_id
WHERE t.adult = 1;
-- aucun film adulte dans le dataset (on gère quand meme le cas dans l'UI même si ceux-ci ne seront jamais affichées)




/*Analyse utilisateurs*/

-- Nombre d'utilisateur unique ayant noté les films
SELECT COUNT(DISTINCT user_id) AS total_users
FROM MovieLens_Ratings;

-- Nombre d'utilisateur unique ayant commenté
SELECT COUNT(DISTINCT user_id) AS total_users
FROM MovieLens_Tags;

-- Nombre total d'utilisateur 
SELECT COUNT(DISTINCT user_id) AS total_users
FROM (
    SELECT user_id FROM MovieLens_Ratings
    UNION
    SELECT user_id FROM MovieLens_Tags
);
/**/
SELECT 
    l.movie_id as movie_id,
    tmd.id AS tmdb_id,
    tmd.title,
    tmd.vote_average,
    tmd.vote_count,
    tmd.status,
    TO_CHAR(tmd.release_date, 'YYYY-MM-DD') AS release_date,
    tmd.adult,
    tmd.backdrop_path,
    tmd.overview,
    tmd.poster_path,
    tmd.genres,
    tmd.keywords
FROM 
    TMDB_movie_dataset tmd
JOIN 
    MovieLens_Links l ON l.tmdb_id = tmd.id
WHERE 
    l.movie_id = 1834;

/**/
SELECT user_id, movie_id, rating, movielens_tags.tag, movielens_ratings.timestamp
FROM movielens_ratings r
FULL OUTER JOIN movielens_tags t ON r.movie_id = t.movie_id
WHERE r.movie_id = 1834 OR t.movie_id = 1834;

SELECT * FROM movielens_ratings WHERE movie_id = 1834;

-- full movie infos
CREATE OR REPLACE VIEW MovieDetailsView AS
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
    MovieLens_Links l ON l.tmdb_id = tmd.id;

SELECT * FROM MovieDetailsView;

    
-- Short movie details
CREATE OR REPLACE VIEW MovieShortView AS
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
        MovieLens_Links l ON l.tmdb_id = tmd.id;
    
SELECT * FROM MovieShortView;

-- Vue filtrage

-- Films classés par ordre alphabétique
CREATE OR REPLACE VIEW MovieShortView_Alphabetical AS
SELECT *
FROM MovieShortView
ORDER BY title ASC;

SELECT *
FROM MovieShortView_Alphabetical;


-- Films classés par popularité (popularity)
CREATE OR REPLACE VIEW MovieShortView_ByPopularity AS
SELECT *
FROM MovieShortView
ORDER BY popularity DESC;

SELECT *
FROM MovieShortView_ByPopularity;

-- Films classés par date de sortie (release_date), les plus récents d'abord
CREATE OR REPLACE VIEW MovieShortView_ByReleaseDate AS
SELECT *
FROM MovieShortView
ORDER BY release_date DESC;

SELECT * FROM MovieShortView_ByReleaseDate;
--  Films classés par note moyenne (vote_average), les mieux notés en premier
CREATE OR REPLACE VIEW MovieShortView_ByRating AS
SELECT *
FROM MovieShortView
ORDER BY vote_average DESC;

SELECT * FROM MovieShortView_ByRating;



-- obtenir les note des utilisateurs avec les details
SELECT * FROM movielens_ratings
INNER JOIN movielens_links ON movielens_ratings.movie_id = movielens_links.movie_id
INNER JOIN TMDB_movie_dataset ON movielens_links.tmdb_id = TMDB_movie_dataset.id
ORDER BY rating DESC
FETCH FIRST 10 ROWS ONLY;

/*debug*/
-- debug des valeurs null
SELECT *
FROM 
        TMDB_movie_dataset tmd
    JOIN 
        MovieLens_Links l ON l.tmdb_id = tmd.id
WHERE id IS NULL
   OR title IS NULL
   OR vote_average IS NULL
   OR vote_count IS NULL
   OR status IS NULL
   OR release_date IS NULL
   --OR revenue IS NULL
   --OR runtime IS NULL
   OR adult IS NULL
   OR backdrop_path IS NULL
   --OR budget IS NULL
   --OR homepage IS NULL
   --OR imdb_id IS NULL
   --OR original_language IS NULL
   --OR original_title IS NULL
   OR overview IS NULL
   OR popularity IS NULL
   OR poster_path IS NULL
   --OR tagline IS NULL
   OR genres IS NULL
   --OR production_companies IS NULL
   --OR production_countries IS NULL
   --OR spoken_languages IS NULL
   --OR keywords IS NULL
   ;


-- nombre total de film dans movie lens link ou tmdb n'est pas relié
SELECT count(*)
FROM MovieLens_Links
WHERE MovieLens_Links.tmdb_id IS NULL;

-- nombre films sans lien avec tmdb
SELECT COUNT(DISTINCT ml.movie_id) AS num_movies_without_tmdb
FROM MovieLens_Ratings ml
LEFT JOIN MovieLens_Links l ON ml.movie_id = l.movie_id
LEFT JOIN TMDB_movie_dataset tmd ON l.tmdb_id = tmd.id
WHERE tmd.id IS NULL;

-- nombre total film tmdb
SELECT COUNT(*) AS total_tmdb_movies
FROM TMDB_movie_dataset;

-- nombre total de film avec des notes
SELECT COUNT(DISTINCT movie_id) AS unique_rating_movies
FROM MovieLens_Ratings;

-- films sans note
SELECT tmd.title
FROM MovieLens_Links ml
LEFT JOIN MovieLens_Ratings mr ON ml.movie_id = mr.movie_id
LEFT JOIN TMDB_movie_dataset tmd ON ml.tmdb_id = tmd.id
WHERE mr.movie_id IS NULL;



-- recuperer stat categorie film :
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


    
/*top meilleure film*/
SELECT 
    ml.movie_id,
    tmd.title,
    AVG(ml.rating) AS average_rating,
    COUNT(ml.rating) AS num_ratings,
    tmd.vote_average AS TMDB_VOTE_RATE
FROM 
    MovieLens_Ratings ml
LEFT JOIN 
    MovieLens_Links l ON ml.movie_id = l.movie_id
LEFT JOIN 
    TMDB_movie_dataset tmd ON l.tmdb_id = tmd.id
GROUP BY 
    ml.movie_id, tmd.title, tmd.vote_average
ORDER BY 
    average_rating DESC,
    num_ratings DESC
FETCH FIRST 10 ROWS ONLY;

/*top pire film*/
SELECT 
    ml.movie_id,
    tmd.title,
    AVG(ml.rating) AS average_rating,
    COUNT(ml.rating) AS num_ratings,
    tmd.vote_average AS TMDB_VOTE_RATE
FROM 
    MovieLens_Ratings ml
LEFT JOIN 
    MovieLens_Links l ON ml.movie_id = l.movie_id
LEFT JOIN 
    TMDB_movie_dataset tmd ON l.tmdb_id = tmd.id
GROUP BY 
    ml.movie_id, 
    tmd.title, 
    tmd.vote_average
ORDER BY 
    average_rating ASC,  -- Note moyenne en ordre croissant
    num_ratings ASC      -- Nombre de notes en ordre croissant
FETCH FIRST 10 ROWS ONLY;

SELECT count(*) FROM TMDB_movie_dataset;
SELECT count(*) FROM movielens_ratings;
SELECT count(*) FROM movielens_links;
COMMIT;

/*DROP TABLE movielens_links;
DROP TABLE movielens_ratings;
DROP TABLE TMDB_movie_dataset;*/

-- USER :

CREATE PUBLIC SYNONYM TopUsers FOR SYSTEM.TopUsers;

SELECT * FROM TopUsers;

CREATE OR REPLACE VIEW TopUsers AS
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
    total_contributions DESC;




-- VIEW STAT : 

-- TODO : 
-- Finir systeme de statistique et l'implementer : counts, tops
-- systeme suppression utilisateur oracle et utilisateur movie lens

DROP PROCEDURE GetStats;

SELECT *
  FROM USER_PROCEDURES
 WHERE object_name = 'CreateUser';

SELECT * FROM MovieLens_Links;

-- get count stat
CREATE OR REPLACE PROCEDURE GetStats(
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
        SELECT COUNT(DISTINCT movie_id)
        INTO total_movies
        FROM MovieDetailsView;

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
    END;
/



-- get top users
SELECT user_id, num_ratings, num_tags
    FROM TopUsers
    FETCH FIRST 5 ROWS ONLY;


-- top film meilleur profit
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

-- Stats test
VARIABLE total_movies NUMBER;
VARIABLE total_ratings NUMBER;
VARIABLE total_tags NUMBER;
VARIABLE distinct_users NUMBER;


EXEC GetStats(:total_movies, :total_ratings, :total_tags, :distinct_users);

PRINT total_movies;
PRINT total_ratings;
PRINT total_tags;
PRINT distinct_users;

GRANT EXECUTE ON SYSTEM.GetStats TO PUBLIC;

-- verifier fonctionnement
SELECT OBJECT_NAME, STATUS
FROM DBA_OBJECTS
WHERE OBJECT_TYPE = 'PROCEDURE'
  AND OWNER = 'SYSTEM'
  AND OBJECT_NAME = 'SUPPRIMERUTILISATEUR';

-- verifier erreur compilation procedure
SELECT *
FROM USER_ERRORS
WHERE NAME = 'GETSTATS'
AND TYPE = 'PROCEDURE';

SELECT COUNT(*)
FROM MovieLens_Links ml
JOIN TMDB_movie_dataset tmd ON ml.tmdb_id = tmd.id;

SELECT COUNT(DISTINCT l.movie_id)
    FROM MovieLens_Links l
    JOIN TMDB_movie_dataset tmd ON l.tmdb_id = tmd.id;


-- Creation/Gestion des utilisateurs


-- Lister les utilisateurs
CREATE OR REPLACE PROCEDURE ListUsers(
    result_cursor OUT SYS_REFCURSOR
) AS
BEGIN
    -- Récupérer les utilisateurs créés dans le tablespace dédié
    OPEN result_cursor FOR
    SELECT 
        u.USER_ID AS id,
        u.USERNAME,
        CASE 
            WHEN EXISTS (
                SELECT 1 
                FROM DBA_ROLE_PRIVS r
                WHERE r.GRANTEE = u.USERNAME 
                  AND r.GRANTED_ROLE = 'DELETE ANY TABLE'
            ) THEN 1
            ELSE 0
        END AS isAdmin
    FROM 
        DBA_USERS u
    WHERE 
        u.DEFAULT_TABLESPACE = 'MOVIE_DB_TBS';
END;
/


CREATE OR REPLACE PROCEDURE CurrentUserStatus(
    user_status OUT SYS_REFCURSOR
) AS
BEGIN
    -- Ouvre un curseur pour retourner les informations de l'utilisateur actuel
    OPEN user_status FOR
    SELECT 
        username,
        is_admin,
        created_at
    FROM 
        Users_Records
    WHERE 
        username = (SELECT USER FROM DUAL);
END;
/


SELECT grantee
FROM dba_role_privs
WHERE granted_role = 'MOVIE_DB_USER';

GRANT SELECT, INSERT ON Users_Records TO movie_db_admin;

DROP TABLESPACE movie_db_tbs INCLUDING CONTENTS AND DATAFILES;

DROP PROCEDURE CreateUser;


CREATE OR REPLACE PROCEDURE CreateUser(
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
    END;
    /
    

GRANT SELECT, INSERT, DELETE ON Users_Records TO movie_db_admin;

BEGIN
    CreateUser('admin_pop', 'pass', 1);
END;
/


GRANT movie_db_user TO movie_db_admin;
GRANT CREATE USER, ALTER USER, DROP USER, GRANT ANY ROLE TO movie_db_admin;


DELETE FROM Users_Records WHERE username = 'king';


SELECT *
FROM USER_ERRORS
WHERE NAME = 'CREATEUSER'
  AND TYPE = 'PROCEDURE';


GRANT EXECUTE ON GetUserList TO movie_db_admin;
GRANT SELECT ON DBA_ROLE_PRIVS TO movie_db_admin;


SELECT * FROM Users_Records;




SELECT SYNONYM_NAME, TABLE_OWNER, TABLE_NAME
FROM DBA_SYNONYMS
WHERE OWNER = 'PUBLIC' AND SYNONYM_NAME = 'TMDB_movie_dataset';

CREATE PUBLIC SYNONYM TMDB_movie_dataset FOR SYSTEM.TMDB_movie_dataset;

GRANT SELECT ON MovieShortView_ByPopularity TO movie_db_admin;

alter session set container=XEPDB1;

CREATE TABLESPACE movie_db_tbs
            DATAFILE '/opt/oracle/oradata/movie_db_tbs02.dbf'
            SIZE 100M
            AUTOEXTEND ON
            NEXT 10M
            MAXSIZE UNLIMITED;
           
DROP TABLESPACE movie_db_tbs INCLUDING CONTENTS AND DATAFILES;

SELECT TABLESPACE_NAME, STATUS FROM DBA_TABLESPACES;

SELECT FILE_NAME, TABLESPACE_NAME, STATUS
FROM DBA_DATA_FILES
WHERE FILE_NAME LIKE '%movie_db_tbs01.dbf';

ALTER DATABASE DATAFILE '/opt/oracle/oradata/movie_db_tbs01.dbf' OFFLINE DROP;


DROP TABLESPACE movie_db_tbs INCLUDING CONTENTS AND DATAFILES;

CREATE USER jklm
    IDENTIFIED BY "123"
    DEFAULT TABLESPACE movie_db_tbs
    QUOTA UNLIMITED ON movie_db_tbs;

SELECT * 
FROM DBA_SYS_PRIVS 
WHERE GRANTEE = 'OK';


SELECT * 
FROM DBA_PROFILES 
WHERE PROFILE = (SELECT PROFILE FROM DBA_USERS WHERE USERNAME = 'OK');


SELECT *
    FROM ALL_USERS;
    
CREATE ROLE movie_db_user;
GRANT SELECT ON SYSTEM.MovieLens_Tags TO movie_db_user;
GRANT movie_db_user TO BOUHHH;

CREATE PUBLIC SYNONYM MovieLens_Tags FOR SYSTEM.MovieLens_Tags;


SELECT * 
FROM DBA_TAB_PRIVS 
WHERE GRANTEE = 'MOVIE_DB_USER';

SELECT TABLESPACE_NAME, FILE_NAME, STATUS 
FROM DBA_DATA_FILES 
WHERE FILE_NAME = '/opt/oracle/oradata/movie_db_tbs01.dbf';


ALTER DATABASE DATAFILE '/opt/oracle/oradata/movie_db_tbs01.dbf' OFFLINE;
ALTER DATABASE DATAFILE '/opt/oracle/oradata/movie_db_tbs01.dbf' DROP;



CREATE OR REPLACE PROCEDURE DeleteUser(
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
END;
/


BEGIN
    DeleteUser('test');
END;
/

SELECT * FROM USERS_RECORDS;

-- test
CREATE OR REPLACE PROCEDURE DeleteMovieLensUser(
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
END;
/

SELECT * FROM MovieLens_Users WHERE user_id = 424;

SELECT 
    u.user_id,
    COALESCE(r.num_ratings, 0) AS num_ratings,
    COALESCE(t.num_tags, 0) AS num_tags
FROM 
    MovieLens_Users u
LEFT JOIN (
    SELECT 
        user_id, 
        COUNT(*) AS num_ratings
    FROM 
        MovieLens_Ratings
    GROUP BY 
        user_id
) r ON u.user_id = r.user_id
LEFT JOIN (
    SELECT 
        user_id, 
        COUNT(*) AS num_tags
    FROM 
        MovieLens_Tags
    GROUP BY 
        user_id
) t ON u.user_id = t.user_id
WHERE 
    u.user_id = 474;


BEGIN
    DeleteMovieLensUser(435);
END;



CREATE OR REPLACE PROCEDURE DeleteMovieLensTag(
    u_id IN NUMBER,
    m_id IN NUMBER,
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
END;
/


BEGIN
    DeleteMovieLensTag(:1, :2, :3);
END;
    
-- Clean tout les utilisateurs
BEGIN
    FOR user_rec IN (
        SELECT username
        FROM all_users
        WHERE username NOT IN (
            'SYS', 'SYSTEM', 'OUTLN', 'DBSNMP', 'APPQOSSYS', 'ORDDATA', 
            'ORDPLUGINS', 'ORDDATA', 'SI_INFORMTN_SCHEMA', 'MDSYS', 
            'CTXSYS', 'ANONYMOUS', 'XDB', 'WMSYS', 'EXFSYS', 
            'APEX_PUBLIC_USER', 'APEX_040000', 'APEX_050000', 
            'AUDSYS', 'LBACSYS', 'FLOWS_FILES', 'PUBLIC'
        )
        AND created > TO_DATE('2023-01-01', 'YYYY-MM-DD') -- Optionnel : filtre sur une date récente
    ) LOOP
        BEGIN
            EXECUTE IMMEDIATE 'DROP USER ' || user_rec.username || ' CASCADE';
            DBMS_OUTPUT.PUT_LINE('Utilisateur ' || user_rec.username || ' supprimé.');
        EXCEPTION
            WHEN OTHERS THEN
                DBMS_OUTPUT.PUT_LINE(
                    'Erreur lors de la suppression de l''utilisateur ' || user_rec.username || ': ' || SQLERRM
                );
        END;
    END LOOP;
END;
/


/*Get current user info*/
CREATE OR REPLACE PROCEDURE GetCurrentUserInfo(
    p_username OUT VARCHAR2,
    is_admin OUT NUMBER
) AS
BEGIN
    -- Récupérer le nom d'utilisateur actuel
    SELECT USER INTO p_username FROM dual;

    BEGIN
        -- Rechercher l'état administrateur (is_admin) dans la table Users_Records
        SELECT is_admin
        INTO is_admin
        FROM Users_Records
        WHERE UPPER(username) = UPPER(p_username);
        
        -- Afficher le résultat pour vérification
        DBMS_OUTPUT.PUT_LINE('Current user: ' || p_username || ', Admin status: ' || is_admin);
    EXCEPTION
        WHEN NO_DATA_FOUND THEN
            -- Si l'utilisateur actuel n'est pas trouvé, définir is_admin à -1
            is_admin := -1;
            DBMS_OUTPUT.PUT_LINE('Current user: ' || p_username || ' not found in Users_Records. Admin status set to -1.');
    END;
EXCEPTION
    WHEN OTHERS THEN
        -- Gérer les autres erreurs éventuelles
        RAISE_APPLICATION_ERROR(
            -20002,
            'An error occurred while retrieving user information: ' || SQLERRM
        );
END;
/

GRANT EXECUTE ON GetCurrentUserInfo TO movie_db_user;
GRANT SELECT ON Users_Records TO movie_db_user;

DECLARE
    current_username VARCHAR2(128);
    admin_status NUMBER;
BEGIN
    GetCurrentUserInfo(current_username, admin_status);
    DBMS_OUTPUT.PUT_LINE('Username: ' || current_username || ', Admin: ' || admin_status);
END;

