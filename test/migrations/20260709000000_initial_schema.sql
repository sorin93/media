------------------------------------------------------------------------------------------------------------------------

-- CREATE DATABASE media;
------------------------------------------------------------------------------------------------------------------------

SET TIME ZONE 'UTC';
CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE EXTENSION IF NOT EXISTS vector;

------------------------------------------------------------------------------------------------------------------------

CREATE TYPE user_status AS ENUM ('active', 'inactive', 'deleted');

CREATE TABLE "user" (
    user_id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    media_id              UUID,
    status                user_status NOT NULL DEFAULT 'active',
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    name                  VARCHAR(50) NOT NULL CHECK (name <> ''),
    email                 VARCHAR(50) NOT NULL UNIQUE CHECK (email <> ''),
    password_hash         VARCHAR(255) NOT NULL,
    phone                 VARCHAR(15) CHECK (phone <> ''),
    country               CHAR(2) NOT NULL CHECK (country <> ''),
    birth_date            DATE NOT NULL CHECK (birth_date <= '2015-01-01'),
    is_mfa                BOOL DEFAULT FALSE,
    following_count       SMALLINT NOT NULL DEFAULT 0 CHECK (following_count BETWEEN 0 AND 1000),
    followed_count        INT NOT NULL DEFAULT 0 CHECK (followed_count BETWEEN 0 AND 1000000),
    media_count           INT NOT NULL DEFAULT 0 CHECK (media_count BETWEEN 0 AND 1000000),
    pending_comments      BOOL DEFAULT FALSE
);

------------------------------------------------------------------------------------------------------------------------

CREATE TABLE follow (
    from_user_id          UUID NOT NULL,
    to_user_id            UUID NOT NULL CHECK (from_user_id <> to_user_id),
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    FOREIGN KEY (from_user_id) REFERENCES "user" (user_id) ON DELETE CASCADE,
    FOREIGN KEY (to_user_id) REFERENCES "user" (user_id) ON DELETE CASCADE,
    PRIMARY KEY (from_user_id, to_user_id)
);

------------------------------------------------------------------------------------------------------------------------

CREATE TYPE media_status AS ENUM ('created', 'active', 'inactive', 'deleted');
CREATE TYPE media_type AS ENUM ('image', 'video');

CREATE TABLE media (
    user_id               UUID NOT NULL,
    parent_media_id       UUID CHECK (parent_media_id <> media_id),
    media_id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    status                media_status NOT NULL DEFAULT 'created',
    type                  media_type NOT NULL,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    caption               VARCHAR(100) CHECK (caption <> ''),
    reply_count           SMALLINT NOT NULL DEFAULT 0 CHECK (reply_count BETWEEN 0 AND 10000),
    comment_count         SMALLINT NOT NULL DEFAULT 0 CHECK (comment_count BETWEEN 0 AND 10000),
    like_count            INT NOT NULL DEFAULT 0 CHECK (like_count BETWEEN 0 AND 1000000),
    embedding             VECTOR(3072),
    blurhash              VARCHAR(100) CHECK (blurhash <> ''),
    metadata              JSONB, -- { type, width, height }
    FOREIGN KEY (user_id) REFERENCES "user" (user_id) ON DELETE RESTRICT,
    FOREIGN KEY (parent_media_id) REFERENCES media (media_id) ON DELETE SET NULL
);

-- Index 1: User media
CREATE INDEX idx_media_user ON media(user_id, created_at DESC) WHERE status = 'active';

-- Index 2: Media replies
CREATE INDEX idx_media_parent ON media(parent_media_id, created_at DESC) WHERE status = 'active';

-- Index 3: Media that needs to be discarded (created, inactive, deleted)
CREATE INDEX idx_media_delete ON media (updated_at ASC) WHERE status <> 'active';

