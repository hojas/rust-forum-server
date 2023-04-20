CREATE TABLE users
(
    id            SERIAL PRIMARY KEY,
    email         VARCHAR   NOT NULL,
    password      VARCHAR   NOT NULL,
    username      VARCHAR   NOT NULL DEFAULT '',
    avatar_url    VARCHAR   NOT NULL DEFAULT '',
    signature     VARCHAR   NOT NULL DEFAULT '',
    role          VARCHAR   NOT NULL DEFAULT 'user',
    last_login_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_at    TIMESTAMP NOT NULL DEFAULT NOW()
)
