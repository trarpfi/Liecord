# LieCord Architecture

## Overview

LieCord uses a service-oriented backend built around REST APIs, WebSockets, and separate storage systems for persistent data, caching, and search.

The current design is intended to support real-time messaging while keeping the main application components independently scalable.

## Technology Stack

### Rust Backend

The backend is written in Rust and uses Tokio for asynchronous I/O.

Rust is used for:

* predictable runtime performance
* memory safety
* strong type checking
* asynchronous networking
* long-lived WebSocket connections

The HTTP API is built with Axum.

### ScyllaDB

ScyllaDB is used as the primary database for persistent application data.

It is a good fit for message storage because the main access pattern is:

* write messages frequently
* fetch messages by channel
* paginate through message history
* scale storage across multiple nodes

The message table uses `channel_id` as the partition key and timestamp-based clustering.

### Redis

Redis is used for short-lived and frequently accessed data.

Current uses include:

* session data
* user presence
* Pub/Sub between backend instances
* rate limiting
* caching

Redis is not treated as the source of truth for persistent application data.

### Elasticsearch

Elasticsearch is used for message search.

It handles:

* full-text search
* relevance ranking
* searching large message collections

Message data can be rebuilt from ScyllaDB if the search index is lost.

## Data Models

### User

```rust
struct User {
    id: UUID,
    username: String,
    email: String,
    password_hash: String,
    avatar: Option<String>,
    nitro: Option<NitroSubscription>,
    badges: Vec<Badge>,
    created_at: Timestamp
}
```

Storage:

* ScyllaDB: persistent user data
* Redis: cached user data
* Elasticsearch: username search index

### Message

```rust
struct Message {
    id: UUID,
    channel_id: UUID,
    content: String,
    author_id: UUID,
    created_at: Timestamp,
    edited: bool,
    attachments: Vec<Attachment>
}
```

The message table is organized by `channel_id`.

The main query pattern is retrieving recent messages from a channel, with timestamp-based pagination.

Elasticsearch maintains a searchable copy of message content.

### Server

```rust
struct Server {
    id: UUID,
    name: String,
    owner_id: UUID,
    channels: Vec<UUID>,
    members: Vec<ServerMember>,
    roles: Vec<Role>,
    boost_level: i32
}
```

## API

### REST API

The REST API handles authentication, server management, message operations, and search.

```text
GET    /api/auth/me
POST   /api/auth/login
POST   /api/auth/register

GET    /api/servers
POST   /api/servers
GET    /api/servers/:id
PATCH  /api/servers/:id
DELETE /api/servers/:id

GET    /api/channels/:id/messages
POST   /api/messages
PATCH  /api/messages/:id
DELETE /api/messages/:id

POST   /api/messages/search
```

### WebSocket API

WebSockets are used for real-time events.

Connection:

```text
ws://localhost:8080/ws
```

Authentication is performed during the WebSocket handshake using a JWT.

Example client events:

```typescript
{
    type: "channel:join",
    channel_id: "uuid"
}

{
    type: "message:send",
    channel_id: "uuid",
    content: "Hello!"
}

{
    type: "typing:start",
    channel_id: "uuid"
}
```

Example server events:

```typescript
{
    type: "message:new",
    message: { ... }
}

{
    type: "user:status",
    user_id: "uuid",
    status: "online"
}

{
    type: "typing:start",
    user_id: "uuid",
    channel_id: "uuid"
}
```

## Message Flow

A message is handled roughly as follows:

```text
Client
  |
  | POST /api/messages
  v
Backend
  |
  +--> ScyllaDB
  |      store message
  |
  +--> Elasticsearch
  |      index message
  |
  +--> Redis Pub/Sub
         publish event
  |
  v
WebSocket clients
         receive message:new
```

The REST request is responsible for writing the message. Redis Pub/Sub is used to notify other backend instances, which then forward the event to connected WebSocket clients.

## Presence

Online presence is stored in Redis with a TTL.

Typical flow:

