-- Add migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS CITEXT;
CREATE TABLE
    IF NOT EXISTS users (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    username VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL CHECK( octet_length(password) <> 0 ),
    fullname TEXT NULL,
    email VARCHAR(100) NOT NULL,
    phone_number VARCHAR(100) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE NULL
    );

CREATE TABLE
    IF NOT EXISTS notes (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        title VARCHAR(255) NOT NULL UNIQUE,
        content TEXT NOT NULL,
        category VARCHAR(100),
        published BOOLEAN DEFAULT FALSE,
        created_by UUID NULL,
        updated_by UUID NULL,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        deleted_at TIMESTAMP WITH TIME ZONE NULL
    );