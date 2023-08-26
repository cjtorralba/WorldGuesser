-- Add up migration script here
CREATE TABLE IF NOT EXISTS users
(
    id  serial PRIMARY KEY,
    rank INTEGER default 0,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    is_admin  BOOLEAN default false
)
