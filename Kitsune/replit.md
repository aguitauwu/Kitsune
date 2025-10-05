# Kitsune (狐) - Discord Security Bot

An enterprise-grade Discord security bot built in Rust with comprehensive threat detection and automated moderation.

## Project Overview

Kitsune is a sophisticated Discord bot that protects servers through multiple layers of security:
- Real-time raid detection with sliding time windows
- Behavioral pattern analysis for spam and abuse detection
- Honeypot traps to catch automated bots
- Automated graduated response system
- Comprehensive forensic logging

## Architecture

The project is organized into focused modules:
- `src/bot/` - Discord integration with Serenity framework
- `src/security/` - Core security systems (raid detection, behavior analysis, honeypots)
- `src/database/` - PostgreSQL models and queries
- `src/config/` - Configuration management
- `src/utils/` - Utility functions (Redis, etc.)

## Setup

1. **Environment Variables** - Create a `.env` file:
   ```
   DISCORD_TOKEN=your_bot_token_here
   DATABASE_URL=<already_configured>
   REDIS_URL=redis://127.0.0.1:6379
   ```

2. **Configuration** - Optionally create `config.toml` from `config.toml.example`

3. **Run** - The bot will automatically start via the configured workflow

## Database Schema

The bot uses PostgreSQL with tables for:
- `guilds` - Server configurations
- `users` - Global user records and reputation
- `behavior_profiles` - Per-guild user behavioral metrics
- `incidents` - Security incidents and actions taken
- `forensic_events` - Detailed event logging
- `honeypot_catches` - Trap activations
- `whitelisted_users` - Trusted users exempt from auto-mod

## Security Systems

### Raid Detection
Monitors join patterns across multiple time windows (5s, 30s, 1m, 5m) and analyzes:
- Account age distribution
- Username similarity
- Avatar duplication

### Behavioral Analysis  
Tracks message patterns including:
- Spam score (message similarity)
- Link density
- Mention ratio
- Caps usage
- Message bursts

### Honeypot Traps
- Hidden channels invisible to normal users
- Fake verification commands
- Timing analysis for bot detection

### Auto-Moderation
Graduated responses based on threat level:
- Low (0.3-0.6): Monitor
- Medium (0.6-0.8): Timeout
- High (0.8-0.95): Kick
- Critical (>0.95): Ban + possible lockdown

## Admin Commands (113 Total)

### 🛡️ Core Security
- `/kitsune status` - View current security status
- `/kitsune scan` - Scan entire server for threats
- `/kitsune check <user>` - Analyze specific user
- `/kitsune analyze <hours>` - Analyze activity in time range
- `/kitsune reputation <user>` - Check global reputation

### 🚨 Moderation
- `/kitsune ban <user> [reason] [days]` - Ban with logging
- `/kitsune kick <user> [reason]` - Kick user
- `/kitsune timeout <user> <minutes> [reason]` - Timeout user
- `/kitsune warn <user> <reason>` - Warn user
- `/kitsune unban <user_id> [reason]` - Unban user
- `/kitsune pardon <user>` - Clear warnings

### ⚙️ Configuration
- `/kitsune config view` - View current configuration
- `/kitsune config automod <enable>` - Toggle auto-moderation
- `/kitsune channel alerts <#channel>` - Set alert channel
- `/kitsune channel logs <#channel>` - Set log channel
- `/kitsune channel reports <#channel>` - Set report channel
- `/kitsune notify add/remove <role>` - Manage notification roles

### 🕸️ Honeypot
- `/kitsune honeypot setup` - Configure honeypot traps
- `/kitsune honeypot list` - Show active traps
- `/kitsune honeypot catches [hours]` - View bot catches

### 📊 Statistics & Reports
- `/kitsune stats` - General security statistics
- `/kitsune stats server/user` - Detailed statistics
- `/kitsune leaderboard threat/activity` - Top users
- `/kitsune report generate <hours>` - Generate security report
- `/kitsune forensics incident/user/search` - Forensic analysis
- `/kitsune export data <format>` - Export data

### 🌐 Reputation Network
- `/kitsune reputation_query <user_id>` - Query global reputation
- `/kitsune reputation_report <user> <type>` - Report to network
- `/kitsune reputation_trust <server> <weight>` - Manage trust
- `/kitsune reputation_sync` - Sync with network

### 🔒 Lockdown
- `/kitsune lockdown <enable> [duration]` - Activate lockdown
- `/kitsune lockdown_status` - Check lockdown status
- `/kitsune verification <level>` - Set verification level

### 👥 Whitelist/Blacklist
- `/kitsune whitelist <user> [reason]` - Add to whitelist
- `/kitsune whitelist_remove <user>` - Remove from whitelist
- `/kitsune blacklist <user_id>` - Add to blacklist
- `/kitsune blacklist_import <server>` - Import blacklist

### 🧪 Testing & Debug
- `/kitsune test raid/ml` - Test systems
- `/kitsune debug` - Debug information
- `/kitsune health` - Health status

### 📚 Help & Info
- `/kitsune help [command]` - Get help
- `/kitsune about` - About Kitsune
- `/kitsune docs` - Documentation link
- `/kitsune invite` - Invite bot

### 📈 Advanced Analytics
- `/kitsune analytics messages/joins/threats/ml` - Analytics
- `/kitsune predict <user>` - Predict behavior
- `/kitsune compare <user1> <user2>` - Compare users

### 💾 Backup & Custom
- `/kitsune backup create/restore/list` - Backup management
- `/kitsune custom response/message/role` - Customization
- `/kitsune webhook add/list/test` - Webhook integration

## Technology Stack

- **Language**: Rust 2021 Edition
- **Discord**: Serenity + Poise
- **Database**: PostgreSQL via SQLx
- **Caching**: Redis
- **Async**: Tokio
- **Logging**: Tracing
