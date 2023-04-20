CREATE TABLE comments
(
    id                SERIAL PRIMARY KEY,
    content           TEXT      NOT NULL,
    author_id         INTEGER   NOT NULL,
    post_id           INTEGER   NOT NULL,
    parent_comment_id INTEGER,
    created_at        TIMESTAMP NOT NULL DEFAULT NOW()
)
