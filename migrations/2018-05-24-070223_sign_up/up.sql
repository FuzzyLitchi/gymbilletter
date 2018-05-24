CREATE TABLE people (
    id SERIAL PRIMARY KEY,
    party INT REFERENCES parties(id) NOT NULL,
    first_name text NOT NULL,
    last_name text NOT NULL,
    email text NOT NULL,
    phone_number text
);
