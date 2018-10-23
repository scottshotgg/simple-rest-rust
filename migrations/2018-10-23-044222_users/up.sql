-- Your SQL goes here
create table users (
    id serial primary key,
    first_name varchar,
    last_name varchar,
    age int,
    hobby varchar,
    email varchar
)