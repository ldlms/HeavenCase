-- =========================================================
-- Types énumérés
-- =========================================================
CREATE TYPE user_role          AS ENUM ('user', 'admin');
CREATE TYPE item_rarity        AS ENUM ('common', 'rare', 'epic', 'legendary', 'mythic');
CREATE TYPE inventory_status   AS ENUM ('owned', 'sold', 'withdrawn', 'upgraded');
CREATE TYPE transaction_type   AS ENUM ('deposit', 'case_open', 'item_sell', 'withdrawal', 'bonus', 'refund');
CREATE TYPE payment_status     AS ENUM ('pending', 'completed', 'failed', 'refunded');
CREATE TYPE withdrawal_status  AS ENUM ('pending', 'processing', 'completed', 'failed', 'cancelled');

-- =========================================================
-- Utilisateurs
-- =========================================================
CREATE TABLE users (
                       id             UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
                       username       TEXT        NOT NULL,
                       email          TEXT        UNIQUE,
                       password_hash  TEXT,
                       steam_id       TEXT        UNIQUE,
                       balance        BIGINT      NOT NULL DEFAULT 0 CHECK (balance >= 0),
                       level          INT         NOT NULL DEFAULT 1,
                       xp             INT         NOT NULL DEFAULT 0,
                       client_seed    TEXT        NOT NULL DEFAULT replace(gen_random_uuid()::text, '-', ''),
                       role           user_role   NOT NULL DEFAULT 'user',
                       created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
                       banned_at      TIMESTAMPTZ,
    -- Un compte doit avoir au moins un moyen de connexion
                       CHECK (password_hash IS NOT NULL OR steam_id IS NOT NULL)
);

-- =========================================================
-- Catégories de caisses (référencée par cases.category_id)
-- =========================================================
CREATE TABLE case_categories (
                                 id    UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                                 name  TEXT NOT NULL,
                                 slug  TEXT NOT NULL UNIQUE
);

-- =========================================================
-- Catalogue d'objets
-- =========================================================
CREATE TABLE items (
                       id         UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
                       name       TEXT        NOT NULL,
                       image_url  TEXT,
                       rarity     item_rarity NOT NULL,
                       value      BIGINT      NOT NULL CHECK (value >= 0),
                       category   TEXT,
                       metadata   JSONB       NOT NULL DEFAULT '{}'
);

-- =========================================================
-- Caisses
-- =========================================================
CREATE TABLE cases (
                       id           UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
                       name         TEXT        NOT NULL,
                       slug         TEXT        NOT NULL UNIQUE,
                       image_url    TEXT,
                       price        BIGINT      NOT NULL CHECK (price >= 0),
                       category_id  UUID        REFERENCES case_categories (id),
                       is_active    BOOLEAN     NOT NULL DEFAULT true,
                       min_level    INT,
                       created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- =========================================================
-- Contenu d'une caisse + probabilités
-- =========================================================
CREATE TABLE case_items (
                            case_id      UUID NOT NULL REFERENCES cases (id) ON DELETE CASCADE,
                            item_id      UUID NOT NULL REFERENCES items (id),
                            drop_weight  INT  NOT NULL CHECK (drop_weight > 0),
                            PRIMARY KEY (case_id, item_id)
);

-- =========================================================
-- Ouvertures (table centrale)
-- =========================================================
CREATE TABLE case_openings (
                               id                UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
                               user_id           UUID        NOT NULL REFERENCES users (id),
                               case_id           UUID        NOT NULL REFERENCES cases (id),
                               item_id           UUID        NOT NULL REFERENCES items (id),
                               price_paid        BIGINT      NOT NULL,
                               item_value        BIGINT      NOT NULL,
                               server_seed_hash  TEXT        NOT NULL,
                               server_seed       TEXT,       -- révélée après rotation du seed
                               client_seed       TEXT        NOT NULL,
                               nonce             INT         NOT NULL,
                               roll_value        NUMERIC     NOT NULL,
                               created_at        TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_case_openings_user ON case_openings (user_id, created_at DESC);
CREATE INDEX idx_case_openings_case ON case_openings (case_id);

-- =========================================================
-- Inventaire
-- =========================================================
CREATE TABLE inventory_items (
                                 id                 UUID             PRIMARY KEY DEFAULT gen_random_uuid(),
                                 user_id            UUID             NOT NULL REFERENCES users (id),
                                 item_id            UUID             NOT NULL REFERENCES items (id),
                                 source_opening_id  UUID             REFERENCES case_openings (id),
                                 status             inventory_status NOT NULL DEFAULT 'owned',
                                 acquired_at        TIMESTAMPTZ      NOT NULL DEFAULT now()
);

CREATE INDEX idx_inventory_user_status ON inventory_items (user_id, status);

-- =========================================================
-- Ledger financier
-- =========================================================
CREATE TABLE transactions (
                              id             UUID             PRIMARY KEY DEFAULT gen_random_uuid(),
                              user_id        UUID             NOT NULL REFERENCES users (id),
                              type           transaction_type NOT NULL,
                              amount         BIGINT           NOT NULL,  -- signé (+/-)
                              balance_after  BIGINT           NOT NULL CHECK (balance_after >= 0),
                              reference_id   UUID,
                              created_at     TIMESTAMPTZ      NOT NULL DEFAULT now()
);

CREATE INDEX idx_transactions_user ON transactions (user_id, created_at DESC);

-- =========================================================
-- Paiements
-- =========================================================
CREATE TABLE payments (
                          id              UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
                          user_id         UUID           NOT NULL REFERENCES users (id),
                          provider        TEXT           NOT NULL,
                          provider_tx_id  TEXT           NOT NULL UNIQUE,
                          amount          BIGINT         NOT NULL CHECK (amount > 0),
                          status          payment_status NOT NULL DEFAULT 'pending',
                          created_at      TIMESTAMPTZ    NOT NULL DEFAULT now()
);

CREATE INDEX idx_payments_user ON payments (user_id);

-- =========================================================
-- Retraits d'objets
-- =========================================================
CREATE TABLE withdrawals (
                             id                 UUID              PRIMARY KEY DEFAULT gen_random_uuid(),
                             user_id            UUID              NOT NULL REFERENCES users (id),
                             inventory_item_id  UUID              NOT NULL REFERENCES inventory_items (id),
                             status             withdrawal_status NOT NULL DEFAULT 'pending',
                             tracking_info      JSONB,
                             created_at         TIMESTAMPTZ       NOT NULL DEFAULT now()
);

CREATE INDEX idx_withdrawals_user ON withdrawals (user_id);-- Add migration script here
