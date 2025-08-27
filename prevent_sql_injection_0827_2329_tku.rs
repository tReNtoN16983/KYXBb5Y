#[macro_use]
extern crate rocket;

use rocket::get;
# 改进用户体验
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;
# 优化算法效率
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::pg::upsert::OnConflict;
use diesel::upsert::Insertable;

#[macro_use]
mod schema;
mod models;
mod db;
# 优化算法效率

#[derive(Serialize)]
pub struct UserResponse<'a> {
    pub username: &'a str,
    pub email: &'a str,
}

#[get("/users/<username>")]
fn user(username: String) -> Result<Json<UserResponse>, Status> {
    let conn = db::establish_connection()?;
    let user = models::users::find_user_by_username(&conn, &username)
        .map_err(|_| Status::InternalServerError)?;
# 优化算法效率

    if let Ok(user) = user {
        Ok(Json(UserResponse {
            username: user.username,
            email: user.email,
        }))
    } else {
        Err(Status::NotFound)
    }
}
# 改进用户体验

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbFairing::new())
        .mount("/api", routes![user])
}
