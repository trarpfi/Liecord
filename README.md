# LieCord

LieCord is a Discord-inspired chat platform built around a Rust backend and a React client.

The project is focused on real-time communication, server/channel management, direct messages, presence, permissions, search, and voice/video features. The backend is designed around Axum, Tokio, ScyllaDB, Redis, and WebSocket-based events.

> Status: In development. APIs and internal structures may change.

## Features

### Messaging

- Real-time messaging over WebSocket
- Text channels and server management
- Direct messages
- Markdown, emoji reactions, and mentions
- Message editing and deletion
- File uploads
- Message search
- Typing indicators
- Presence states (online, offline, idle, DND)

### Servers

- Create and manage servers
- Text and voice channels
- Roles and permissions
- Server invites
- Friend requests and relationships

### Voice and video

- Voice channels
- WebRTC video calls
- Screen sharing
- Mute/deafen controls

### Nitro

LieCord currently models a Nitro-style subscription system with:

- Multiple subscription tiers
- Subscription-based badges
- Server boosting
- Custom profiles
- Larger upload limits
- HD streaming

These features are part of the project implementation and are not intended to imply any relationship with Discord.

## Architecture

The project is split into a client layer, an API/backend layer, and supporting data services.

```text
┌──────────────────────────────────────────────────────────────┐
│                        Client Layer                          │
│                                                              │
│    Web (React)       Desktop (Electron)      Mobile (future) │
└──────────────────────────────┬───────────────────────────────┘
                               │
                               ▼
┌──────────────────────────────────────────────────────────────┐
│                       API / Backend                          │
│                         Rust + Axum                          │
│                                                              │
│             REST API                 WebSocket               │
└───────────────┬───────────────────────────────┬──────────────┘
                │                               │
                ▼                               ▼
        ┌──────────────┐                ┌──────────────┐
        │   ScyllaDB   │                │    Redis     │
        │ users         │                │ sessions     │
        │ servers       │                │ presence     │
        │ channels      │                │ cache        │
        │ messages      │                │ pub/sub      │
        └──────────────┘                └──────────────┘
                │
                ▼
        ┌──────────────────┐
        │  Elasticsearch   │
        │  message search  │
        └──────────────────┘
```

## Stack

### Backend

- Rust 1.75+
- Axum
- Tokio
- ScyllaDB
- Redis
- Elasticsearch
- WebSocket

### Client

- React 18
- TypeScript
- Vite
- Tailwind CSS
- Zustand
- React Query
- Socket.IO client
- Electron

### Infrastructure

- Docker / Docker Compose
- Nginx (optional)

## Data model

ScyllaDB is used for the primary application data.

Current tables include:

- `users`
- `servers`
- `server_members`
- `channels`
- `messages`
- `dm_messages`
- `direct_messages`
- `roles`
- `reactions`
- `friends`
- `friend_requests`
- `invites`
- `voice_states`

Messages are partitioned by channel to keep reads and writes local to a channel partition.

```cql
CREATE TABLE messages (
    channel_id UUID,
    id UUID,
    created_at TIMESTAMP,
    ...
    PRIMARY KEY (channel_id, created_at, id)
) WITH CLUSTERING ORDER BY (created_at DESC, id DESC);
```

Redis is used for sessions, presence, caching, and pub/sub. Elasticsearch handles full-text message search.

## Requirements

- Rust 1.75+
- Node.js 18+
- Docker and Docker Compose

You can run the full stack with Docker, or start the services individually for development.

## Running with Docker

Clone the repository:

```bash
git clone https://github.com/yourusername/liecord.git
cd liecord
```

Create the backend environment file:

```bash
cp backend/.env.example backend/.env
```

Start the stack:

```bash
docker-compose up -d
```

Initialize the database:

```bash
docker exec -it liecord-scylla cqlsh -f /migrations/001_init_schema.cql
```

The default services are available at:

```text
Frontend       http://localhost:3000
Backend API    http://localhost:8080
ScyllaDB       localhost:9042
Redis          localhost:6379
Elasticsearch  http://localhost:9200
```

## Development

### ScyllaDB

Ubuntu/Debian:

```bash
sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 5E08FBD8B5D6EC9C
sudo curl -L --output /etc/apt/sources.list.d/scylla.list http://downloads.scylladb.com/deb/ubuntu/scylla-5.2.list
sudo apt-get update
sudo apt-get install scylla
sudo systemctl start scylla-server
```

macOS:

```bash
brew install scylladb/tap/scylla
```

### Redis

Ubuntu/Debian:

```bash
sudo apt-get install redis-server
```

macOS:

```bash
brew install redis
brew services start redis
```

