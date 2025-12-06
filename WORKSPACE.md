# Workspace Configuration

## Problem
- Docker needs: `ecash-core` + `ecash-server` only
- Local dev needs: All crates including `ecash-client` + `demo-wallet`

## Solution

### For Docker Build
Keep `Cargo.toml` minimal:
```toml
[workspace]
members = [
    "crates/ecash-core",
    "crates/ecash-server",
]
```

### For Local Development (Demo Wallet)
Edit `Cargo.toml` temporarily:
```toml
[workspace]
members = [
    "crates/ecash-core",
    "crates/ecash-server",
    "crates/ecash-client",
    "crates/demo-wallet",
]
```

## Quick Commands

**Docker (no changes needed):**
```bash
docker-compose up -d --build
```

**Demo Wallet (add to workspace first):**
```bash
# Edit Cargo.toml to include client crates
cargo run --release -p demo-wallet
```

**After demo testing, revert Cargo.toml for Docker:**
```bash
# Remove client crates from members list
git checkout Cargo.toml  # or manually edit
```
