## 1. Introduction

### Project Overview

This academic project aims to demonstrate skills in designing, securing, optimizing, and automating a database management system (DBMS). The goal is to create a robust information system capable of efficiently handling data operations while providing relevant insights through queries, procedures, and automated processes.

We chose to focus on building an enriched cinematic database that combines user interactions from MovieLens with detailed metadata from TMDb. The objective is to develop a comprehensive platform that enables exploring and analyzing movies while centralizing data related to cinematic works and user interactions.

The features include searching for movies by genre or title, sorting based on various criteria, displaying enriched details for each movie, generating detailed statistics, and providing advanced SQL user management with different access levels. An intuitive graphical user interface (GUI) simplifies interaction and enhances the user experience.

This application is built using Rust (for backend logic), Oracle SQL (for database operations), and Tauri (for GUI development).

### Datasets

The project relies on two primary datasets, which are combined to enhance and enrich the information.

#### MovieLens [[source]](https://www.kaggle.com/datasets/aigamer/movie-lens-dataset)

The MovieLens dataset forms the foundation of our project. Created and maintained by the GroupLens research lab, it contains a rich collection of user interactions with movies, including ratings and tags. These data are essential for ranking movies by popularity, analyzing user preferences, and providing detailed statistics. Tags serve as "short comments" from users, while ratings help establish rankings and identify the most appreciated movies.


#### TMDb (The Movie Database) [[source]](https://www.kaggle.com/code/asaniczka/tmdb-movies-daily-updates/output)

To complement MovieLens data, we integrated the TMDb dataset, which provides detailed metadata about movies. It includes information such as movie descriptions, release dates, budgets, revenues, and posters. These data enrich MovieLens by adding a qualitative and visual dimension to create an engaging user interface. Posters and summaries facilitate exploration, while financial data and popularity metrics enable in-depth performance analysis.

## 2. Project Structure
The project is divided into two primary components, each addressing distinct aspects of the system's functionality.

### 2.1 Migration Script
The migration script serves as the backbone for setting up the database environment. It automates the creation and configuration of essential database components, ensuring consistency and efficiency across the system. Key features include:

#### 2.1.1 Tablespace Creation
The project creates a dedicated tablespace, `movie_db_tbs`, to logically separate and optimize the storage of project-specific tables and indexes. 
This aims to :
- Improves data organization.
- Facilitates scalability by isolating movie-related data from the default Oracle tablespace.
- Enhances performance for queries by grouping related data structures.

#### 2.1.2 User and Role Management
User management is automated through stored procedures and role-based access control:

- Roles:
    - `movie_db_user`: Grants read-only permissions to query tables and views.
    - `movie_db_admin`: Provides read-write access, including permissions to create, modify, and delete database objects.

- Procedures:
    - `CreateUser`: Automates user creation, assigning appropriate roles (`movie_db_user` or `movie_db_admin`) based on admin status. It also logs user details into the `Users_Records` table.
    - `DeleteUser`: Deletes users and removes their records from the `Users_Records` table, ensuring synchronization between SQL users and the database.

> **NOTE:** The script will automatically create two default users, the first “spectator”, a normal user, and the other “admin” with administrative privileges.

#### 2.1.3 Table Creation
The migration script defines the schema for storing movies, user reviews, tags, and other related data:

- **Movie Table:** Stores essential movie information such as title, release_date, budget, revenue, and genres.
- **Ratings Table:** Tracks user ratings for movies, including the rating value and timestamp.
- **Tags Table:** Captures user-generated tags, acting as short comments or categorizations for movies.
- **Link Table:** Establishes a relationship between MovieLens and TMDb datasets by mapping movie_id to tmdb_id. This table is pivotal in integrating user interaction data from MovieLens with the enriched metadata from TMDb.
- **MovieLens_Users Table:**
Stores the list of unique users in the MovieLens dataset. The table is populated by the migration script, listing the ids of all users who have interacted in the ratings and tags tables.


**Tables are configured to utilize the movie_db_tbs tablespace, ensuring optimal performance and logical data separation.**

