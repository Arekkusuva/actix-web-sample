CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(254) NOT NULL,
    password VARCHAR(120) NOT NULL,
    -- Constraints
    CONSTRAINT users_email_unq_key UNIQUE(email)
);