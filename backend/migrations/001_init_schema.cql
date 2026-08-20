-- Create keyspace
CREATE KEYSPACE IF NOT EXISTS liecord
WITH replication = {
    'class': 'NetworkTopologyStrategy',
    'datacenter1': 3
};

USE liecord;

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    username TEXT,
    email TEXT,
    password_hash TEXT,
    display_name TEXT,
    avatar TEXT,
    banner TEXT,
    about TEXT,
    status TEXT,
    custom_status TEXT,
    badges SET<TEXT>,
    nitro_active BOOLEAN,
    nitro_tier TEXT,
    nitro_start_date TIMESTAMP,
    nitro_expiry_date TIMESTAMP,
    nitro_subscription_months INT,
    created_at TIMESTAMP,
    last_seen TIMESTAMP
);

CREATE INDEX IF NOT EXISTS ON users (username);
CREATE INDEX IF NOT EXISTS ON users (email);

-- Servers table
CREATE TABLE IF NOT EXISTS servers (
    id UUID PRIMARY KEY,
    name TEXT,
    icon TEXT,
    banner TEXT,
    description TEXT,
    owner_id UUID,
    verification_level TEXT,
    default_notifications TEXT,
    boost_level INT,
    boost_count INT,
    created_at TIMESTAMP
);

CREATE INDEX IF NOT EXISTS ON servers (owner_id);

-- Server members table
CREATE TABLE IF NOT EXISTS server_members (
    server_id UUID,
    user_id UUID,
    nickname TEXT,
    roles SET<UUID>,
    joined_at TIMESTAMP,
    PRIMARY KEY (server_id, user_id)
);

CREATE INDEX IF NOT EXISTS ON server_members (user_id);

-- User servers (for quick lookup)
CREATE TABLE IF NOT EXISTS user_servers (
    user_id UUID,
    server_id UUID,
    joined_at TIMESTAMP,
    PRIMARY KEY (user_id, server_id)
);

-- Channels table
CREATE TABLE IF NOT EXISTS channels (
    id UUID PRIMARY KEY,
    name TEXT,
    channel_type TEXT,
    topic TEXT,
    server_id UUID,
    position INT,
    nsfw BOOLEAN,
    rate_limit_per_user INT,
    category_id UUID,
    last_message_at TIMESTAMP,
    created_at TIMESTAMP
);

CREATE INDEX IF NOT EXISTS ON channels (server_id);

-- Messages table (partitioned by channel for scalability)
CREATE TABLE IF NOT EXISTS messages (
    channel_id UUID,
    id UUID,
    content TEXT,
    author_id UUID,
    attachments LIST<FROZEN<MAP<TEXT, TEXT>>>,
    mentions SET<UUID>,
    pinned BOOLEAN,
    edited BOOLEAN,
    edited_at TIMESTAMP,
    reply_to UUID,
    created_at TIMESTAMP,
    PRIMARY KEY (channel_id, created_at, id)
) WITH CLUSTERING ORDER BY (created_at DESC, id DESC)
AND default_time_to_live = 0
AND compaction = {
    'class': 'TimeWindowCompactionStrategy',
    'compaction_window_size': 1,
    'compaction_window_unit': 'DAYS'
};

CREATE INDEX IF NOT EXISTS ON messages (author_id);

-- DM messages (partitioned by DM conversation)
CREATE TABLE IF NOT EXISTS dm_messages (
    dm_id UUID,
    id UUID,
    content TEXT,
    author_id UUID,
    attachments LIST<FROZEN<MAP<TEXT, TEXT>>>,
    edited BOOLEAN,
    edited_at TIMESTAMP,
    reply_to UUID,
    created_at TIMESTAMP,
    PRIMARY KEY (dm_id, created_at, id)
) WITH CLUSTERING ORDER BY (created_at DESC, id DESC);

-- Direct messages table
CREATE TABLE IF NOT EXISTS direct_messages (
    id UUID PRIMARY KEY,
    participant_1 UUID,
    participant_2 UUID,
    last_message_id UUID,
    last_message_at TIMESTAMP,
    created_at TIMESTAMP
);

-- User DMs lookup
CREATE TABLE IF NOT EXISTS user_dms (
    user_id UUID,
    dm_id UUID,
    other_user_id UUID,
    last_message_at TIMESTAMP,
    PRIMARY KEY (user_id, last_message_at, dm_id)
) WITH CLUSTERING ORDER BY (last_message_at DESC);

-- Roles table
CREATE TABLE IF NOT EXISTS roles (
    id UUID PRIMARY KEY,
    server_id UUID,
    name TEXT,
    color TEXT,
    position INT,
    permissions MAP<TEXT, BOOLEAN>,
    hoist BOOLEAN,
    mentionable BOOLEAN,
    is_default BOOLEAN,
    created_at TIMESTAMP
);

CREATE INDEX IF NOT EXISTS ON roles (server_id);

-- Message reactions (separate table for scalability)
CREATE TABLE IF NOT EXISTS reactions (
    message_id UUID,
    emoji TEXT,
    user_id UUID,
    created_at TIMESTAMP,
    PRIMARY KEY (message_id, emoji, user_id)
);

-- Friends table
CREATE TABLE IF NOT EXISTS friends (
    user_id UUID,
    friend_id UUID,
    created_at TIMESTAMP,
    PRIMARY KEY (user_id, friend_id)
);

-- Friend requests table
CREATE TABLE IF NOT EXISTS friend_requests (
    id UUID PRIMARY KEY,
    from_user UUID,
    to_user UUID,
    status TEXT,
    created_at TIMESTAMP
);

CREATE INDEX IF NOT EXISTS ON friend_requests (to_user);
CREATE INDEX IF NOT EXISTS ON friend_requests (from_user);

-- Blocked users table
CREATE TABLE IF NOT EXISTS blocked_users (
    user_id UUID,
    blocked_user_id UUID,
    created_at TIMESTAMP,
    PRIMARY KEY (user_id, blocked_user_id)
);

-- Server invites table
CREATE TABLE IF NOT EXISTS invites (
    code TEXT PRIMARY KEY,
    server_id UUID,
    creator_id UUID,
    uses INT,
    max_uses INT,
    expires_at TIMESTAMP,
    created_at TIMESTAMP
);

CREATE INDEX IF NOT EXISTS ON invites (server_id);

-- Server boosts table
CREATE TABLE IF NOT EXISTS server_boosts (
    server_id UUID,
    user_id UUID,
    boosted_at TIMESTAMP,
    PRIMARY KEY (server_id, user_id)
);

-- Voice states table (who's in which voice channel)
CREATE TABLE IF NOT EXISTS voice_states (
    user_id UUID PRIMARY KEY,
    server_id UUID,
    channel_id UUID,
    joined_at TIMESTAMP,
    muted BOOLEAN,
    deafened BOOLEAN,
    self_muted BOOLEAN,
    self_deafened BOOLEAN
);

CREATE INDEX IF NOT EXISTS ON voice_states (channel_id);

-- Typing indicators (short TTL for ephemeral data)
CREATE TABLE IF NOT EXISTS typing_indicators (
    channel_id UUID,
    user_id UUID,
    timestamp TIMESTAMP,
    PRIMARY KEY (channel_id, user_id)
) WITH default_time_to_live = 10;
