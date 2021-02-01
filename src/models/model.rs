use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub published: bool,
}

use super::super::schema::posts;

#[derive(Deserialize, Insertable)] 
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Deserialize, Insertable)] 
#[table_name="posts"]
pub struct UpdatePost<'a> {
    pub title: Option<&'a str>,
    pub body: Option<&'a str>,
    pub published: Option<bool>,
}

use super::super::schema::sys_user;
#[derive(Serialize, Deserialize, Queryable)]
pub struct SysUser{
    pub id: i32,
    pub account: String,
    pub password: String,
    pub name: String,
    pub del: String,
}

#[derive(Deserialize, Insertable)]
#[table_name="sys_user"]
pub struct UPdateSysUser<'a> {
    pub account: &'a str,
    pub password: &'a str,
    pub name: &'a str,
    pub del: &'a str,
}

#[derive(Deserialize, Insertable)]
#[table_name="sys_user"]
pub struct SysUserAO<'a> {
    pub account: &'a str,
    pub password: &'a str,
    pub name: &'a str,
    pub del: &'a str,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct ResPageDTO {
    pub page: Option<u64>,
    pub size: Option<u64>,
}