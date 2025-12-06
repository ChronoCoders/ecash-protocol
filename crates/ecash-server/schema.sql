-- Create database (run manually with superuser)
-- CREATE DATABASE ecash_db;
-- CREATE USER ecash_user WITH PASSWORD 'ecash_password_2024';
-- GRANT ALL PRIVILEGES ON DATABASE ecash_db TO ecash_user;

-- Connect to ecash_db and run:

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Tokens table for tracking spent serials
CREATE TABLE IF NOT EXISTS tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    serial_number BYTEA NOT NULL UNIQUE,
    serial_hex VARCHAR(64) NOT NULL UNIQUE,
    denomination BIGINT NOT NULL,
    currency VARCHAR(10) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'redeemed',
    redeemed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    merchant_id VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Transactions audit log
CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    transaction_type VARCHAR(20) NOT NULL,
    amount BIGINT NOT NULL,
    denomination BIGINT NOT NULL,
    token_count INTEGER NOT NULL,
    institution_id VARCHAR(255) NOT NULL,
    key_id VARCHAR(255) NOT NULL,
    status VARCHAR(20) NOT NULL,
    error_message TEXT,
    request_data JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Keys table for key rotation
CREATE TABLE IF NOT EXISTS signing_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    key_id VARCHAR(255) NOT NULL UNIQUE,
    institution_id VARCHAR(255) NOT NULL,
    public_key_pem TEXT NOT NULL,
    private_key_encrypted BYTEA,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ
);

-- Indexes
CREATE INDEX idx_tokens_serial_hex ON tokens(serial_hex);
CREATE INDEX idx_tokens_status ON tokens(status);
CREATE INDEX idx_tokens_redeemed_at ON tokens(redeemed_at);
CREATE INDEX idx_transactions_type ON transactions(transaction_type);
CREATE INDEX idx_transactions_created_at ON transactions(created_at);
CREATE INDEX idx_signing_keys_key_id ON signing_keys(key_id);
CREATE INDEX idx_signing_keys_status ON signing_keys(status);

-- Grant permissions
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO ecash_user;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO ecash_user;
