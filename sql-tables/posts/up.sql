CREATE TABLE posts
(
    id         SERIAL PRIMARY KEY,
    title      VARCHAR   NOT NULL,
    content    TEXT      NOT NULL,
    author_id  INTEGER   NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
)
