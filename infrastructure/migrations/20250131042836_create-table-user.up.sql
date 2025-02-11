CREATE TABLE IF NOT EXISTS app_user
(
    app_user_id UUID NOT NULL,
    username    TEXT NOT NULL,
    email       TEXT NOT NULL,
    password    TEXT NOT NULL,

    PRIMARY KEY (app_user_id),
    UNIQUE (username),
    UNIQUE (email)
);
