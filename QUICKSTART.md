# eCash Protocol - Quick Start Guide

## 🚀 Phase 2 Complete: Backend API Server

### ✅ What's Included

```
ecash-protocol/
├── crates/
│   ├── ecash-core/          ✅ Cryptographic library
│   └── ecash-server/        ✅ Backend API server (NEW!)
│       ├── src/
│       │   ├── main.rs      - Server entrypoint
│       │   ├── handlers.rs  - API endpoints
│       │   ├── db.rs        - PostgreSQL operations
│       │   ├── cache.rs     - Redis double-spend prevention
│       │   ├── config.rs    - Configuration management
│       │   ├── state.rs     - Application state
│       │   ├── models.rs    - Database models
│       │   ├── types.rs     - API request/response types
│       │   └── error.rs     - Error handling
│       ├── .env             - Configuration (PostgreSQL + Redis credentials)
│       ├── schema.sql       - Database schema
│       ├── docker-compose.yml - Local development setup
│       ├── Dockerfile       - Container image
│       └── README.md        - Detailed documentation
```

## 📦 Windows Setup (Localhost)

### Prerequisites

1. **PostgreSQL 16+**
   - Download: https://www.postgresql.org/download/windows/
   - Install with default settings

2. **Redis (via WSL2 or Windows build)**
   - WSL2: `wsl --install` then `sudo apt-get install redis-server`
   - Windows: https://github.com/tporadowski/redis/releases

3. **Rust 1.91+**
   - Already installed ✅

### Database Setup

```cmd
REM 1. Open PostgreSQL SQL Shell (psql)
REM Login as postgres user

REM 2. Create database and user
CREATE DATABASE ecash_db;
CREATE USER ecash_user WITH PASSWORD 'ecash_password_2024';
GRANT ALL PRIVILEGES ON DATABASE ecash_db TO ecash_user;
\q

REM 3. Run schema
cd crates\ecash-server
psql -U ecash_user -d ecash_db -f schema.sql
```

### Redis Setup

```cmd
REM WSL2:
wsl
sudo redis-server --requirepass redis_password_2024 --daemonize yes

REM Windows (run in separate terminal):
redis-server.exe --requirepass redis_password_2024
```

### Build & Run

```cmd
REM Build
cargo build --release -p ecash-server

REM Run server
cargo run --release -p ecash-server

REM Or run binary directly
target\release\ecash-server.exe
```

### Test Endpoints

```cmd
REM Health check
curl http://localhost:8080/health

REM Get public key
curl http://localhost:8080/api/v1/keys

REM Response should show:
REM {
REM   "key_id": "key_001",
REM   "institution_id": "inst_primary",
REM   "public_key_n": "...",
REM   "public_key_e": "...",
REM   "denominations": [10, 50, 100, 500, 1000]
REM }
```

## 🐳 Docker Setup (Easier!)

```cmd
cd crates\ecash-server

REM Start everything (PostgreSQL + Redis + Server)
docker-compose up -d

REM View logs
docker-compose logs -f

REM Stop everything
docker-compose down
```

## 🔧 Configuration

Edit `crates/ecash-server/.env`:

```env
# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8080

# PostgreSQL (CHANGE THESE!)
DATABASE_URL=postgresql://ecash_user:YOUR_PASSWORD@localhost:5432/ecash_db

# Redis (CHANGE THESE!)
REDIS_URL=redis://:YOUR_PASSWORD@localhost:6379

# Institution
INSTITUTION_ID=inst_primary
KEY_ID=key_001
TOKEN_EXPIRY_DAYS=90
DENOMINATIONS=10,50,100,500,1000
```

## 📡 API Endpoints

### 1. Health Check
```bash
GET /health
```

### 2. Get Public Key
```bash
GET /api/v1/keys
```

### 3. Withdraw Tokens (Blind Signature Issuance)
```bash
POST /api/v1/withdraw
Content-Type: application/json

{
  "amount": 100,
  "denomination": 50,
  "blinded_tokens": [...]
}
```

### 4. Redeem Tokens (Spend)
```bash
POST /api/v1/redeem
Content-Type: application/json

{
  "tokens": [...],
  "merchant_id": "merchant_123"
}
```

### 5. Verify Token
```bash
POST /api/v1/verify
Content-Type: application/json

{
  "token": {...}
}
```

## 🔐 Security Features

✅ RSA-3072 blind signatures
✅ Redis-based double-spend prevention (in-memory cache)
✅ PostgreSQL audit trail (permanent record)
✅ Atomic database operations
✅ Signature verification on redemption
✅ Token expiration checking
✅ Request logging

## 📊 Database Schema

### Tables
- **tokens**: Spent token serials (prevents double-spend)
- **transactions**: Audit log (withdraw/redeem)
- **signing_keys**: Key rotation support (future)

### Indexes
- serial_hex (fast lookup)
- redeemed_at (time-based queries)
- transaction_type (analytics)

## 🎯 Performance Targets

Single node:
- Withdrawal: <100ms, 100 req/sec
- Redemption: <50ms, 500 req/sec
- Verification: <10ms, 1000 req/sec

## 🔍 Monitoring

Logs are output to stdout/stderr:
```
RUST_LOG=debug cargo run --release -p ecash-server
```

## 📝 Next Steps

### Phase 3: Client SDK (TODO)
- Wallet management
- Token storage
- Transaction history

### Phase 4: Web Frontend (TODO)
- Leptos (Rust WASM)
- User interface
- QR code support

### Production Hardening (TODO)
- Rate limiting
- HSM key storage
- Prometheus metrics
- Load balancing
- TLS/HTTPS

## 🆘 Troubleshooting

### PostgreSQL connection failed
```cmd
REM Check PostgreSQL is running
pg_isready -h localhost -p 5432

REM Check credentials in .env match database
```

### Redis connection failed
```cmd
REM Check Redis is running
redis-cli ping
REM Should return: PONG

REM If password protected:
redis-cli -a redis_password_2024 ping
```

### Build errors
```cmd
REM Clean build
cargo clean
cargo build --release -p ecash-server
```

## 📚 Documentation

- Server README: `crates/ecash-server/README.md`
- API docs: Run `cargo doc --open`
- Whitepaper: `ecash-protocol-whitepaper.md`

## 🎉 Success Checklist

- [ ] PostgreSQL running and database created
- [ ] Redis running with password
- [ ] Server builds without errors
- [ ] Server starts successfully
- [ ] Health endpoint returns 200 OK
- [ ] Public key endpoint returns valid response
- [ ] No errors in logs

---

**Credentials (CHANGE IN PRODUCTION!):**
- PostgreSQL: `ecash_user` / `ecash_password_2024`
- Redis: `redis_password_2024`
- Server Port: `8080`
