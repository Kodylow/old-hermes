-- Up
CREATE TABLE app_user (
    id SERIAL PRIMARY KEY,
    pubkey VARCHAR(64) NOT NULL,
    name VARCHAR(20) NOT NULL,
    dm_type VARCHAR(5) NOT NULL,
    federation_id VARCHAR(64) NOT NULL,
);
CREATE TABLE relay (
    id SERIAL PRIMARY KEY,
    relay VARCHAR(255) NOT NULL
);
CREATE TABLE app_user_relays (
    app_user_id INTEGER REFERENCES app_user(id),
    relay_id INTEGER REFERENCES relay(id),
    PRIMARY KEY (app_user_id, relay_id)
);
CREATE TABLE invoice (
    id SERIAL PRIMARY KEY,
    federation_id VARCHAR(64) NOT NULL,
    op_id VARCHAR(64) NOT NULL,
    app_user_id INTEGER NOT NULL references app_user(id),
    bolt11 VARCHAR(2048) NOT NULL,
    amount BIGINT NOT NULL,
    state INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE zaps
(
    id       INTEGER NOT NULL PRIMARY KEY references invoice (id),
    request  TEXT    NOT NULL,
    event_id VARCHAR(64)
);
