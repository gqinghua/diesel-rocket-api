#![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use] extern crate rocket;
// #[macro_use] extern crate diesel;


use diesel::prelude::*;
use diesel::pg::PgConnection;
use rocket::request::Form;
use rocket_contrib::json::Json;
// use models::models::{Post, NewPost, UpdatePost,SysUser};
// use crate::models::model;
// use crate::models::model::{Post, NewPost, UpdatePost,SysUser};
use dotenv::dotenv;
use std::env;
use log::{info};
use crate::models::model::{Post, NewPost, UpdatePost,SysUser};
use rocket::Rocket;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(FromForm)]
struct ReadPostParams {
    is_published: Option<bool>,
    limit: Option<i64>,
}

#[get("/posts?<read_post_params..>")]
fn read(read_post_params: Form<ReadPostParams>) -> Json<Vec<Post>> {
    // use schema::posts::dsl::{posts, published};
    use super::super::schema::posts::dsl::{posts, published};

    let is_published = match read_post_params.is_published {
        Some(v) => v,
        None => true,
    };

    let limit = match read_post_params.limit {
        Some(v) => v,
        None => 5,
    };

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(is_published))
        .limit(limit)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    Json(results)
}

#[post("/posts", data = "<post>")]
fn create(post: Json<NewPost>) -> Json<Post> {
    // use schema::posts;
    use super::super::schema::posts;

    let new_post = NewPost {
        title: &post.title,
        body: &post.body,
    };

    let connection = establish_connection();
    let result: Post = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&connection)
        .expect("Error saving new post");

    Json(result)
}

#[get("/posts/<id>")]
fn read_detail(id: i32) -> Json<Post> {
    // use schema::posts::dsl::posts;
    use super::super::schema::posts::dsl::{posts};

    info!("Razor id: {}",  id);
    let connection = establish_connection();
    let result = posts
        .find(id)
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));
    Json(result)
}
#[get("/sysUser/<id>")]
fn sysUserById(id: i32) -> Json<SysUser> {
    // use schema::sys_user::dsl::sys_user;
    use super::super::schema::sys_user::dsl::{sys_user};
    info!("Razor id: {}",  id);
    let connection = establish_connection();
    let result = sys_user
        .find(id)
        .get_result(&connection)
        .expect(&format!("Unable to find post {}", id));
    Json(result)
}

#[patch("/posts/<id>", data = "<post>")]
fn update_detail(id: i32, post: Json<UpdatePost>) -> Json<Post> {
    // use schema::posts::dsl::{posts, published};
    use super::super::schema::posts::dsl::{posts,published};

    let is_published = match &post.published {
        Some(v) => v,
        None => &false,
    };

    let connection = establish_connection();
    let result = diesel::update(posts.find(id))
        .set(published.eq(is_published))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));

    Json(result)
}

#[delete("/posts/<id>")]
fn delete_detail(id: i32) -> Json<Post> {
    // use schema::posts::dsl::{posts};
    use super::super::schema::posts::dsl::{posts};

    let connection = establish_connection();
    let result = diesel::delete(posts.find(id))
        .get_result::<Post>(&connection)
        .expect("Error deleting posts");

    Json(result)
}
// fn main() {
//     // dotenv().ok();
//     // product::router::create_routes();
//     rocket::ignite()
//         .mount("/", routes![read,sysUserById, create, read_detail, update_detail, delete_detail])
//         .launch();
// }
pub fn fuel(rocket: Rocket) -> Rocket {
    // rocket.mount("/", routes![read,sysUserById, create, read_detail, update_detail, delete_detail])
    rocket.mount("/", routes![read,sysUserById, create, read_detail, update_detail, delete_detail])
}