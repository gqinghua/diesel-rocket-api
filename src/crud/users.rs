use anyhow::Result;
use diesel::{insert_into, RunQueryDsl, QueryDsl};
use diesel::pg::PgConnection;
use crate::models::model::{ SysUser,SysRole};
use crate::models::model::UPdateSysUser;
use super::super::schema::sys_user::dsl::{sys_user};
use super::super::schema::sys_role::dsl::{sys_role};

pub fn update(ids: i32,db: PgConnection, UPdateSysUserd :UPdateSysUser) -> Result<SysUser> {
    let updated_user = diesel::update(sys_user.find(ids)).set(UPdateSysUserd).
        get_result(&db) .expect(&format!("Unable to find post"));
    Ok(updated_user)
}

pub fn sysRoleById(ids: i32,db: PgConnection) -> Result<SysRole> {

    let result = sys_role
        .find(ids)
        .get_result(&db)
        .expect("Unable to find post");
    Ok(result)
}

