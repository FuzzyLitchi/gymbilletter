CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    user_id uuid DEFAULT uuid_generate_v4() PRIMARY KEY NOT NULL,
    username character varying(30) NOT NULL,
    is_admin boolean NOT NULL,
    salt_hash text NOT NULL
);
