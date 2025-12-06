use ecash_client::Wallet;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=================================");
    println!("    eCash Protocol - Demo CLI");
    println!("=================================\n");

    let server_url = std::env::var("ECASH_SERVER_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());
    
    let db_path = std::env::var("ECASH_DB_PATH")
        .unwrap_or_else(|_| "wallet.db".to_string());

    println!("Server: {}", server_url);
    println!("Database: {}\n", db_path);

    let mut wallet = Wallet::new(server_url, db_path)?;
    
    println!("Initializing wallet...");
    wallet.initialize().await?;
    println!("✓ Wallet initialized\n");

    loop {
        println!("\n--- Menu ---");
        println!("1. Check balance");
        println!("2. Withdraw tokens");
        println!("3. Spend tokens");
        println!("4. List tokens");
        println!("5. Health check");
        println!("6. Exit");
        print!("\nChoice: ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "1" => check_balance(&wallet).await?,
            "2" => withdraw_tokens(&wallet).await?,
            "3" => spend_tokens(&wallet).await?,
            "4" => list_tokens(&wallet).await?,
            "5" => health_check(&wallet).await?,
            "6" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }

    Ok(())
}

async fn check_balance(wallet: &Wallet) -> anyhow::Result<()> {
    let balance = wallet.get_balance()?;
    println!("\n💰 Balance: ${}", balance);
    Ok(())
}

async fn withdraw_tokens(wallet: &Wallet) -> anyhow::Result<()> {
    print!("Amount: $");
    io::stdout().flush()?;
    let mut amount = String::new();
    io::stdin().read_line(&mut amount)?;
    let amount: u64 = amount.trim().parse()?;

    print!("Denomination (10/50/100/500/1000): $");
    io::stdout().flush()?;
    let mut denom = String::new();
    io::stdin().read_line(&mut denom)?;
    let denom: u64 = denom.trim().parse()?;

    println!("\n⏳ Withdrawing...");
    let tokens = wallet.withdraw(amount, denom).await?;
    println!("✓ Withdrew {} tokens!", tokens.len());
    
    Ok(())
}

async fn spend_tokens(wallet: &Wallet) -> anyhow::Result<()> {
    print!("Amount to spend: $");
    io::stdout().flush()?;
    let mut amount = String::new();
    io::stdin().read_line(&mut amount)?;
    let amount: u64 = amount.trim().parse()?;

    println!("\n⏳ Spending...");
    let tx_id = wallet.spend(amount).await?;
    println!("✓ Spent! Transaction ID: {}", tx_id);
    
    Ok(())
}

async fn list_tokens(wallet: &Wallet) -> anyhow::Result<()> {
    let tokens = wallet.get_available_tokens()?;
    
    if tokens.is_empty() {
        println!("\nNo tokens available");
        return Ok(());
    }

    println!("\n📋 Available Tokens:");
    println!("{:<10} {:<15} {:<20}", "Denom", "Status", "Created");
    println!("{}", "-".repeat(50));
    
    for token in tokens {
        println!(
            "${:<9} {:<15} {}",
            token.token.denomination,
            "Available",
            token.created_at.format("%Y-%m-%d %H:%M")
        );
    }
    
    Ok(())
}

async fn health_check(wallet: &Wallet) -> anyhow::Result<()> {
    println!("\n⏳ Checking server health...");
    let healthy = wallet.health_check().await?;
    
    if healthy {
        println!("✓ Server is healthy");
    } else {
        println!("✗ Server is not responding");
    }
    
    Ok(())
}
