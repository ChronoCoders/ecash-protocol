# eCash Protocol Server

Production-grade backend API server implementing the eCash blind signature protocol.

## Architecture

- **Framework**: Axum (async Rust web framework)
- **Database**: PostgreSQL (audit trail, token tracking)
- **Cache**: Redis (double-spend prevention, hot data)
- **Crypto**: RSA-3072 blind signatures

## Features

- ✅ Token withdrawal (blind signature issuance)
- ✅ Token redemption (verification + double-spend check)
- ✅ Token verification
- ✅ Public key distribution
- ✅ Audit logging
- ✅ Health monitoring

## Prerequisites

### PostgreSQL Setup

```bash
# Install PostgreSQL
sudo apt-get install postgresql postgresql-contrib

# Create database and user
sudo -u postgres psql
```

```sql
CREATE DATABASE ecash_db;
CREATE USER ecash_user WITH PASSWORD 'ecash_password_2024';
GRANT ALL PRIVILEGES ON DATABASE ecash_db TO ecash_user;
\q
```

```bash
# Run schema
psql -U ecash_user -d ecash_db -f schema.sql
```

### Redis Setup

```bash
# Install Redis
sudo apt-get install redis-server

# Set password in /etc/redis/redis.conf
requirepass redis_password_2024

# Restart Redis
sudo systemctl restart redis-server
```

## Configuration

Edit `.env` file:

```env
# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8080

# PostgreSQL
DATABASE_URL=postgresql://ecash_user:ecash_password_2024@localhost:5432/ecash_db
DATABASE_MAX_CONNECTIONS=10

# Redis
REDIS_URL=redis://:redis_password_2024@localhost:6379
REDIS_MAX_CONNECTIONS=10

# Institution
INSTITUTION_ID=inst_primary
KEY_ID=key_001
TOKEN_EXPIRY_DAYS=90
DENOMINATIONS=10,50,100,500,1000

# Logging
RUST_LOG=info,ecash_server=debug
```

## Build & Run

```bash
# Build
cargo build --release

# Run
cargo run --release

# Or run binary
./target/release/ecash-server
```

## API Endpoints

### Health Check
```bash
GET /health
```

### Get Public Key
```bash
GET /api/v1/keys
```

### Withdraw Tokens
```bash
POST /api/v1/withdraw
Content-Type: application/json

{
  "amount": 100,
  "denomination": 50,
  "blinded_tokens": [...]
}
```

### Redeem Tokens
```bash
POST /api/v1/redeem
Content-Type: application/json

{
  "tokens": [...],
  "merchant_id": "merchant_123"
}
```

### Verify Token
```bash
POST /api/v1/verify
Content-Type: application/json

{
  "token": {...}
}
```

## Testing

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test '*'

# Health check
curl http://localhost:8080/health

# Get public key
curl http://localhost:8080/api/v1/keys
```

## Performance

Target metrics (single node):
- Withdrawal: <100ms, 100 req/sec
- Redemption: <50ms, 500 req/sec
- Verification: <10ms, 1000 req/sec

## Security

- RSA-3072 keys (128-bit security)
- Constant-time signature operations
- Redis-based double-spend prevention
- PostgreSQL audit trail
- Rate limiting (TODO)
- HSM integration (TODO)

## Monitoring

Metrics exposed at `/metrics` (TODO):
- Request latency (p50, p95, p99)
- Request rate
- Error rate
- Database connection pool
- Redis connection pool
- Token issuance/redemption counts

## Production Deployment

See `deployment/` directory for:
- Docker containers
- Kubernetes manifests
- Nginx reverse proxy config
- Systemd service files

## License

See LICENSE file in root directory.
