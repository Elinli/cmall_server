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


CREATE TYPE effect_status AS ENUM(
    'enable',
    'disable'
);

-- generate role table from Role struct and EffectStatus enum
CREATE TABLE IF NOT EXISTS roles (
    id BIGSERIAL PRIMARY KEY,
    code VARCHAR(64) NOT NULL,
    name VARCHAR(64) NOT NULL,
    status effect_status NOT NULL,
    description TEXT NOT NULL,
    create_time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    create_by VARCHAR(64) NOT NULL,
    update_time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    update_by VARCHAR(64) NOT NULL
);

-- CREATE INDEX role_code_index ON roles(code);
CREATE UNIQUE INDEX IF NOT EXISTS role_code_index ON roles(code);


-- create department table
CREATE TABLE IF NOT EXISTS departments (
    id BIGSERIAL PRIMARY KEY,
    identifier VARCHAR(64) NOT NULL,
    name VARCHAR(64) NOT NULL,
    status effect_status NOT NULL,
    description TEXT NOT NULL,
    create_time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    create_by VARCHAR(64) NOT NULL,
    update_time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    update_by VARCHAR(64) NOT NULL
);

-- create index for department for identifier
CREATE UNIQUE INDEX IF NOT EXISTS department_identifier_index ON departments(identifier);

CREATE TYPE menu_type AS ENUM(
  'menu',
  'link',
  'button'
);

-- create menu table
CREATE TABLE IF NOT EXISTS menus (
    id BIGSERIAL PRIMARY KEY,
    menu_id VARCHAR(64) NOT NULL,
    path VARCHAR(64) NOT NULL,
    chinese_name VARCHAR(64) NOT NULL,
    english_name VARCHAR(256) NOT NULL,
    icon VARCHAR(64) DEFAULT '',
    order_num INT NOT NULL,
    type menu_type NOT NULL,
    parent_menu_id VARCHAR(64) NOT NULL DEFAULT 'root',
    status effect_status NOT NULL,
    description TEXT NOT NULL,
    create_time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    create_by VARCHAR(64) NOT NULL,
    update_time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    update_by VARCHAR(64) NOT NULL
);

-- create index for menu for menu_id
CREATE UNIQUE INDEX IF NOT EXISTS menu_id_index ON menus(menu_id);

ALTER TABLE users ADD CONSTRAINT fk_dept_id FOREIGN KEY (dept_id) REFERENCES departments(id);

-- 添加用户角色关系表
CREATE TABLE IF NOT EXISTS user_roles (
    user_id BIGINT NOT NULL,
    role_id BIGINT NOT NULL,
    PRIMARY KEY (user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (role_id) REFERENCES roles(id)
);
