-- Kitsune Security Bot - Initial Database Schema

-- Guilds configuration table
CREATE TABLE IF NOT EXISTS guilds (
    guild_id BIGINT PRIMARY KEY,
    name TEXT NOT NULL,
    owner_id BIGINT NOT NULL,
    config JSONB NOT NULL DEFAULT '{}'::jsonb,
    raid_threshold_5s INTEGER NOT NULL DEFAULT 5,
    raid_threshold_30s INTEGER NOT NULL DEFAULT 10,
    raid_threshold_1m INTEGER NOT NULL DEFAULT 15,
    raid_threshold_5m INTEGER NOT NULL DEFAULT 30,
    new_account_days INTEGER NOT NULL DEFAULT 7,
    auto_mod_enabled BOOLEAN NOT NULL DEFAULT true,
    lockdown_active BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Global user records and reputation
CREATE TABLE IF NOT EXISTS users (
    user_id BIGINT PRIMARY KEY,
    username TEXT NOT NULL,
    discriminator TEXT,
    global_reputation INTEGER NOT NULL DEFAULT 0,
    total_incidents INTEGER NOT NULL DEFAULT 0,
    first_seen TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb
);

-- Per-guild behavior profiles
CREATE TABLE IF NOT EXISTS behavior_profiles (
    id SERIAL PRIMARY KEY,
    guild_id BIGINT NOT NULL REFERENCES guilds(guild_id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    message_count INTEGER NOT NULL DEFAULT 0,
    join_timestamp TIMESTAMPTZ,
    last_message_time TIMESTAMPTZ,
    spam_score REAL NOT NULL DEFAULT 0.0,
    link_density REAL NOT NULL DEFAULT 0.0,
    mention_ratio REAL NOT NULL DEFAULT 0.0,
    caps_ratio REAL NOT NULL DEFAULT 0.0,
    emoji_density REAL NOT NULL DEFAULT 0.0,
    channel_diversity REAL NOT NULL DEFAULT 0.0,
    reply_ratio REAL NOT NULL DEFAULT 0.0,
    unique_interactions INTEGER NOT NULL DEFAULT 0,
    features JSONB NOT NULL DEFAULT '{}'::jsonb,
    threat_score REAL NOT NULL DEFAULT 0.0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(guild_id, user_id)
);

-- Security incidents
CREATE TABLE IF NOT EXISTS incidents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    guild_id BIGINT NOT NULL REFERENCES guilds(guild_id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    incident_type TEXT NOT NULL,
    severity TEXT NOT NULL,
    threat_score REAL NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}'::jsonb,
    action_taken TEXT,
    moderator_id BIGINT,
    resolved BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Forensic event log
CREATE TABLE IF NOT EXISTS forensic_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    guild_id BIGINT NOT NULL REFERENCES guilds(guild_id) ON DELETE CASCADE,
    user_id BIGINT,
    event_type TEXT NOT NULL,
    content TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    threat_score REAL NOT NULL DEFAULT 0.0,
    related_events UUID[],
    tags TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Honeypot catches
CREATE TABLE IF NOT EXISTS honeypot_catches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    guild_id BIGINT NOT NULL REFERENCES guilds(guild_id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    trap_type TEXT NOT NULL,
    trap_name TEXT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Whitelisted users (bypass auto-mod)
CREATE TABLE IF NOT EXISTS whitelisted_users (
    id SERIAL PRIMARY KEY,
    guild_id BIGINT NOT NULL REFERENCES guilds(guild_id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    reason TEXT,
    added_by BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(guild_id, user_id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_behavior_profiles_guild_user ON behavior_profiles(guild_id, user_id);
CREATE INDEX IF NOT EXISTS idx_behavior_profiles_threat ON behavior_profiles(threat_score DESC) WHERE threat_score > 0.5;
CREATE INDEX IF NOT EXISTS idx_incidents_guild_time ON incidents(guild_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_incidents_user ON incidents(user_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_incidents_unresolved ON incidents(guild_id) WHERE resolved = false;
CREATE INDEX IF NOT EXISTS idx_forensic_events_guild_time ON forensic_events(guild_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_forensic_events_user ON forensic_events(user_id, created_at DESC) WHERE user_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_forensic_events_tags ON forensic_events USING GIN(tags);
CREATE INDEX IF NOT EXISTS idx_honeypot_catches_guild ON honeypot_catches(guild_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_honeypot_catches_user ON honeypot_catches(user_id);