#### 2.1.4 Batch Data Insertion
To handle large datasets efficiently, batch processing is used during the migration:

- Allows up to 32,768 rows to be inserted per execution, reducing I/O overhead.
- Implements duplicate filtering to ensure data integrity during insertion.

#### 2.1.5 Views for Query Simplification
Views aggregate and structure data for common use cases

#### 2.1.6 Index Optimization
Indexes are strategically created to enhance query performance:

**Example:** The TMDB_movie_dataset_title_idx index on the title column significantly improves search operations and sorting by movie title.

#### 2.1.7 Public Synonyms
Public synonyms simplify access by eliminating the need for schema prefixes:

Example: A synonym for the TMDB_movie_dataset table allows users to query directly using SELECT * FROM TMDB_movie_dataset without specifying the schema.

Synonyms are also created for views and stored procedures to standardize access across users.

#### 2.1.8 Stored Procedures 

Procedures are automatically created by the migration script to streamline database operations. They handle user management, statistical retrieval, and data cleanup for `MovieLens` interactions.

#### 2.1.9 Triggers

Triggers are implemented to automatically ensure data integrity. They guarantee cascading deletions for dependent data when associated data is deleted.

### 2.2 Application

The application is designed to provide a user-friendly interface for seamless interaction with the database. Built with Tauri, it relies on a two-part architecture:

- **Frontend:** Developed with Svelte, the frontend handles the user interface and interactive experience.
- **Backend:** Implemented in Rust, the backend manages all logic, including executing SQL queries, user authentication, and data management. Rust functions act as a bridge between the frontend and the Oracle SQL database.

All SQL operations and database connections are handled through Rust, while Svelte components make API calls to the associated Rust functions.

## 3. Data Structure

### 3.1 Tables
The project relies on several essential tables to organize and manage the data. Below is a list of the main tables and their purposes:

- **TMDB_movie_dataset**

```sql
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
) TABLESPACE movie_db_tbs
```

Stores detailed information about movies, such as title, release date, budget, revenue, posters, genres, etc.
Provides enriched metadata for each movie, used for display and analysis within the application.

An index is created on the title to accelerate title-based searches.

- **MovieLens_Ratings**

```sql
CREATE TABLE MovieLens_Ratings (
    user_id NUMBER,
    movie_id NUMBER,
    rating NUMBER(2, 1),
    timestamp NUMBER
) TABLESPACE movie_db_tbs
```


Contains user ratings for movies.
Facilitates calculations of average ratings, ranks movies by popularity, and analyzes user preferences.

Indexes are created on user_id and movie_id.


- **MovieLens_Tags**

```sql
CREATE TABLE MovieLens_Tags (
    user_id NUMBER,
    movie_id NUMBER,
    tag VARCHAR2(255),
    timestamp NUMBER
) TABLESPACE movie_db_tbs
```

Stores tags assigned by users to movies.

Indexes are created on user_id and movie_id.

- **MovieLens_Users**

```sql
CREATE TABLE MovieLens_Users (
    user_id NUMBER PRIMARY KEY
) TABLESPACE movie_db_tbs
```

Stores all unique MovieLens users encountered in the MovieLens_Ratings and MovieLens_Tags datasets. This table improves user tracking and management.

- **MovieLens_Links**

```sql
CREATE TABLE MovieLens_Links (
    movie_id NUMBER PRIMARY KEY,
    imdb_id VARCHAR2(20),
    tmdb_id NUMBER
) TABLESPACE movie_db_tbs
```

Associates user interaction data from MovieLens with detailed metadata from TMDb.

Indexes are created on movie_id and tmdb_id.

- **Users_Records**

```sql
CREATE TABLE Users_Records (
    username VARCHAR2(128) PRIMARY KEY,
    is_admin NUMBER(1) NOT NULL CHECK (is_admin IN (0, 1)),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
) TABLESPACE movie_db_tbs
```

This table catalogs all SQL users created within the database and tracks their access levels (administrator or standard user).

