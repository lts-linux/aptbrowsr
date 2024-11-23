use diesel::prelude::*;
use libapt::Distro;

use crate::models::{DbDistro, NewDistro};

use crate::{Result, Error};

pub fn get_distros(conn: &mut SqliteConnection, limit: Option<i64>) -> Result<Vec<Distro>> {
    use crate::schema::distros::dsl::*;

    let repos: Vec<DbDistro> = if let Some(l) = limit {
        distros
            .select(DbDistro::as_select())
            .limit(l)
            .load(conn)
            .map_err(|e| Error::from_error(&e, "get_distros"))?
    } else {
        distros
            .select(DbDistro::as_select())
            .load(conn)
            .map_err(|e| Error::from_error(&e, "get_distros"))?
    };
    
    Ok(repos.into_iter().filter_map(|r| r.to_distro()).collect())
}

pub fn create_distro(
    conn: &mut SqliteConnection,
    distro: Distro,

) -> Result<i32> {
    use crate::schema::distros;

    let new_distro = match NewDistro::from_distro(distro) {
        Some(d) => Ok(d),
        None => Err(Error::new("create_distro: invalid data!"))
    }?;

    let d: DbDistro = diesel::insert_into(distros::table)
        .values(&new_distro)
        .returning(DbDistro::as_returning())
        .get_result(conn)
        .map_err(|e| Error::from_error(&e, "get_distros"))?;

    Ok(d.id)
}
