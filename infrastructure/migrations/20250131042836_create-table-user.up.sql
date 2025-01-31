create table if not exists app_user
(
    id       uuid primary key,
    username text not null unique,
    email    text not null unique,
    password text not null
);