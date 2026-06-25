# Rust Async Job Queue

A background-task processing service built with Rust, Axum, and Tokio.

## What it demonstrates
This is a different category of project from fraud/identity APIs: it models a core backend infrastructure component. Clients submit long-running jobs; an async worker consumes them in the background; clients poll job status.

- Async HTTP API with Axum
- Background worker using Tokio tasks
- Concurrent shared state with `Arc<RwLock<...>>` and `Mutex`
- Queue lifecycle: `queued → processing → completed`
- UUID job IDs and typed JSON contracts
- HTTP observability via request tracing

## Run
```bash
cargo run
```

## Try it
Create a job:
```bash
curl -X POST http://localhost:3000/v1/jobs   -H "content-type: application/json"   -d '{"task":"generate_report","payload":"{"customer_id":"42"}"}'
```

Copy the returned `id`, then:
```bash
curl http://localhost:3000/v1/jobs/<JOB_ID>
```

## Architecture
```
Client → Axum API → in-memory queue → Tokio worker → job-status store
```

## Production roadmap
- Replace the in-memory queue with Redis Streams, RabbitMQ, or Kafka.
- Add retry policies, exponential backoff, dead-letter queues, and idempotency keys.
- Persist job state in PostgreSQL using SQLx.
- Add Docker Compose, Prometheus metrics, OpenAPI docs, and integration tests.

## Resume bullet
Built an asynchronous job-processing API in Rust using Axum and Tokio; implemented concurrent queue management, background workers, job lifecycle tracking, and typed REST endpoints.
