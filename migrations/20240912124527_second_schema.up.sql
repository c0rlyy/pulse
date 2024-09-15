-- Add up migration script here
DO $$ BEGIN
    CREATE TYPE user_role AS ENUM ('USER', 'ADMIN');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Step 2: Create the users table with user_role enum
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(32) UNIQUE NOT NULL,
    email VARCHAR(64) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    role user_role DEFAULT 'USER',  -- Changed from is_admin BOOLEAN to role enum
    is_active BOOLEAN DEFAULT TRUE,
    is_private BOOLEAN DEFAULT FALSE
);


CREATE TABLE IF NOT EXISTS channels (
    id BIGSERIAL PRIMARY KEY,
    owner_id BIGINT NOT NULL,  -- Reference to the owner (probably a user)
    channel_name VARCHAR(32) UNIQUE NOT NULL,  -- Name of the channel
    CONSTRAINT fk_owner FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_channels (
    user_id BIGINT NOT NULL,  
    channel_id BIGINT NOT NULL,  -- Changed BIGSERIAL to BIGINT for consistency
    CONSTRAINT pk_user_channel PRIMARY KEY (user_id, channel_id),
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_channel FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS channel_messages (
    id BIGSERIAL PRIMARY KEY,
    channel_id BIGINT NOT NULL,  
    contents TEXT NOT NULL,  
    sender_id BIGINT NOT NULL,  
    sent_at TIMESTAMPTZ DEFAULT NOW(),  -- Timestamp of when the message was sent
    CONSTRAINT fk_channel FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE CASCADE,
    CONSTRAINT fk_sender FOREIGN KEY (sender_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_messages (
    id BIGSERIAL PRIMARY KEY,
    from_user_id BIGINT NOT NULL,  -- Reference to the sender (a user)
    to_user_id BIGINT NOT NULL,  -- Reference to the recipient (a user)
    contents TEXT NOT NULL,  -- Message content
    sent_at TIMESTAMPTZ DEFAULT NOW(),  -- Timestamp of when the message was sent
    CONSTRAINT fk_from_user FOREIGN KEY (from_user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_to_user FOREIGN KEY (to_user_id) REFERENCES users(id) ON DELETE CASCADE
);