Unlike the MovieLens_Users table, which catalogs user IDs from the MovieLens dataset, Users_Records is dedicated to managing and auditing SQL database users created within the system. This table acts as a comprehensive audit trail, enabling secure access control and privilege verification, ensuring that administrative and user roles are properly tracked and enforced.

### 3.2 Views

Views simplify complex SQL queries by aggregating or transforming data for specific use cases. Below are the main views and their purposes:

- **MovieDetailsView**

```sql
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
        MovieLens_Links l ON l.tmdb_id = tmd.id
```

Aggregates detailed information about a movie by combining TMDb and MovieLens data.

- **MovieShortView**

```sql
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
        MovieLens_Links l ON l.tmdb_id = tmd.id
```

Provides a subset of key information about movies (title, popularity, genres, posters, etc.).

- **MovieShortView_Alphabetical**

```sql
CREATE OR REPLACE VIEW MovieShortView_Alphabetical AS
    SELECT *
    FROM MovieShortView
    ORDER BY title ASC
```

Sorts movies alphabetically by title.

- **MovieShortView_ByPopularity**

```sql
CREATE OR REPLACE VIEW MovieShortView_ByPopularity AS
    SELECT *
    FROM MovieShortView
    ORDER BY popularity DESC
```

Orders movies by descending popularity.

- **MovieShortView_ByReleaseDate**

```sql
CREATE OR REPLACE VIEW MovieShortView_ByReleaseDate AS
    SELECT *
    FROM MovieShortView
    ORDER BY release_date DESC
```

Orders movies by release date (most recent first).

- **MovieShortView_ByRating**

```sql
CREATE OR REPLACE VIEW MovieShortView_ByRating AS
    SELECT *
    FROM MovieShortView
    ORDER BY vote_average DESC
```

Ranks movies based on their average rating in descending order.

- **TopUsers**

```sql
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
        total_contributions DESC
```

Displays the most active users by combining their ratings and tags contributions.

### 3.4 Triggers

The following triggers are used to maintain data integrity :

- `trg_delete_user_cleanup`

```sql
CREATE OR REPLACE TRIGGER trg_delete_user_cleanup
    AFTER DELETE ON MovieLens_Users
    FOR EACH ROW 
BEGIN
    -- Supprimer les ratings associés à l'utilisateur
    DELETE FROM MovieLens_Ratings WHERE user_id = :OLD.user_id;
            
    -- Supprimer les tags associés à l'utilisateur
    DELETE FROM MovieLens_Tags WHERE user_id = :OLD.user_id;
END;
```

Automatically deletes all ratings and tags associated with a user when they are removed from the MovieLens_Users table.

- `trg_delete_movie_cleanup`

```sql
CREATE OR REPLACE TRIGGER trg_delete_movie_cleanup
    AFTER DELETE ON MovieLens_Links
    FOR EACH ROW
BEGIN
    -- Supprimer les ratings associés au film
    DELETE FROM MovieLens_Ratings WHERE movie_id = :OLD.movie_id;
            
    -- Supprimer les tags associés au film
    DELETE FROM MovieLens_Tags WHERE movie_id = :OLD.movie_id;
END;
```

Ensures that all ratings and tags related to a movie are deleted when the corresponding entry in the MovieLens_Links table is removed.

## 4. Stored Procedures

Stored procedures automate complex database operations, ensuring consistency, security, and ease of use. These procedures handle critical tasks such as SQL user management, MovieLens data maintenance, and statistical analysis.

### 4.1 SQL User Management Procedures

- `CreateUser`

```sql
CREATE OR REPLACE PROCEDURE CreateUser(
    username IN VARCHAR2,
    password IN VARCHAR2,
    is_admin IN NUMBER
) AUTHID CURRENT_USER AS BEGIN
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
```

Automates SQL user creation by assigning roles (`movie_db_user` by default, `movie_db_admin` for administrators) and recording their details, including admin status and creation timestamp, in the `Users_Records` table for auditing.

> This procedure is only accessible to users with the administrator role.

- `DeleteUser`

