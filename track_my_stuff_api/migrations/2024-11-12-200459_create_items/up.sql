DROP TABLE IF EXISTS items;

CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR,
    expected_arrival_date TIMESTAMP NOT NULL,
    item_received BOOLEAN NOT NULL
);