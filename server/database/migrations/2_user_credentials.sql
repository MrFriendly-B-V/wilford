CREATE TABLE user_credentials (
    user_id VARCHAR(64) NOT NULL,
    password_hash VARCHAR(72) NOT NULL,
    change_required BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (user_id)
);