```sql
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
```

Deletes a SQL user using `DROP USER` and removes their record from the `Users_Records` table, ensuring data integrity and handling potential errors during the process.

> This procedure is only accessible to users with the administrator role.

### 4.2 Data Analysis and Statistics Procedures
- `GetStats`

```sql
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
END;
```

This procedure provides both quantitative statistics and rankings for the database. Quantitative statistics include the total number of movies, user ratings, tags, and unique users, which are returned as output parameters. Rankings, on the other hand, are delivered through cursors (database query pointers that allow row-by-row traversal). These rankings encompass data such as the number of movies categorized by genre, the most active contributors based on their activity, and the most profitable movies.

### 4.3 MovieLens Data Management Procedures

- `DeleteMovieLensUser`

```sql
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
```

This procedure manages the deletion of users from the `MovieLens_Users` table. By removing a user based on their `user_id`, it relies on triggers to automatically clean up related data, such as ratings and tags associated with the user. It ensures consistency across the dataset.

> This procedure is only accessible to users with the administrator role.

- `DeleteMovieLensTag`

```sql
CREATE OR REPLACE PROCEDURE DeleteMovieLensTag(
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

    EXCEPTION WHEN OTHERS THEN
        -- Gérer les erreurs éventuelles
        RAISE_APPLICATION_ERROR(
            -20003,
            'Erreur lors de la suppression du tag pour user_id=' || u_id || ', movie_id=' || m_id || ', timestamp=' || tag_timestamp || ': ' || SQLERRM
        );
END;
```

This procedure allows precise removal of a tag from the `MovieLens_Tags` table. By specifying the `user_id`, `movie_id`, and `timestamp`, it targets and deletes the desired record.

> This procedure is only accessible to users with the administrator role.

## 5. Application Features

### 5.1 Movie Search, Sorting, and Filtering

The core feature of the application is the ability to explore the vast collection of recorded movies through an advanced search system. Users can easily navigate the database using filtering and sorting options tailored to their preferences. The SQL query at the heart of this system is:

```sql 
SELECT movie_id, tmdb_id, title, vote_average, poster_path
FROM {VIEW_NAME}
WHERE LOWER(title) LIKE :1
{GENRE_FILTER}
FETCH FIRST 100 ROWS ONLY
```

- **Search by Title :** When a user enters a keyword, the application dynamically generates a case-insensitive search query. This ensures that all movies containing the keyword in their titles, regardless of letter casing, are included in the results. The `LIKE` operator is leveraged to match titles partially, enabling flexible search results.

- **Filter by Genre :** To refine the search, users can filter movies by genre. If a specific genre is selected, the {GENRE_FILTER} placeholder in the SQL query is dynamically replaced with a condition that uses DBMS_LOB.INSTR. This function checks whether the specified genre is present in the movie's genre list stored in the database.

- **Sorting Options :** The application allows users to sort movies using various predefined criteria. This is achieved by leveraging SQL views specifically designed for each sorting option:

    - **Alphabetical Order:** Uses the `MovieShortView_Alphabetical` view to sort movies by their titles in ascending order.

    - **Popularity:** Utilizes the `MovieShortView_ByPopularity` view to rank movies based on their popularity scores, with the most popular movies appearing first.

    - **Release Date:** Employs the `MovieShortView_ByReleaseDate` view to display movies starting with the most recent releases.

    - **Top Rated:** Leverages the `MovieShortView_ByRating` view to sort movies by their average user ratings in descending order, highlighting the highest-rated films.

### 5.2 Movie Details Page

The Movie Details Page provides enriched information about specific films in the database. By integrating detailed movie metadata, user ratings, and comments (tags), this page delivers a comprehensive overview of the film. Below are the queries used to retrieve these distinct pieces of information :

