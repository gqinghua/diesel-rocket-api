use anyhow::Result;
use diesel::insert_into;
use diesel::pg::PgConnection;
use crate::models::model::{UPdateSysUser};
use crate::schemas::users::{UserCreate, UserUpdate};



pub fn update(ids: &i32,db: &PgConnection, obj_in: &UPdateSysUser) -> Result<User> {
    let updated_user = diesel::update(obj_in).set(obj_in).get_result(db)?;
    Ok(updated_user)
}

