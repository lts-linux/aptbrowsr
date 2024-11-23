use actix_web::error::{ErrorBadRequest};
use actix_web::{get, web, HttpResponse, Responder, post};

use libapt::Distro;

use storage::distro::{get_distros, create_distro};

use crate::DbPool;

#[get("/")]
async fn list_distros(
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let distros = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        get_distros(&mut conn, None)
    })
    .await?;

    Ok(HttpResponse::Ok().json(distros))
}

#[post("/")]
async fn new_distro(
    pool: web::Data<DbPool>,
    new_distro: web::Json<Distro>
) -> actix_web::Result<impl Responder> {

    let id = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        create_distro(&mut conn, new_distro.into_inner())
    })
    .await?;

    let id = id.map_err(|e| ErrorBadRequest(e))?;

    Ok(HttpResponse::Ok().json(id))
}
