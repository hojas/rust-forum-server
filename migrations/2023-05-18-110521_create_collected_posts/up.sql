CREATE TABLE collected_posts
(
    id         SERIAL PRIMARY KEY,
    user_id    INTEGER   NOT NULL REFERENCES users (id),
    post_id    INTEGER   NOT NULL REFERENCES posts (id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
)
