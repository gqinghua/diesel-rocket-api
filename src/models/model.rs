use serde::{Deserialize, Serialize};
use super::super::schema::sys_role;
use super::super::schema::sys_user_role;

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

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name="sys_user"]
pub struct UPdateSysUser{
    pub account: String,
    pub password: String,
    pub name: String,
    pub del: String,
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



#[derive(Serialize, Deserialize, Queryable)]
pub struct SysRole{
    pub id: i32,
    pub name: String,
    pub del: String,
    pub create_date: String,
    pub parent_id: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct SysUserRole{
    pub id: i32,
    pub userId: String,
    pub roleId: String,
    pub createDate: String,
}

#[derive(Deserialize, Insertable)]
#[table_name="sys_user_role"]
pub struct SysUserRoleAO<'a> {
    pub user_id: &'a str,
    pub role_id: &'a str,
    pub create_date: &'a str,
}
