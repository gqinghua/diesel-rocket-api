use anyhow::Result;
use diesel::{insert_into, RunQueryDsl, QueryDsl};
use diesel::pg::PgConnection;
use crate::models::model::{UPdateSysUser, SysUser};
use crate::schemas::users::{UserCreate, UserUpdate};
use super::super::schema::sys_user::dsl::{sys_user};


pub fn update(ids: &i32,db: &PgConnection, UPdateSys: &UPdateSysUser) -> Result<SysUser> {
    let updated_user = diesel::update(sys_user.find(ids)).set(UPdateSys).get_result(db);
    Ok(updated_user)
}

