use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use libapt::Distro;

pub mod models;
pub mod schema;
pub mod distro;

use self::models::{Repo, NewRepo};


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_distros(conn: &mut SqliteConnection, limit: Option<i64>) -> Vec<Distro> {
    use self::schema::distros::dsl::*;

    let repos: Vec<Repo> = if let Some(l) = limit {
        distros
            .select(Repo::as_select())
            .limit(l)
            .load(conn)
            .expect("Error loading distros")
    } else {
        distros
            .select(Repo::as_select())
            .load(conn)
            .expect("Error loading distros")
    };
    
    repos.into_iter().filter_map(|r| r.to_distro()).collect()    
}

pub fn create_repo(
    conn: &mut SqliteConnection,
    url: &str,
    distro_name: &str,
    distro_key: Option<&str>,
    armored: bool,
) -> Option<Distro> {
    use crate::schema::distros;

    let new_repo = NewRepo { 
        url: url, 
        name: Some(distro_name),
        path: None, 
        key: distro_key, 
        armored_key: armored
    };

    let repo: Repo = diesel::insert_into(distros::table)
        .values(&new_repo)
        .returning(Repo::as_returning())
        .get_result(conn)
        .expect("Error saving new distro");

    repo.to_distro()
}

pub fn create_flat_repo(
    conn: &mut SqliteConnection,
    url: &str,
    distro_path: &str,
    distro_key: Option<&str>,
    armored: bool,
) -> Option<Distro> {
    use crate::schema::distros;

    let new_repo = NewRepo { 
        url: url, 
        name: None,
        path: Some(distro_path), 
        key: distro_key, 
        armored_key: armored };

        let repo: Repo = diesel::insert_into(distros::table)
        .values(&new_repo)
        .returning(Repo::as_returning())
        .get_result(conn)
        .expect("Error saving new flat distro");

    repo.to_distro()
}
