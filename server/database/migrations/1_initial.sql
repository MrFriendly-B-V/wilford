CREATE TABLE oauth2_clients (
    name VARCHAR(64) NOT NULL,
    redirect_uri TEXT NOT NULL,
    client_id VARCHAR(32) NOT NULL,
    client_secret VARCHAR(48) NOT NULL,
    is_internal BOOL NOT NULL,
    PRIMARY KEY (client_id)
);

CREATE TABLE oauth2_pending_authorizations (
    id VARCHAR(16) NOT NULL,
    client_id VARCHAR(32) NOT NULL,
    scopes TEXT DEFAULT NULL,
    state TEXT DEFAULT NULL,
    user_id TEXT DEFAULT NULL,
    ty TEXT NOT NULL,
    nonce TEXT DEFAULT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE oauth2_access_tokens (
    token VARCHAR(32) NOT NULL,
    client_id VARCHAR(32) NOT NULL,
    expires_at BIGINT NOT NULL,
    issued_at BIGINT NOT NULL,
    user_id VARCHAR(64) NOT NULL,
    scopes TEXT DEFAULT NULL,
    PRIMARY KEY (token)
);

CREATE TABLE oauth2_refresh_tokens (
    token VARCHAR(32) NOT NULL,
    client_id VARCHAR(32) NOT NULL,
    user_id VARCHAR(64) NOT NULL,
    scopes TEXT DEFAULT NULL,
    PRIMARY KEY (token)
);

CREATE TABLE oauth2_authorization_codes (
    client_id VARCHAR(32) NOT NULL,
    code VARCHAR(32) NOT NULL,
    expires_at BIGINT NOT NULL,
    scopes TEXT DEFAULT NULL,
    user_id TEXT NOT NULL,
    nonce TEXT DEFAULT NULL,
    PRIMARY KEY (code)
);

CREATE TABLE users (
    user_id VARCHAR(64) NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    is_admin BOOL,
    PRIMARY KEY (user_id)
);

CREATE TABLE user_permitted_scopes (
    user_id VARCHAR(64) NOT NULL,
    scope VARCHAR(64) NOT NULL,
    PRIMARY KEY (user_id, scope)
);

CREATE TABLE constant_access_tokens (
    name VARCHAR(64),
    token VARCHAR(32),
    PRIMARY KEY (token),
    UNIQUE (name)
)