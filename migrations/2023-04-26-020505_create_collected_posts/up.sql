CREATE TABLE collected_posts
(
    id         SERIAL PRIMARY KEY,
    user_id    INTEGER   NOT NULL,
    post_id    INTEGER   NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
)
