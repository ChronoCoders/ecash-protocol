# Demo Wallet CLI

Interactive command-line wallet for testing the eCash protocol.

## Prerequisites

Make sure the server is running:

```bash
# In one terminal:
docker-compose up -d

# Verify server is running:
curl http://localhost:8080/health
```

## Running the Demo

**IMPORTANT for Windows:** Due to stack size limitations in debug mode, use release mode:

```cmd
cd C:\ecash-protocol

REM Build and run in release mode (RECOMMENDED)
cargo run --release -p demo-wallet

REM Alternative: Build first, then run
cargo build --release -p demo-wallet
target\release\demo-wallet.exe
```

### Windows (Debug Mode - May Crash)

```cmd
cd C:\ecash-protocol

REM Debug mode (may cause stack overflow on withdraw)
cargo run -p demo-wallet
```

### Linux/Mac

```bash
cd ecash-protocol

# Run the demo wallet (debug mode works fine on Unix)
cargo run -p demo-wallet

# Or build and run the binary:
cargo build --release -p demo-wallet
./target/release/demo-wallet
```
cargo build --release -p demo-wallet
./target/release/demo-wallet
```

## Usage

The wallet will connect to `http://localhost:8080` by default.

**Menu Options:**

1. **Check balance** - View your current wallet balance
2. **Withdraw tokens** - Withdraw new tokens from the server
   - Enter amount (e.g., 100)
   - Enter denomination (10, 50, 100, 500, or 1000)
   - Tokens are blindly signed by the server
3. **Spend tokens** - Redeem tokens with the server
   - Enter amount to spend
   - Wallet automatically selects tokens
4. **List tokens** - View all available tokens in your wallet
5. **Health check** - Check if the server is responding
6. **Exit** - Close the wallet

## Example Session

```
=================================
    eCash Protocol - Demo CLI
=================================

Server: http://localhost:8080
Database: wallet.db

Initializing wallet...
✓ Wallet initialized

--- Menu ---
1. Check balance
2. Withdraw tokens
3. Spend tokens
4. List tokens
5. Health check
6. Exit

Choice: 2
Amount: $100
Denomination (10/50/100/500/1000): $50

⏳ Withdrawing...
✓ Withdrew 2 tokens!

--- Menu ---
Choice: 1

💰 Balance: $100

--- Menu ---
Choice: 3
Amount to spend: $50

⏳ Spending...
✓ Spent! Transaction ID: 123e4567-e89b-12d3-a456-426614174000

--- Menu ---
Choice: 1

💰 Balance: $50
```

## Environment Variables

```bash
# Optional: Customize server URL
export ECASH_SERVER_URL=http://localhost:8080

# Optional: Customize database path
export ECASH_DB_PATH=my_wallet.db

cargo run -p demo-wallet
```

## What's Happening?

1. **Initialize**: Fetches the server's RSA public key
2. **Withdraw**: 
   - Generates random serial numbers
   - Blinds the tokens (hides serial from server)
   - Server signs the blinded tokens
   - Unblind the signatures
   - Store tokens locally in SQLite
3. **Spend**:
   - Select tokens from local wallet
   - Send tokens to server
   - Server verifies signatures
   - Server checks for double-spending
   - Mark tokens as spent locally

## Database

Wallet data is stored in `wallet.db` (SQLite):
- Available tokens
- Spent tokens
- Transaction history

To start fresh:
```bash
rm wallet.db
cargo run -p demo-wallet
```

## Troubleshooting

**Server not running:**
```
Error: Connection refused

Solution: Start the server first:
docker-compose up -d
```

**Invalid denomination:**
```
Error: Denomination not supported

Solution: Use: 10, 50, 100, 500, or 1000
```

**Insufficient balance:**
```
Error: Insufficient balance

Solution: Withdraw more tokens first (option 2)
```

## Building with Client Workspace

The demo wallet uses a separate workspace to avoid Docker build conflicts:

```cmd
REM Windows - Use client workspace
cargo run --release --manifest-path Cargo-client.toml -p demo-wallet

REM Or set as alias
set CARGO_MANIFEST=Cargo-client.toml
cargo run --release -p demo-wallet
```

```bash
# Linux/Mac - Use client workspace
cargo run --release --manifest-path Cargo-client.toml -p demo-wallet
```
