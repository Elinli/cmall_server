-- Add migration script here
-- create user table

-- create user status type
CREATE TYPE user_status AS ENUM(
  'active',
  'off',
  'offline',
  'online'
);

CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    dept_id BIGINT NOT NULL,
    username VARCHAR(64) NOT NULL,
    password_hash VARCHAR(128) NOT NULL,
    email VARCHAR(64) NOT NULL,
    phone VARCHAR(32) NOT NULL,
    avatar VARCHAR(255) DEFAULT 'default_avatar',
    status user_status NOT NULL,
    roles BIGINT[] DEFAULT '{}',
    create_time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);


-- create index for users for email
CREATE UNIQUE INDEX IF NOT EXISTS email_index ON users(email);

