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
    espo_user_id TEXT DEFAULT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE oauth2_access_tokens (
    token VARCHAR(32) NOT NULL,
    client_id VARCHAR(32) NOT NULL,
    expires_at BIGINT NOT NULL,
    espo_user_id VARCHAR(64) NOT NULL,
    PRIMARY KEY (token)
);

CREATE TABLE oauth2_authorization_codes (
    client_id VARCHAR(32) NOT NULL,
    code VARCHAR(32) NOT NULL,
    expires_at BIGINT NOT NULL,
    scopes TEXT DEFAULT NULL,
    PRIMARY KEY (code)
);