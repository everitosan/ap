-- Schema DB for PostgreSQL (Development)
-- All fields are NOT NULL by default

-- Extension for UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Topics catalogue table
DROP TABLE IF EXISTS topics_catalogue CASCADE;
CREATE TABLE topics_catalogue (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(50) NOT NULL
);

-- Seed: Topics catalogue (13 temas de interés comunes en México)
INSERT INTO topics_catalogue (name) VALUES
    ('Fútbol'),
    ('Música'),
    ('Gastronomía'),
    ('Familia'),
    ('Tecnología'),
    ('Cine y TV'),
    ('Viajes'),
    ('Salud y Bienestar'),
    ('Moda'),
    ('Negocios'),
    ('Política'),
    ('Espiritualidad'),
    ('Educación');

-- Users table
DROP TABLE IF EXISTS users CASCADE;
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    phone VARCHAR(13) NOT NULL,
    address JSONB,
    username VARCHAR(40) NULL,
    created TIMESTAMP NOT NULL DEFAULT NOW(),
    topics JSONB NULL,
    last_login TIMESTAMP NULL
);

CREATE INDEX idx_users_phone ON users(phone);

-- Validation codes table (simulates cache)
DROP TABLE IF EXISTS validation_codes CASCADE;
CREATE TABLE validation_codes (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    code VARCHAR(5) NOT NULL,
    expires_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_validation_codes_code ON validation_codes(code);
CREATE INDEX idx_validation_codes_expires_at ON validation_codes(expires_at);


