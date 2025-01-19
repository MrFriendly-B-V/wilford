CREATE TABLE user_emails (
    user_id VARCHAR(64) NOT NULL,
    registered_at BIGINT NOT NULL,
    address TEXT NOT NULL,
    verified BOOL DEFAULT FALSE,
    PRIMARY KEY (user_id, address)
);

CREATE TABLE user_email_verifications (
    user_id VARCHAR(64) NOT NULL,
    address TEXT NOT NULL,
    verification_code VARCHAR(32) NOT NULL,
    PRIMARY KEY (user_id, address, verification_code)
)