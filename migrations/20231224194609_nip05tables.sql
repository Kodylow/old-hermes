-- Up
CREATE TABLE nip05 (
    id SERIAL PRIMARY KEY,
    pubkey VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL
);
CREATE TABLE relay (
    id SERIAL PRIMARY KEY,
    relay VARCHAR(255) NOT NULL
);
CREATE TABLE nip05relays (
    nip05_id INTEGER REFERENCES nip05(id),
    relay_id INTEGER REFERENCES relay(id),
    PRIMARY KEY (nip05_id, relay_id)
);