### Elasticsearch

Ubuntu/Debian:

```bash
wget https://artifacts.elastic.co/downloads/elasticsearch/elasticsearch-8.11.0-amd64.deb
sudo dpkg -i elasticsearch-8.11.0-amd64.deb
sudo systemctl start elasticsearch
```

macOS:

```bash
brew install elasticsearch
brew services start elasticsearch
```

### Backend

```bash
cd backend

cp .env.example .env
cargo build --release

cqlsh -f migrations/001_init_schema.cql

cargo run --release
```

### Client

```bash
cd client

npm install
npm run dev
```

Production build:

```bash
npm run build
```

### Electron

```bash
cd client

npm run electron-dev
npm run electron-build
```

## Configuration

Backend configuration lives in `backend/.env`:

```env
HOST=0.0.0.0
PORT=8080
ENVIRONMENT=development

SCYLLA_NODES=127.0.0.1:9042
SCYLLA_KEYSPACE=liecord
SCYLLA_REPLICATION_FACTOR=3

REDIS_URL=redis://127.0.0.1:6379
REDIS_POOL_SIZE=10

ELASTICSEARCH_URL=http://localhost:9200
ELASTICSEARCH_INDEX_PREFIX=liecord

JWT_SECRET=your_super_secret_jwt_key_change_this_in_production
JWT_EXPIRATION=604800

UPLOAD_DIR=./uploads
MAX_UPLOAD_SIZE=52428800
ALLOWED_FILE_TYPES=image/jpeg,image/png,image/gif,image/webp,video/mp4,video/webm,application/pdf

CORS_ORIGIN=http://localhost:3000
```

Client configuration:

```env
VITE_API_URL=http://localhost:8080
VITE_WS_URL=ws://localhost:8080
```

Do not use the example JWT secret in a production deployment.

## API

The HTTP API is currently organized around authentication, servers, channels, messages, and Nitro.

### Authentication

```http
POST /api/auth/register
POST /api/auth/login
GET  /api/auth/me
```

Example login response:

```json
{
  "user": {},
  "token": "jwt_token_here"
}
```

### Servers

```http
POST   /api/servers
GET    /api/servers
GET    /api/servers/:id
PATCH  /api/servers/:id
DELETE /api/servers/:id
POST   /api/servers/:id/invite
POST   /api/servers/join/:code
```

### Channels

```http
POST   /api/channels
GET    /api/channels/:id
PATCH  /api/channels/:id
DELETE /api/channels/:id
GET    /api/channels/:id/messages
```

### Messages

```http
POST   /api/messages
PATCH  /api/messages/:id
DELETE /api/messages/:id
POST   /api/messages/:id/react
DELETE /api/messages/:id/react
POST   /api/messages/search
```

### Nitro

```http
GET  /api/nitro/info
POST /api/nitro/subscribe
POST /api/nitro/cancel
POST /api/nitro/renew
POST /api/nitro/boost/:server_id
```

## WebSocket

Client events currently include:

```javascript
socket.emit("authenticate", { token: "jwt_token" });

socket.emit("channel:join", {
  channel_id: "uuid"
});

socket.emit("message:send", {
  channel_id: "uuid",
  content: "Hello!"
});

socket.emit("typing:start", {
  channel_id: "uuid"
});

socket.emit("typing:stop", {
  channel_id: "uuid"
});

socket.emit("voice:join", {
  channel_id: "uuid"
});

socket.emit("voice:signal", {
  to: "user_id",
  signal: data
});
```

Server events include:

```javascript
socket.on("message:new", (data) => {});
socket.on("user:status", (data) => {});
socket.on("typing:start", (data) => {});
socket.on("voice:signal", (data) => {});
```

## Testing

Backend:

```bash
cd backend
cargo test
```

Frontend:

```bash
cd client
npm test
```

End-to-end:

```bash
npm run test:e2e
```

## Performance

The current storage and caching layout is intended for a write-heavy messaging workload.

- Messages are partitioned by `channel_id`
- Timestamps are used for clustering and time-range reads
- Time-window compaction is used for message workloads
- Redis handles hot session/presence data
- Redis pub/sub is used for event fan-out
- Elasticsearch handles search-heavy reads
- Backend instances can be scaled horizontally behind a reverse proxy

The current target workload is approximately 70% writes / 30% reads.

## Security

Implemented security controls include:

- JWT authentication
- Argon2 password hashing
- API rate limiting
- Input validation and sanitization
- CORS protection
- Parameterized database queries
- XSS protection
- File upload validation

Production deployments still require environment-specific hardening, monitoring, secret management, and infrastructure review.

## License

MIT. See `LICENSE` for details.
