

CREATE TABLE users
(
    id            serial PRIMARY KEY,
    username      text UNIQUE              NOT NULL,
    password_hash text                     NOT NULL,
    email         text UNIQUE              NOT NULL,
    created_at    timestamp with time zone NOT NULL default current_timestamp,
    updated_at    timestamp with time zone
);

CREATE TABLE inventory_items
(
    id          serial PRIMARY KEY,
    user_id     integer REFERENCES users (id),
    name        text                     NOT NULL,
    description text,
    quantity    integer                  NOT NULL,
    created_at  timestamp with time zone NOT NULL default current_timestamp,
    updated_at  timestamp with time zone
);