-- Index 4: vector similarity search, for <=> (<=> best, <-> slightly faster, <#> fastest)
CREATE INDEX idx_media_embedding
ON media
USING hnsw ( (subvector(embedding, 1, 768)::vector(768)) vector_cosine_ops )
WITH (m = 16, ef_construction = 100)
WHERE status = 'active';

------------------------------------------------------------------------------------------------------------------------

CREATE TABLE "like" (
    user_id               UUID NOT NULL,
    media_id              UUID NOT NULL,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    FOREIGN KEY (user_id) REFERENCES "user" (user_id) ON DELETE CASCADE,
    FOREIGN KEY (media_id) REFERENCES media (media_id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, media_id)
);

-- Index 1: Media likes
CREATE INDEX idx_like_media ON "like"(media_id, created_at DESC);

------------------------------------------------------------------------------------------------------------------------

CREATE TYPE text_status AS ENUM ('active', 'inactive', 'deleted');

CREATE TABLE "text" (
    text_id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    status                text_status NOT NULL DEFAULT 'active',
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    "text"                VARCHAR(100) UNIQUE CHECK ("text" <> ''),
    search_count          INT NOT NULL DEFAULT 1 CHECK (search_count >= 1),
    embedding             VECTOR(3072) NOT NULL
);

CREATE INDEX idx_text_embedding
ON "text"
USING hnsw ( (subvector(embedding, 1, 768)::vector(768)) vector_cosine_ops )
WITH (m = 16, ef_construction = 100)
WHERE status = 'active';

------------------------------------------------------------------------------------------------------------------------

CREATE TYPE history_type AS ENUM ('image', 'video', 'text', 'similar', 'user');

CREATE TABLE history (
    user_id               UUID NOT NULL,
    target_id             UUID NOT NULL,
    type                  history_type NOT NULL,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    FOREIGN KEY (user_id) REFERENCES "user" (user_id) ON DELETE CASCADE
);

------------------------------------------------------------------------------------------------------------------------

CREATE TYPE comment_status AS ENUM ('active', 'inactive', 'deleted');

CREATE TABLE comment (
    from_user_id          UUID NOT NULL,
    to_user_id            UUID NOT NULL,
    media_id              UUID NOT NULL,
    comment_id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    status                comment_status NOT NULL DEFAULT 'active',
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    text                  VARCHAR(1000) NOT NULL CHECK (text <> ''),
    FOREIGN KEY (from_user_id) REFERENCES "user" (user_id) ON DELETE CASCADE,
    FOREIGN KEY (to_user_id) REFERENCES "user" (user_id) ON DELETE CASCADE,
    FOREIGN KEY (media_id) REFERENCES media (media_id) ON DELETE CASCADE
);

-- Index 1: User comments
CREATE INDEX idx_comment_user ON comment(to_user_id, created_at DESC) WHERE status = 'active';

-- Index 1: Media comments
CREATE INDEX idx_comment_media ON comment(media_id, created_at DESC) WHERE status = 'active';

------------------------------------------------------------------------------------------------------------------------

CREATE TYPE refresh_token_status AS ENUM ('active', 'used', 'revoked');

CREATE TABLE refresh_token (
    token_hash         VARCHAR(255) PRIMARY KEY CHECK (token_hash <> ''),
    user_id            UUID NOT NULL,
    session_id         UUID NOT NULL,
    status             refresh_token_status NOT NULL DEFAULT 'active',
    created_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at         TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (user_id) REFERENCES "user" (user_id) ON DELETE CASCADE
);

CREATE INDEX idx_refresh_token_user ON refresh_token (user_id);
CREATE INDEX idx_refresh_token_session ON refresh_token (session_id);

------------------------------------------------------------------------------------------------------------------------

CREATE TYPE verify_type AS ENUM ('register', 'login', 'email');

CREATE TABLE verify (
    user_id            UUID,
    type               verify_type NOT NULL,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at         TIMESTAMPTZ NOT NULL,
    email              VARCHAR(100) NOT NULL CHECK (email <> ''),
    code_hash          VARCHAR(255) NOT NULL CHECK (code_hash <> ''),
    password_hash      VARCHAR(100) NOT NULL CHECK (password_hash <> ''),
    attempts           SMALLINT NOT NULL DEFAULT 0 CHECK (attempts BETWEEN 0 AND 10),
    PRIMARY KEY (email, type)
);

CREATE INDEX idx_verify_user ON verify (user_id);

------------------------------------------------------------------------------------------------------------------------