1. The client connects through WebSocket.
2. The backend stores the user's presence in Redis.
3. A heartbeat refreshes the TTL while the connection is active.
4. The presence entry is removed or expires after disconnect.
5. Presence changes are broadcast through WebSockets.

Redis Pub/Sub allows presence events to work when multiple backend instances are running.

## Caching

Caching is split between process-local memory and Redis.

### Application Cache

Used for short-lived data such as:

* active connections
* frequently accessed session data

### Redis Cache

Used for data shared between backend instances:

* user profiles
* server metadata
* presence
* rate-limit state

### Persistent Storage

ScyllaDB remains the source of truth for persistent application data.

Cache entries may expire through TTLs or be invalidated when the underlying data changes.

## Database Layout

The message table uses a channel-based partition.

```cql
CREATE TABLE messages (
    channel_id UUID,
    created_at TIMESTAMP,
    id UUID,
    content TEXT,
    author_id UUID,
    PRIMARY KEY (channel_id, created_at, id)
) WITH CLUSTERING ORDER BY (created_at DESC, id DESC);
```

This layout matches the most common message queries.

### Recent Messages

```cql
SELECT * FROM messages
WHERE channel_id = ?
LIMIT 50;
```

### Pagination

```cql
SELECT * FROM messages
WHERE channel_id = ?
AND created_at < ?
LIMIT 50;
```

### Large Channels

A very active channel can eventually create a hot partition.

When that becomes a real bottleneck, messages can be split into time buckets:

```cql
PRIMARY KEY ((channel_id, bucket), created_at, id)
```

The bucket can represent a fixed time period such as a day.

This should only be introduced when the workload actually requires it.

## Scaling

### Backend

Backend instances are stateless apart from local connection state, so multiple instances can run behind a load balancer.

```text
              Load Balancer
                    |
       +------------+------------+
       |            |            |
   Backend 1    Backend 2    Backend N
       |            |            |
       +------------+------------+
                    |
              Redis Pub/Sub
```

WebSocket connections remain attached to individual backend instances. Redis Pub/Sub is used to distribute events between instances.

### ScyllaDB

ScyllaDB can be expanded by adding nodes to the cluster.

The replication and consistency settings should be chosen according to the deployment's availability and consistency requirements rather than using fixed values for every environment.

## Authentication and Authorization

### Authentication

The current authentication flow is:

1. Client submits credentials.
2. Backend validates them.
3. Backend issues a signed JWT.
4. Client sends the token with subsequent requests.
5. Backend validates the token before processing the request.

JWT payloads contain the user identifier and expiration time.

### Authorization

LieCord uses role-based permissions.

```rust
struct Role {
    id: UUID,
    server_id: UUID,
    permissions: Permissions
}

struct Permissions {
    administrator: bool,
    manage_channels: bool,
    send_messages: bool,
    // ...
}
```

A permission check should be performed on the backend before protected actions are allowed.

## Monitoring

Useful metrics include:

### Application

* HTTP request latency
* HTTP error rate
* active WebSocket connections
* dropped connections
* messages processed per second

### Database

* ScyllaDB read/write latency
* partition sizes
* Redis memory usage and hit rate
* Elasticsearch query latency and index size

### Infrastructure

* CPU usage
* memory usage
* disk usage
* network I/O

Prometheus and Grafana can be used for metrics and dashboards.

## Backups and Recovery

### ScyllaDB

Persistent data should be backed up regularly.

Backups should also be tested by restoring them periodically.

### Redis

Redis data is treated as replaceable where possible. Persistent Redis snapshots can still be enabled for deployments that need them.

### Elasticsearch

Search indexes can be recreated from the persistent message data when necessary.

## Deployment

A production deployment should include:

* health checks
* service monitoring
* automated backups
* failure handling
* controlled database migrations
* separate configuration for development and production

Multi-region deployment can be added later if the project's availability requirements justify the added operational complexity.

## Future Work

Possible improvements include:

* message compression
* CDN-backed file uploads
* image resizing and thumbnails
* a dedicated voice service
* message partitioning for very large channels
* API improvements based on actual client usage

These are future options rather than requirements for the current architecture.

---

*Last updated: 2026*
::: 
