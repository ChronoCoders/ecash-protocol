# eCash Protocol - Complete Implementation

Production-ready blind signature eCash protocol implementation in Rust.

## 🎯 Project Status

**✅ COMPLETE** - All core phases implemented and tested.

### Components

- ✅ **ecash-core**: Cryptographic library (RSA blind signatures)
- ✅ **ecash-server**: REST API backend (Axum + PostgreSQL + Redis)
- ✅ **ecash-client**: SDK for wallet management
- ✅ **Production deployment**: Docker Compose + Kubernetes

### Features

- ✅ RSA-3072 blind signatures (Chaum's protocol)
- ✅ Double-spend prevention (Redis + PostgreSQL)
- ✅ Token withdrawal & redemption
- ✅ Wallet SDK with local storage
- ✅ QR code support
- ✅ Complete audit trail
- ✅ Health monitoring
- ✅ Rate limiting
- ✅ Production deployment configs

## 🚀 Quick Start (Docker)

### Prerequisites

- Docker & Docker Compose
- 4GB RAM minimum
- 10GB disk space

### One-Command Setup

```bash
# Clone/extract the project
cd ecash-protocol

# Start everything
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f ecash-server
```

**Services:**
- API Server: http://localhost:8080
- PostgreSQL: localhost:5432
- Redis: localhost:6379
- Nginx: http://localhost:80

### Test the API

```bash
# Health check
curl http://localhost:8080/health

# Get public key
curl http://localhost:8080/api/v1/keys

# Response:
# {
#   "key_id": "key_001",
#   "institution_id": "inst_primary",
#   "public_key_n": "...",
#   "public_key_e": "65537",
#   "denominations": [10, 50, 100, 500, 1000]
# }
```

## 📦 Manual Setup (Development)

### 1. Install Dependencies

```bash
# Rust 1.91+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# PostgreSQL 16
# Ubuntu/Debian:
sudo apt-get install postgresql postgresql-contrib

# Windows: Download from postgresql.org
# macOS: brew install postgresql

# Redis
# Ubuntu/Debian:
sudo apt-get install redis-server

# Windows: Download from github.com/tporadowski/redis
# macOS: brew install redis
```

### 2. Database Setup

```bash
# Start PostgreSQL
sudo systemctl start postgresql  # Linux
# or pg_ctl start  # Windows

# Create database
sudo -u postgres psql
CREATE DATABASE ecash_db;
CREATE USER ecash_user WITH PASSWORD 'ecash_password_2024';
GRANT ALL PRIVILEGES ON DATABASE ecash_db TO ecash_user;
\q

# Load schema
cd crates/ecash-server
psql -U ecash_user -d ecash_db -f schema.sql
```

### 3. Redis Setup

```bash
# Start Redis
sudo systemctl start redis  # Linux
# or redis-server  # Windows/macOS

# Set password
redis-cli
CONFIG SET requirepass redis_password_2024
CONFIG REWRITE
exit
```

### 4. Build & Run

```bash
# Build all crates
cargo build --release

# Run server
cd crates/ecash-server
cargo run --release

# Server starts on http://localhost:8080
```

## 🏗️ Architecture

```
┌─────────────┐     ┌──────────────┐     ┌────────────┐
│   Client    │────▶│ ecash-server │────▶│ PostgreSQL │
│   (Wallet)  │     │   (Axum)     │     │  (Audit)   │
└─────────────┘     └──────────────┘     └────────────┘
                           │
                           ▼
                    ┌────────────┐
                    │   Redis    │
                    │  (Cache)   │
                    └────────────┘
```

### Technology Stack

**Backend:**
- Rust 1.91+
- Axum (web framework)
- PostgreSQL 16 (audit trail)
- Redis 7 (double-spend prevention)
- RSA-3072 (blind signatures)

**Client:**
- Rust SDK
- SQLite (local storage)
- QR code generation

**Deployment:**
- Docker & Docker Compose
- Kubernetes
- Nginx (reverse proxy + rate limiting)

## 📚 Documentation

### API Endpoints

**Health Check**
```
GET /health
```

**Get Public Key**
```
GET /api/v1/keys
```

**Withdraw Tokens**
```
POST /api/v1/withdraw
Content-Type: application/json

{
  "amount": 100,
  "denomination": 50,
  "blinded_tokens": [...]
}
```

**Redeem Tokens**
```
POST /api/v1/redeem
Content-Type: application/json

{
  "tokens": [...],
  "merchant_id": "merchant_123"
}
```

**Verify Token**
```
POST /api/v1/verify
Content-Type: application/json

{
  "token": {...}
}
```

### Client SDK Usage

```rust
use ecash_client::Wallet;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize wallet
    let mut wallet = Wallet::new(
        "http://localhost:8080".to_string(),
        "wallet.db".to_string(),
    )?;
    
    wallet.initialize().await?;
    
    // Withdraw tokens
    let tokens = wallet.withdraw(100, 50).await?;
    println!("Withdrew {} tokens", tokens.len());
    
    // Check balance
    let balance = wallet.get_balance()?;
    println!("Balance: ${}", balance);
    
    // Spend tokens
    let tx_id = wallet.spend(50).await?;
    println!("Transaction: {}", tx_id);
    
    Ok(())
}
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Test specific crate
cargo test -p ecash-core
cargo test -p ecash-server
cargo test -p ecash-client

# With output
cargo test -- --nocapture

# Clippy (linter)
cargo clippy --all-targets

# Format check
cargo fmt --check
```

## 🔐 Security

### Cryptography

- **RSA-3072**: 128-bit security level
- **Blind signatures**: Chaum's protocol
- **SHA-256**: Hashing
- **Constant-time operations**: Side-channel protection

### Double-Spend Prevention

1. **Redis check**: In-memory cache (fast)
2. **PostgreSQL check**: Persistent storage (reliable)
3. **Atomic operations**: Race condition protection

### Network Security

- TLS/HTTPS (production)
- Rate limiting (Nginx)
- Input validation
- SQL injection protection (parameterized queries)

## 📊 Performance

### Target Metrics (Single Node)

- **Withdrawal**: <100ms, 100 req/sec
- **Redemption**: <50ms, 500 req/sec
- **Verification**: <10ms, 1000 req/sec

### Scaling

- **Horizontal**: 3+ API servers
- **Database**: PostgreSQL replication (1 primary + 2 replicas)
- **Cache**: Redis cluster (1 master + 2 replicas)
- **Load balancer**: Nginx

## 🐳 Production Deployment

### Docker Compose

```bash
# Start
docker-compose up -d

# Scale API servers
docker-compose up -d --scale ecash-server=3

# Stop
docker-compose down

# Clean volumes
docker-compose down -v
```

### Kubernetes

```bash
# Apply manifests
kubectl apply -f deployment/kubernetes/

# Check status
kubectl get pods -n ecash

# View logs
kubectl logs -f deployment/ecash-server -n ecash

# Scale servers
kubectl scale deployment ecash-server --replicas=5 -n ecash
```

### Environment Variables

```bash
# Required
DATABASE_URL=postgresql://user:pass@host:5432/db
REDIS_URL=redis://:pass@host:6379
SERVER_PORT=8080

# Optional
INSTITUTION_ID=inst_primary
KEY_ID=key_001
TOKEN_EXPIRY_DAYS=90
DENOMINATIONS=10,50,100,500,1000
RUST_LOG=info,ecash_server=debug
```

## 🔧 Configuration

### Production Checklist

- [ ] Change PostgreSQL password
- [ ] Change Redis password
- [ ] Enable HTTPS/TLS
- [ ] Configure rate limits
- [ ] Set up monitoring
- [ ] Enable audit logging
- [ ] Backup database
- [ ] HSM for key storage (recommended)

## 📁 Project Structure

```
ecash-protocol/
├── crates/
│   ├── ecash-core/          # Cryptographic library
│   ├── ecash-server/        # API backend
│   └── ecash-client/        # Wallet SDK
├── deployment/
│   ├── kubernetes/          # K8s manifests
│   ├── nginx/              # Nginx configs
│   └── monitoring/         # Prometheus/Grafana
├── examples/
│   └── demo_cli.rs         # Demo wallet CLI
├── docker-compose.yml       # Docker setup
└── README.md               # This file
```

## 🆘 Troubleshooting

### Server won't start

```bash
# Check PostgreSQL
pg_isready -h localhost -p 5432

# Check Redis
redis-cli -a redis_password_2024 ping

# Check logs
docker-compose logs ecash-server
```

### Database connection failed

```bash
# Verify credentials in .env
cat crates/ecash-server/.env

# Test connection
psql -U ecash_user -d ecash_db -h localhost
```

### Build errors

```bash
# Clean build
cargo clean
cargo build --release

# Update dependencies
cargo update
```

## 📖 References

- Whitepaper: `ecash-protocol-whitepaper.md`
- Chaum's original paper: "Blind Signatures for Untraceable Payments" (1983)
- Server README: `crates/ecash-server/README.md`
- Client docs: `cargo doc --open`

## 📄 License

See LICENSE file for details.

## 🙏 Credits

Based on David Chaum's blind signature protocol for untraceable payments.

Implementation by: [Your Name]
Date: December 2024
