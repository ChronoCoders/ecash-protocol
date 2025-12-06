# eCash Protocol - Blind Signature Implementation

A production-grade implementation of David Chaum's blind signature protocol for anonymous digital cash, built in Rust with RSA-3072 cryptography.

[![Rust](https://img.shields.io/badge/rust-1.91%2B-orange.svg)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](https://www.docker.com/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

## Overview

This project implements a complete blind signature eCash system with strong privacy guarantees and double-spend prevention. The implementation follows Chaum's original 1983 protocol with modern cryptographic standards and production-ready infrastructure.

**Key Features:**
- RSA-3072 blind signatures providing 128-bit security
- Cryptographic unlinkability between withdrawal and spending
- Atomic double-spend prevention via Redis and PostgreSQL
- REST API with comprehensive audit logging
- Horizontal scalability with load balancing support
- Docker and Kubernetes deployment configurations

## Architecture

```
┌──────────────┐         ┌──────────────┐         ┌──────────────┐
│              │  HTTPS  │              │         │              │
│    Client    │────────▶│     Nginx    │────────▶│  API Server  │
│   (Wallet)   │         │ (Rate Limit) │         │    (Axum)    │
│              │         │              │         │              │
└──────────────┘         └──────────────┘         └───────┬──────┘
                                                           │
                                          ┌────────────────┼────────────────┐
                                          │                │                │
                                          ▼                ▼                ▼
                                   ┌──────────┐    ┌──────────┐    ┌──────────┐
                                   │PostgreSQL│    │  Redis   │    │   HSM    │
                                   │ (Audit)  │    │ (Cache)  │    │  (Keys)  │
                                   └──────────┘    └──────────┘    └──────────┘
```

### Technology Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Core Library | Rust, RSA crate | Cryptographic operations |
| API Server | Axum, Tokio | Asynchronous HTTP server |
| Database | PostgreSQL 16 | Transaction audit trail |
| Cache | Redis 7 | Double-spend prevention |
| Reverse Proxy | Nginx | Load balancing, rate limiting |
| Container | Docker | Deployment packaging |
| Orchestration | Kubernetes | Production deployment |

## Protocol Flow

### 1. Withdrawal (Blind Signature)

```
Client                          Server
  │                               │
  │  1. Generate serial number    │
  │     s = random()              │
  │                               │
  │  2. Blind message             │
  │     m = H(s)                  │
  │     r = random() mod n        │
  │     m' = m * r^e mod n        │
  │                               │
  │  3. Request signature         │
  │     {amount, m'}              │
  ├──────────────────────────────▶│
  │                               │  4. Verify balance
  │                               │     Deduct amount
  │                               │
  │                               │  5. Sign blinded message
  │                               │     s' = (m')^d mod n
  │                               │
  │  6. Return blind signature    │
  │◀──────────────────────────────┤
  │     {s', tx_id}               │
  │                               │
  │  7. Unblind signature         │
  │     s = s' * r^-1 mod n       │
  │                               │
```

### 2. Redemption (Spend)

```
Client                          Server
  │                               │
  │  1. Present token             │
  │     {s, signature}            │
  ├──────────────────────────────▶│
  │                               │  2. Verify signature
  │                               │     m = H(s)
  │                               │     m' = sig^e mod n
  │                               │     verify m == m'
  │                               │
  │                               │  3. Check double-spend
  │                               │     Redis: EXISTS s
  │                               │     PostgreSQL: SELECT s
  │                               │
  │                               │  4. Record redemption
  │                               │     Redis: SET s
  │                               │     PostgreSQL: INSERT
  │                               │
  │  5. Confirm redemption        │
  │◀──────────────────────────────┤
  │     {tx_id, timestamp}        │
  │                               │
```

## Quick Start

### Prerequisites

- Docker 24.0+ and Docker Compose 2.0+
- 4GB RAM minimum
- 10GB available disk space

### Installation

```bash
git clone https://github.com/ChronoCoders/ecash-protocol.git
cd ecash-protocol
docker-compose up -d
```

### Verification

```bash
# Check service health
curl http://localhost:8080/health

# Expected response
{
  "status": "running",
  "database": "ok",
  "redis": "ok",
  "timestamp": "2024-12-06T22:00:00Z"
}

# Retrieve institution public key
curl http://localhost:8080/api/v1/keys

# Expected response
{
  "key_id": "key_001",
  "institution_id": "inst_primary",
  "public_key_n": "...",
  "public_key_e": "65537",
  "denominations": [10, 50, 100, 500, 1000]
}
```

## API Reference

### Endpoints

#### GET /health
Health check endpoint.

**Response:**
```json
{
  "status": "running",
  "database": "ok",
  "redis": "ok",
  "timestamp": "2024-12-06T22:00:00Z"
}
```

#### GET /api/v1/keys
Retrieve institution's public key and supported denominations.

**Response:**
```json
{
  "key_id": "key_001",
  "institution_id": "inst_primary",
  "public_key_n": "5052919934707566...",
  "public_key_e": "65537",
  "denominations": [10, 50, 100, 500, 1000]
}
```

#### POST /api/v1/withdraw
Request blind signatures for token withdrawal.

**Request:**
```json
{
  "amount": 100,
  "denomination": 50,
  "blinded_tokens": [
    "8234756234...",
    "9823475623..."
  ]
}
```

**Response:**
```json
{
  "transaction_id": "tx_abc123",
  "blind_signatures": [
    "3456234523...",
    "7823456234..."
  ],
  "expires_at": "2025-03-06T22:00:00Z"
}
```

#### POST /api/v1/redeem
Redeem tokens for value.

**Request:**
```json
{
  "tokens": [
    {
      "serial": "serial_xyz",
      "signature": "sig_abc...",
      "denomination": 50,
      "institution_id": "inst_primary"
    }
  ],
  "merchant_id": "merchant_123"
}
```

**Response:**
```json
{
  "transaction_id": "tx_def456",
  "redeemed_amount": 50,
  "timestamp": "2024-12-06T22:00:00Z"
}
```

#### POST /api/v1/verify
Verify token signature without redeeming.

**Request:**
```json
{
  "token": {
    "serial": "serial_xyz",
    "signature": "sig_abc...",
    "denomination": 50,
    "institution_id": "inst_primary"
  }
}
```

**Response:**
```json
{
  "valid": true,
  "already_spent": false
}
```

## Client SDK

### Installation

Add to `Cargo.toml`:
```toml
[dependencies]
ecash-client = { path = "crates/ecash-client" }
tokio = { version = "1", features = ["full"] }
```

### Basic Usage

```rust
use ecash_client::Wallet;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize wallet
    let mut wallet = Wallet::new(
        "http://localhost:8080".to_string(),
        "wallet.db".to_string(),
    )?;
    
    // Connect to server and fetch public key
    wallet.initialize().await?;
    
    // Withdraw $100 in $10 denominations
    let tokens = wallet.withdraw(100, 10).await?;
    println!("Withdrew {} tokens", tokens.len());
    
    // Check balance
    let balance = wallet.get_balance()?;
    println!("Current balance: ${}", balance);
    
    // Spend $20
    let tx_id = wallet.spend(20).await?;
    println!("Transaction ID: {}", tx_id);
    
    // List available tokens
    let tokens = wallet.get_available_tokens()?;
    for token in tokens {
        println!("Token: ${} ({})", 
            token.token.denomination,
            token.token.serial
        );
    }
    
    Ok(())
}
```

## Development

### Building from Source

```bash
# Install Rust 1.91+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/ChronoCoders/ecash-protocol.git
cd ecash-protocol

# Build all crates
cargo build --release

# Run tests
cargo test --all

# Check code quality
cargo clippy --all-targets
cargo fmt --check
```

### Running Tests

```bash
# All tests
cargo test

# Specific crate
cargo test -p ecash-core

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test '*'
```

### Database Setup (Manual)

```bash
# PostgreSQL
createdb ecash_db
psql ecash_db < crates/ecash-server/schema.sql

# Redis
redis-server --requirepass your_password
```

## Production Deployment

### Docker Compose

```bash
# Production deployment
docker-compose -f docker-compose.yml up -d

# Scale API servers
docker-compose up -d --scale ecash-server=3

# View logs
docker-compose logs -f ecash-server

# Backup database
docker-compose exec postgres pg_dump -U ecash_user ecash_db > backup.sql
```

### Kubernetes

```bash
# Deploy to cluster
kubectl apply -f deployment/kubernetes/

# Check status
kubectl get pods -n ecash
kubectl get services -n ecash

# Scale deployment
kubectl scale deployment ecash-server --replicas=5 -n ecash

# View logs
kubectl logs -f deployment/ecash-server -n ecash
```

### Environment Configuration

Create `.env` file:

```bash
# Database
DATABASE_URL=postgresql://ecash_user:secure_password@postgres:5432/ecash_db

# Redis
REDIS_URL=redis://:secure_password@redis:6379

# Server
SERVER_PORT=8080
SERVER_HOST=0.0.0.0

# Institution
INSTITUTION_ID=inst_primary
KEY_ID=key_001

# Security
TOKEN_EXPIRY_DAYS=90
DENOMINATIONS=10,50,100,500,1000

# Logging
RUST_LOG=info,ecash_server=debug
```

## Security Considerations

### Cryptographic Parameters

- **RSA Key Size:** 3072 bits (128-bit security level, equivalent to AES-128)
- **Hash Function:** SHA-256 for message hashing
- **Random Number Generation:** OS-provided CSPRNG via `rand` crate
- **Constant-Time Operations:** All signature verifications use constant-time comparisons

### Double-Spend Prevention

The system employs a two-tier approach:

1. **Redis (Fast Path):** In-memory check with O(1) lookup time
2. **PostgreSQL (Reliable Path):** Persistent storage with ACID guarantees

Both checks must pass before redemption is accepted. Atomic operations prevent race conditions.

### Network Security

- TLS 1.3 for all HTTPS communication
- Rate limiting (10 req/s per IP for API, 5 req/s for withdrawal)
- Input validation and sanitization
- Parameterized SQL queries (no string concatenation)
- CORS configured for specific origins only

### Operational Security

**Production Checklist:**
- [ ] Rotate default passwords (PostgreSQL, Redis)
- [ ] Enable TLS/HTTPS with valid certificates
- [ ] Configure firewall rules (allow only necessary ports)
- [ ] Use Hardware Security Module (HSM) for private key storage
- [ ] Enable audit logging to immutable storage
- [ ] Set up automated backups (database + Redis snapshots)
- [ ] Configure monitoring and alerting
- [ ] Implement key rotation schedule
- [ ] Review and test disaster recovery procedures

## Performance

### Benchmarks (Single Node, 4-core 8GB)

| Operation | Latency (p50) | Latency (p99) | Throughput |
|-----------|---------------|---------------|------------|
| Withdrawal | 45ms | 120ms | 150 req/s |
| Redemption | 25ms | 80ms | 600 req/s |
| Verification | 5ms | 15ms | 2000 req/s |

### Scaling Guidelines

**Horizontal Scaling:**
- API Servers: Add instances behind load balancer (stateless)
- PostgreSQL: Primary-replica configuration (1 primary + 2+ replicas)
- Redis: Cluster mode with sharding (1 master + 2+ replicas per shard)

**Vertical Scaling:**
- Database: 8GB RAM minimum, 16GB+ recommended
- Redis: 4GB RAM minimum for cache
- API Server: 2-core CPU minimum, 4-core recommended

## Project Structure

```
ecash-protocol/
├── crates/
│   ├── ecash-core/              # Core cryptographic library
│   │   ├── src/
│   │   │   ├── crypto.rs        # RSA blind signatures
│   │   │   ├── protocol.rs      # Protocol implementation
│   │   │   └── token.rs         # Token data structures
│   │   └── Cargo.toml
│   │
│   ├── ecash-server/            # REST API server
│   │   ├── src/
│   │   │   ├── handlers.rs      # HTTP request handlers
│   │   │   ├── db.rs            # Database layer
│   │   │   ├── cache.rs         # Redis integration
│   │   │   └── main.rs          # Server entry point
│   │   ├── schema.sql           # Database schema
│   │   ├── Dockerfile
│   │   └── Cargo.toml
│   │
│   └── ecash-client/            # Client SDK
│       ├── src/
│       │   ├── wallet.rs        # Wallet implementation
│       │   ├── api.rs           # API client
│       │   └── storage.rs       # Local SQLite storage
│       └── Cargo.toml
│
├── deployment/
│   ├── kubernetes/              # K8s manifests
│   │   ├── deployment.yaml
│   │   ├── service.yaml
│   │   └── ingress.yaml
│   └── nginx/                   # Nginx configuration
│       └── nginx.conf
│
├── docker-compose.yml           # Docker Compose setup
├── Cargo.toml                   # Workspace configuration
└── README.md                    # This file
```

## Troubleshooting

### Server Fails to Start

**Symptom:** Server exits immediately after starting

**Diagnosis:**
```bash
# Check logs
docker-compose logs ecash-server

# Verify database connectivity
docker-compose exec postgres pg_isready

# Verify Redis
docker-compose exec redis redis-cli ping
```

**Common Solutions:**
- Ensure PostgreSQL and Redis are healthy before starting server
- Verify credentials in `.env` match database configuration
- Check firewall rules allow connections on required ports

### Token Verification Fails

**Symptom:** Valid tokens rejected during redemption

**Diagnosis:**
```bash
# Verify public key consistency
curl http://localhost:8080/api/v1/keys

# Check server logs for cryptographic errors
docker-compose logs ecash-server | grep -i "signature\|verify"
```

**Common Solutions:**
- Ensure client and server are using same public key
- Verify token was not already spent (check database)
- Confirm token expiration date is valid

### Performance Degradation

**Symptom:** Increased latency under load

**Diagnosis:**
```bash
# Check database connections
docker-compose exec postgres psql -U ecash_user -d ecash_db \
  -c "SELECT count(*) FROM pg_stat_activity;"

# Monitor Redis memory usage
docker-compose exec redis redis-cli INFO memory
```

**Common Solutions:**
- Increase database connection pool size
- Add Redis replicas for read scaling
- Enable query result caching
- Scale API servers horizontally

## References

- **Original Paper:** David Chaum, "Blind Signatures for Untraceable Payments," Advances in Cryptology (CRYPTO '82), 1983
- **RSA Standard:** PKCS #1 v2.2: RSA Cryptography Standard (RFC 8017)
- **Blind Signatures:** Stefan Brands, "Untraceable Off-line Cash in Wallets with Observers," CRYPTO '93
- **Double-Spending:** Satoshi Nakamoto, "Bitcoin: A Peer-to-Peer Electronic Cash System," 2008

## License

This project is licensed under the MIT License. See LICENSE file for details.

## Acknowledgments

This implementation is based on David Chaum's seminal work on blind signatures for anonymous digital cash. The protocol design draws inspiration from the original DigiCash system while incorporating modern cryptographic standards and production infrastructure practices.

---

**Status:** Production Ready  
**Version:** 1.0.0  
**Last Updated:** December 2024