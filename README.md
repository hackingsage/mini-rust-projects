# Async Rust Backend Services

This repository contains a collection of portfolio-ready backend production services built using **Rust**, **Axum**, and **Tokio**. These projects demonstrate foundational backend architecture patterns, high-concurrency handling, thread-safe memory management, and security-focused domain logic.

---

## 1. Rust Async Job Queue

A background-task processing service that models core backend infrastructure components. Instead of immediate processing, clients submit long-running jobs, an async worker consumes them in the background, and clients poll the service for job status updates.

### What It Demonstrates

* **Async HTTP API:** Powered by Axum for handling incoming job requests.
* **Background Worker Processing:** Managed concurrently using isolated Tokio tasks.
* **Concurrent Shared State:** Safe multi-threaded state coordination using `Arc<RwLock<...>>` and `Mutex`.
* **Robust Lifecycle Tracking:** Models a clear queue lifecycle state machine: `queued` $\rightarrow$ `processing` $\rightarrow$ `completed`.
* **Type-Safe Design:** Strong typing using UUID job IDs and robust JSON serialization/deserialization contracts.
* **HTTP Observability:** Full request tracing integration for structured instrumentation.

### Architecture

```text
Client ──> Axum API ──> In-Memory Queue ──> Tokio Worker ──> Job-Status Store

```

### Quick Start & Usage

#### Run the service:

```bash
cargo run

```

#### Create a background job:

```bash
curl -X POST http://localhost:3000/v1/jobs \
  -H "content-type: application/json" \
  -d '{"task":"generate_report","payload":{"customer_id":"42"}}'

```

#### Poll for job status (using the returned `id`):

```bash
curl http://localhost:3000/v1/jobs/<JOB_ID>

```

### Production Roadmap

* Replace the underlying in-memory queue with production message brokers like **Redis Streams**, **RabbitMQ**, or **Kafka**.
* Implement fault-tolerant retry policies, exponential backoff, dead-letter queues (DLQ), and idempotency keys.
* Persist long-term job states in **PostgreSQL** using SQLx.
* Add operational configurations: Docker Compose, Prometheus metrics, OpenAPI/Swagger documentation, and end-to-end integration tests.

---

## 2. Fraud Velocity API

A low-latency, security-oriented identity-verification service. It exposes a high-performance REST endpoint designed to detect and block rapid, repeated validation attempts originating from the same subject/IP pair within a rolling 5-minute window.

### Why This Is Worth Showcasing

* **Security-Oriented Domain:** Real-world applicability modeling rate/velocity compliance controls standard across enterprise fraud-prevention systems.
* **Production-Grade Signals:** Fully instrumented with structured logging, cross-origin resource sharing (CORS), typed JSON contracts, native health endpoints, and accurate HTTP status responses.
* **Efficient Memory Modeling:** Utilizes a highly efficient tracking structure via sliding timestamp deques.

### Architecture

```text
Axum Handler ──> Tokio RwLock ──> HashMap<subject:ip, VecDeque<timestamp>> ──> Decision JSON

```

### Quick Start & Usage

#### Run the service:

```bash
cargo run

```

#### Check health status:

```bash
curl http://localhost:3000/health

```

#### Submit a verification attempt:

```bash
curl -X POST http://localhost:3000/v1/verifications \
  -H 'content-type: application/json' \
  -d '{"subject_id":"user_42","ip_address":"203.0.113.10","country":"IN"}'

```

> **Note:** Repeating this POST request 6 times within a 5-minute window will trigger rate limits, returning a `429 Too Many Requests` status along with a dynamic cooldown/retry time.

### Future Improvements

1. Transition to **Redis sorted sets** with TTL expiration to support stateless multi-instance correctness.
2. Integrate an audit log data stream to a **PostgreSQL** persistence layer using SQLx.
3. Build out API-key middleware alongside per-tenant rate limitations.
4. Export OpenAPI specifications, configure standard Dockerfiles, and establish automated CI/CD validation pipelines.
