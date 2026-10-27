-- Add migration script here
-- Unicité de l'email insensible à la casse
ALTER TABLE users DROP CONSTRAINT users_email_key;
CREATE UNIQUE INDEX users_email_lower_key ON users (lower(email));

-- Date de validation (NULL = pas encore validé)
ALTER TABLE users ADD COLUMN email_verified_at TIMESTAMPTZ;

-- Jetons de validation envoyés par mail
CREATE TABLE email_verification_tokens (
                                           token_hash  TEXT        PRIMARY KEY,
                                           user_id     UUID        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
                                           expires_at  TIMESTAMPTZ NOT NULL,
                                           created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_email_verification_user ON email_verification_tokens (user_id);