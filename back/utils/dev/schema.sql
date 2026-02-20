-- Schema DB for PostgreSQL (Development)
-- All fields are NOT NULL by default

-- Extension for UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Topics catalogue table
-- Using SERIAL for bitmask approach: ID position maps to bit position
-- Topic with ID=n has bitmask value of 2^(n-1) or (1 << (n-1))
-- Example: ID=1 → 2^0=1, ID=2 → 2^1=2, ID=3 → 2^2=4, etc.
DROP TABLE IF EXISTS topics_catalogue CASCADE;
CREATE TABLE topics_catalogue (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    active BOOLEAN DEFAULT true
);

-- Seed: Topics catalogue (13 temas de interés comunes en México)
-- IDs will be: 1-13, mapping to bitmask values: 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096
INSERT INTO topics_catalogue (name) VALUES
    ('Fútbol'),          -- ID=1,  bitmask=1
    ('Música'),          -- ID=2,  bitmask=2
    ('Gastronomía'),     -- ID=3,  bitmask=4
    ('Familia'),         -- ID=4,  bitmask=8
    ('Tecnología'),      -- ID=5,  bitmask=16
    ('Cine y TV'),       -- ID=6,  bitmask=32
    ('Viajes'),          -- ID=7,  bitmask=64
    ('Salud y Bienestar'), -- ID=8,  bitmask=128
    ('Moda'),            -- ID=9,  bitmask=256
    ('Negocios'),        -- ID=10, bitmask=512
    ('Política'),        -- ID=11, bitmask=1024
    ('Espiritualidad'),  -- ID=12, bitmask=2048
    ('Educación');       -- ID=13, bitmask=4096

-- Users table
-- topics field uses bitmask approach: each bit represents a topic from topics_catalogue
-- Example: topics=22 (binary: 10110) means user has topics with IDs 2, 3, and 5
-- To add topic: topics = topics | (1 << (topic_id - 1))
-- To check topic: (topics & (1 << (topic_id - 1))) > 0
-- To find common topics: user1.topics & user2.topics
-- To count common topics: BIT_COUNT(user1.topics & user2.topics)
DROP TABLE IF EXISTS users CASCADE;
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    phone VARCHAR(13) NOT NULL,
    address JSONB,
    username VARCHAR(40) NULL,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    topics INTEGER DEFAULT 0,
    blocked_users JSONB DEFAULT '[]'::jsonb,
    last_login TIMESTAMPTZ NULL
);

CREATE INDEX idx_users_phone ON users(phone);

-- Validation codes table (simulates cache)
DROP TABLE IF EXISTS validation_codes CASCADE;
CREATE TABLE validation_codes (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    code VARCHAR(5) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_validation_codes_code ON validation_codes(code);
CREATE INDEX idx_validation_codes_expires_at ON validation_codes(expires_at);

-- Matching queue table
-- Users who have requested a match but haven't been paired yet
DROP TABLE IF EXISTS matching_queue CASCADE;
CREATE TABLE matching_queue (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    queued_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_matching_queue_queued_at ON matching_queue(queued_at);

-- Active pairings table (1:1 matches)
-- Constraint: user_a_id < user_b_id prevents duplicate pairs
DROP TABLE IF EXISTS pairings CASCADE;
CREATE TABLE pairings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_a_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    user_b_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    affinity_score INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_a_id),
    UNIQUE(user_b_id),
    CHECK (user_a_id < user_b_id)
);

CREATE INDEX idx_pairings_user_a ON pairings(user_a_id);
CREATE INDEX idx_pairings_user_b ON pairings(user_b_id);
