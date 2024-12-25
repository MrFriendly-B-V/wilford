CREATE TABLE user_credentials (
    user_id VARCHAR(64) NOT NULL,
    password_hash VARCHAR(72) NOT NULL,
    PRIMARY KEY (user_id)
);