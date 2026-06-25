# Fraud Velocity API (Rust)

A portfolio-ready backend service for an identity-verification workflow. It exposes a low-latency REST endpoint that detects repeated verification attempts from the same subject/IP pair in a rolling five-minute window.

## Why this is worth showcasing
- **Rust + Axum + Tokio**: async HTTP service with shared concurrent state.
- **Security-oriented domain**: rate/velocity controls are common in fraud and identity systems.
- **Production signals**: structured logging, request tracing, CORS, typed JSON contracts, health endpoint, meaningful HTTP status codes.
- **Clear next steps**: swap the in-memory store for Redis and add PostgreSQL audit persistence.

## Run
```bash
cargo run
```

## Try it
```bash
curl http://localhost:3000/health

curl -X POST http://localhost:3000/v1/verifications \
  -H 'content-type: application/json' \
  -d '{"subject_id":"user_42","ip_address":"203.0.113.10","country":"IN"}'
```

Repeat the POST six times within five minutes to receive `429 Too Many Requests` and a retry time.

## Architecture
`Axum handler → Tokio RwLock → HashMap<subject:ip, VecDeque<timestamp>> → decision JSON`

## Resume bullet
Built an async Rust/Axum fraud-velocity API using Tokio concurrency primitives; implemented rolling-window detection, typed REST contracts, HTTP tracing, and risk decisions for identity-verification workflows.

## Improvements to implement
1. Redis sorted sets with TTL for multi-instance correctness.
2. PostgreSQL audit log via SQLx.
3. API-key middleware and per-tenant limits.
4. OpenAPI spec, integration tests, Dockerfile, and CI.
