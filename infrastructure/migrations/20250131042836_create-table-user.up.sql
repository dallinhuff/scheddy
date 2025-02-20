CREATE TABLE IF NOT EXISTS app_user
(
    app_user_id UUID NOT NULL,
    email       TEXT NOT NULL,
    password    TEXT NOT NULL,
    full_name   TEXT NOT NULL,

    PRIMARY KEY (app_user_id),
    UNIQUE (email)
);
