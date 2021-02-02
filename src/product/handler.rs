#![feature(proc_macro_hygiene, decl_macro)]


use diesel::prelude::*;
use diesel::pg::PgConnection;
use rocket::request::Form;
use rocket_contrib::json::Json;

use dotenv::dotenv;
use std::env;
use log::{info};
use crate::models::model::{Post, NewPost, UpdatePost, SysUser,SysUserAO,UPdateSysUser,SysRole};
use rocket::Rocket;
use crate::db::pool::pg_connection;
use anyhow::Result;
use rocket::response::status;
use crate::crud::users;
use crate::models::response::{Response,ResponseWithStatus};
use rocket::http::Status;

// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();
//
//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }

#[derive(FromForm)]
struct ReadPostParams {
    is_published: Option<bool>,
    limit: Option<i64>,
}

#[get("/posts?<read_post_params..>")]
fn read(read_post_params: Form<ReadPostParams>) -> Json<Vec<Post>> {
    use super::super::schema::posts::dsl::{posts, published};

    let is_published = match read_post_params.is_published {
        Some(v) => v,
        None => true,
    };

    let limit = match read_post_params.limit {
        Some(v) => v,
        None => 5,
    };

    let connection = pg_connection();
    let results = posts
        .filter(published.eq(is_published))
        .limit(limit)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    Json(results)
}

#[post("/posts", data = "<post>")]
fn create(post: Json<NewPost>) -> Result<Json<Post>> {
    use super::super::schema::posts;

    let new_post = NewPost {
        title: &post.title,
        body: &post.body,
    };
    info!("Razor id: {}", new_post.body);
    let connection = pg_connection();
    let result: Post = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&connection)
        .expect("Error saving new post");

    Ok(Json(result))
}

#[get("/posts/<id>")]
fn read_detail(id: i32) -> Result<Json<Post>> {
    use super::super::schema::posts::dsl::{posts};

    info!("Razor id: {}", id);
    let connection = pg_connection();
    let result = posts
        .find(id)
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));
   Ok(Json(result))
}

#[get("/sysUser/<id>")]
fn sysUserById(id: i32) -> Result<Json<SysUser>> {
    use super::super::schema::sys_user::dsl::{sys_user};
    info!("Razor id: {}", id);
    let connection = pg_connection();
    let result = sys_user
        .find(id)
        .get_result(&connection)
        .expect("Unable to find post");
    Ok(Json(result))
}

#[get("/sysRoleById/<id>")]
fn sysRoleById(id: i32) -> Result<Json<SysRole>> {
    use super::super::schema::sys_role::dsl::{sys_role};
    info!("Razor id: {}", id);
    let connection = pg_connection();
    let result = sys_role
        .find(id)
        .get_result(&connection)
        .expect("Unable to find post");
    Ok(Json(result))
}

#[post("/CreateSysuser", data = "<SysUserAO>")]
fn createSysUser(SysUserAO: Json<SysUserAO>) -> Result<Json<SysUser>> {
    use super::super::schema::sys_user;

    let CreaSysUserAO = SysUserAO {
        account: &SysUserAO.account,
        password: &SysUserAO.password,
        name: &SysUserAO.name,
        del: &SysUserAO.del,
    };
    info!("Razor id: {}", CreaSysUserAO.name);
    let connection = pg_connection();
    let result: SysUser = diesel::insert_into(sys_user::table)
        .values(CreaSysUserAO)
        .get_result(&connection)
        .expect("Error saving new post");

    Ok(Json(result))
}
#[post("/createSysusers/<ids>", data = "<SysUserAOUpdate>")]
fn UPdateSysUser(ids: i32,SysUserAOUpdate: Json<UPdateSysUser>) -> Result<Json<SysUser>> {
    let connection = pg_connection();
    let updated_user = users::update(ids,connection, SysUserAOUpdate.0)?;
    Ok(Json(updated_user))
}

#[patch("/posts/<id>", data = "<post>")]
fn update_detail(id: i32, post: Json<UpdatePost>) -> Result<Json<Post>> {
    use super::super::schema::posts::dsl::{posts,published};

    let is_published = match &post.published {
        Some(v) => v,
        None => &false,
    };

    let connection = pg_connection();
    let result = diesel::update(posts.find(id))
        .set(published.eq(is_published))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));

    Ok(Json(result))
}

#[delete("/posts/<id>")]
fn delete_detail(id: i32) -> Result<Json<Post>> {
    // use schema::posts::dsl::{posts};
    use super::super::schema::posts::dsl::{posts};

    let connection = pg_connection();
    let result = diesel::delete(posts.find(id))
        .get_result::<Post>(&connection)
        .expect("Error deleting posts");

    Ok(Json(result))
}




pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount("/", routes![sysRoleById,
    read,sysUserById,createSysUser, create, read_detail, update_detail, delete_detail,UPdateSysUser])
}