```sql
SELECT 
    movie_id,
    tmdb_id,
    title,
    vote_average,
    vote_count,
    status,
    TO_CHAR(release_date, 'YYYY-MM-DD') AS release_date,
    runtime,
    adult,
    backdrop_path,
    overview,
    poster_path,
    genres,
    keywords
FROM 
    MovieDetailsView
WHERE 
    movie_id = :1
```
This query retrieves detailed metadata for a specific movie, including its title, release date, runtime, genres, description, and visual elements such as poster and backdrop images. It forms the core of the movie details section by providing all necessary information about the movie itself.

```sql 
SELECT 
    user_id, 
    rating, 
    timestamp
FROM 
    MovieLens_Ratings
WHERE 
    movie_id = :1
```

This query fetches user ratings for the selected movie, including the user ID, rating score, and the timestamp of when the rating was provided. These ratings are used to calculate aggregated statistics such as the average score and total number of votes.

```sql
SELECT 
    user_id, 
    tag, 
    timestamp
FROM 
    MovieLens_Tags
WHERE 
    movie_id = :1
```

This query retrieves user-submitted tags (comments or keywords) associated with the movie. Each tag includes the user ID who submitted it, the tag content, and the timestamp. These tags provide qualitative insights into user impressions of the movie.



**Features of the Movie Details Page :**
- **Detailed Movie Metadata:** The page displays a wealth of information from MovieDetailsView, including the title, TMDb ratings, genre tags, runtime, release date, and a short synopsis. Visual assets such as the poster and backdrop enhance the user experience by providing a cinematic preview.

- **TMDb Ratings and Popularity:** TMDb data offers insights into the movie’s popularity and quality through its average rating, vote count, and overall reception.

- **User Ratings from MovieLens:** The integration of MovieLens_Ratings enables users to see detailed scores provided by the community, helping establish a broader context of user preferences and perceptions.

- **User Tags for Contextual Feedback:** Tags from the MovieLens_Tags table are also included, providing a form of user commentary. These tags often describe the movie in succinct terms, helping other users quickly identify key themes, elements, or reactions associated with the movie.

### 5.3 Statistics and Analysis

The Statistics and Analysis page provides users with a comprehensive view of the database's key metrics and insights. The data displayed on this page is retrieved using the GetStats stored procedure, executed via the following PL/SQL block:

```sql
DECLARE
    genre_count SYS_REFCURSOR;
    top_users SYS_REFCURSOR;
    top_profits_films SYS_REFCURSOR;
BEGIN
    GetStats(:1, :2, :3, :4, genre_count, top_users, top_profits_films);
    DBMS_SQL.RETURN_RESULT(genre_count);
    DBMS_SQL.RETURN_RESULT(top_users);
    DBMS_SQL.RETURN_RESULT(top_profits_films);
END;
```

**This stored procedure organizes and returns a variety of data, categorized as follows:**

- **Quantitative Statistics :** The procedure retrieves and returns key metrics, including:

    - Total number of movies in the database.
    - Total user ratings recorded.
    - Total tags submitted by users.
    - Number of distinct users interacting with the system.

- **Genre-Based Analysis :** Through the `genre_count` cursor, the procedure provides a detailed breakdown of movies by genre. This enables users to see which genres are most prevalent and supports further genre-specific analyses.

- **Top Contributors :** The `top_users` cursor identifies the most active users on the platform, ranking them by their total contributions, including ratings and tags. This highlights the community's key contributors.

- **Top Profitable Movies :** The `top_profits_films` cursor lists movies with the highest profit margins, calculated as the difference between revenue and budget. This functionality allows users to easily identify commercially successful movies.

### 5.4 User Management

The User Management section of the application provides administrators with the necessary tools to efficiently manage SQL users. This functionality ensures secure access control through user interfaces and role-based permissions within the database system. With an intuitive interface, administrators can easily create and delete users. The main features of this module are as follows:

- **User Listing**

```sql
SELECT
    username, 
    is_admin,
    created_at
FROM 
    USERS_RECORDS
ORDER BY 
    created_at DESC
```

Retrieve a comprehensive list of users, including their roles and creation timestamps, to simplify auditing and privilege management.

- **User Creation**

