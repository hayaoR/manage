-- Add migration script here
CREATE TABLE employees (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    firstName text NOT NULL,
    lastName text NOT NULL,
    email text NOT NULL UNIQUE,
    isAdmin boolean NOT NULL,
    registered_at timestamptz NOT NULL
);
