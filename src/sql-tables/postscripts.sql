CREATE TABLE postscripts
(
    id         SERIAL PRIMARY KEY,
    post_id    INTEGER   NOT NULL,
    content    TEXT      NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
)
