-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(64) NOT NULL
);

CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    secret_key VARCHAR(255),
    filename VARCHAR(255) NOT NULL,
    user_id SERIAL NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TYPE access_type AS ENUM (
    'pending',
    'accepted',
    'rejected'
);

CREATE TABLE post_accesses (
    id SERIAL PRIMARY KEY,
    user_id SERIAL NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    post_id SERIAL NOT NULL REFERENCES posts (id) ON DELETE CASCADE,
    access_type access_type NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    text TEXT NOT NULL,
    from_id SERIAL NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    post_id SERIAL NOT NULL REFERENCES posts (id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);