```sql
BEGIN
    /*
    * Arguments:
    * 1: username
    * 2: password
    * 3: is_admin (0 or 1)
    */

    CreateUser(:1, :2, :3);
END;
```

Create new users with predefined roles (`movie_db_user` for read-only access and `movie_db_admin` for administrative privileges), while automatically logging their details in the `Users_Records` table to ensure secure access control.

- **User Deletion**

```sql
BEGIN
    /*
    * Arguments:
    * 1: username
    */

    DeleteUser(:1);
END;
```

Safely delete users, ensuring their records are removed from the system while handling errors such as non-existent users.


### 5.5 MovieLens Data Management

The application includes advanced features for administrators to manage MovieLens data directly from the interface, providing control over user interactions and ensuring data integrity.

- **Delete Tag** 
```sql
BEGIN
     -- Arguments: movie_id, user_id, timestamp
    DeleteMovieLensTag(:1, :2, :3);
END;
```
Removes the specific tag from the database

- **Delete User** 
```sql
BEGIN
    -- Arguments: user_id
    DeleteMovieLensUser(:1);
END;
```

Deletes the MovieLens user and associated data (tags and ratings)


## 6. Installation Guide

This section provides detailed instructions for setting up the project, including prerequisites, installation steps, and configuration.

### 6.1 Prerequisites
Before starting, ensure you have the following tools and software installed on your system:

- Oracle Database XE: For hosting the database.

Being on MacOS, the database was configured using Colima. Follow the guide available at [Running Oracle Database on Docker on Apple M1 Chip](https://oralytics.com/2022/09/22/running-oracle-database-on-docker-on-apple-m1-chip/) for installation.

- Rust Programming Language: Ensure `cargo` is installed for building and running the backend logic.
- Tauri Development Framework: For creating the application's GUI.
- Node.js: Required for building the frontend using Svelte.

### 6.2 Installation

1. **Clone the Project Repository**

2. **Install Dependencies:**
Ensure you install all necessary dependencies for both the migration script and the Tauri-based application.

    - **For the Migration Script:** The migration script is written in Rust, and you need cargo installed to build and execute it:
    ```shell
    rustup update
    ```

    - **For the Tauri GUI Application:** Install Tauri CLI and other dependencies for the graphical interface:

    ```shell
    cargo install create-tauri-app --locked
    npm install
    ```

### 6.3 Configuration

After installing the required dependencies, configure the database by running the migration script. This script automates the entire setup process, including creating the tablespace, tables, views, stored procedures, roles, and users.

1. **Prepare the Environment File:**

Create a `.env` file at the root of the project to provide the necessary credentials for the database administrator. The file should include the following variables:

```
USERNAME=<Administrator username>
PASSWORD=<Administrator password>
HOST=<Database Host>
```

- Replace `<Administrator username>` and `<Administrator password>` with the credentials of a database user that has elevated privileges (e.g., the `SYSTEM` user).

- Replace `<Database Host>` with the address of your Oracle SQL instance (e.g., `//localhost:1521/XEPDB1`)

> **Note:** The provided user must have administrative rights capable of creating tablespaces, tables, users, roles, managing privileges...

2. **Run the Migration Script:**

Execute the migration script using Cargo to initialize and configure the database:

```shell
cargo run
```
The script will automatically:

- Create the necessary tablespace, tables, views, and triggers.
- Configure roles and user management.
- Set up permissions and populate essential data structures.

Confirmation messages will appear in the terminal upon successful execution, indicating that the database is ready for use.

> **NOTE:** The script will automatically create two default users, the first “spectator”, a normal user, and the other “admin” with administrative privileges. Both have the default password 'pass', which can be changed directly in the migration script.

3. **Start the Application:**
Navigate to the application folder, and run the following command to launch the development environment:

```shell
cargo tauri dev
```

Ensure you have installed all dependencies required for Tauri and the Rust backend (as detailed in the installation section).

By following these steps, you will have a fully configured database and application ready for exploration and management.

> Try to connect using the default accounts: `admin` (admin role) or `spectator` (read-only role), both with the password `"pass"` !