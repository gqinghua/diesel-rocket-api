-- Your SQL goes here

CREATE TABLE sys_user_role (
  id SERIAL PRIMARY KEY,
  user_id varchar(45) ,
  role_id varchar(45) ,
  create_date VARCHAR
)