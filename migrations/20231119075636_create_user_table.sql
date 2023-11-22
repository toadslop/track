CREATE TABLE user_ (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    user_id VARCHAR (50) UNIQUE,
    password TEXT NOT NULL,
    nickname VARCHAR (30),
    comment VARCHAR (20),
    created_at TIMESTAMP NOT NULL
);