use anyhow::Result;
use diesel::{insert_into, RunQueryDsl, QueryDsl};
use diesel::pg::PgConnection;
use crate::models::model::{ SysUser,SysRole,SysUserRoleAO,SysUserRole};
use crate::models::model::UPdateSysUser;
use super::super::schema::sys_user::dsl::{sys_user};
use super::super::schema::sys_role::dsl::{sys_role};
use super::super::schema::sys_user_role::dsl::{sys_user_role};

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

pub fn createSysUserRole(SysUserRoleAO :SysUserRoleAO,db: PgConnection) -> Result<SysUserRole> {
    let result: SysUserRole = diesel::insert_into(sys_user_role)
          .values(SysUserRoleAO)
          .get_result(&db)
          .expect("Error saving new post");
    Ok(result)
}



pub fn queryUserRoleId(id: i32,db: PgConnection) -> Result<SysUserRole> {

    let result = sys_user_role
        .find(id)
        .get_result(&db)
        .expect("Unable to find post");
    Ok(result)
}