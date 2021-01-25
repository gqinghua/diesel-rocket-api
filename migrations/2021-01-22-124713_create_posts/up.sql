-- Your SQL goes here
CREATE TABLE sys_user (
  id SERIAL PRIMARY KEY ,
  account  VARCHAR ,
  password varchar ,
 name varchar ,
  del int ,
  create_date timestamp
)
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
)