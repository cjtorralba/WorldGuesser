-- Add up migration script here
CREATE TABLE IF NOT EXISTS user_creds
(
    id  serial PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    is_admin  BOOLEAN default false
)
