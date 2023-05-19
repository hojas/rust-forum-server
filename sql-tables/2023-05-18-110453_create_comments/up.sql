CREATE TABLE comments
(
    id                SERIAL PRIMARY KEY,
    content           TEXT      NOT NULL,
    author_id         INTEGER   NOT NULL REFERENCES users(id),
    post_id           INTEGER   NOT NULL REFERENCES posts(id),
    parent_comment_id INTEGER REFERENCES comments(id),
    created_at        TIMESTAMP NOT NULL DEFAULT NOW()
)
