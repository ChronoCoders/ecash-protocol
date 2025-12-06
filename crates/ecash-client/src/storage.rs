use crate::error::Result;
use chrono::{DateTime, Utc};
use ecash_core::Token;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredToken {
    pub id: String,
    pub token: Token,
    pub status: TokenStatus,
    pub created_at: DateTime<Utc>,
    pub spent_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TokenStatus {
    Available,
    Spent,
    Pending,
}

impl std::fmt::Display for TokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TokenStatus::Available => "available",
            TokenStatus::Spent => "spent",
            TokenStatus::Pending => "pending",
        };
        write!(f, "{}", s)
    }
}

impl TokenStatus {
    fn from_str(s: &str) -> Self {
        match s {
            "available" => TokenStatus::Available,
            "spent" => TokenStatus::Spent,
            "pending" => TokenStatus::Pending,
            _ => TokenStatus::Available,
        }
    }
}

pub struct WalletStorage {
    conn: Connection,
}

impl WalletStorage {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS tokens (
                id TEXT PRIMARY KEY,
                token_data TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL,
                spent_at TEXT
            )
            "#,
            [],
        )?;
        
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS transactions (
                id TEXT PRIMARY KEY,
                tx_type TEXT NOT NULL,
                amount INTEGER NOT NULL,
                token_count INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                metadata TEXT
            )
            "#,
            [],
        )?;
        
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tokens_status ON tokens(status)",
            [],
        )?;
        
        Ok(Self { conn })
    }

    pub fn store_token(&self, token: Token) -> Result<StoredToken> {
        let stored = StoredToken {
            id: Uuid::new_v4().to_string(),
            token,
            status: TokenStatus::Available,
            created_at: Utc::now(),
            spent_at: None,
        };
        
        let token_json = serde_json::to_string(&stored.token)?;
        
        self.conn.execute(
            "INSERT INTO tokens (id, token_data, status, created_at, spent_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                &stored.id,
                token_json,
                stored.status.to_string(),
                stored.created_at.to_rfc3339(),
                stored.spent_at.map(|dt| dt.to_rfc3339()),
            ],
        )?;
        
        Ok(stored)
    }

    pub fn get_available_tokens(&self) -> Result<Vec<StoredToken>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, token_data, status, created_at, spent_at FROM tokens WHERE status = 'available' ORDER BY created_at"
        )?;
        
        let tokens = stmt.query_map([], |row| {
            let token_json: String = row.get(1)?;
            let token: Token = serde_json::from_str(&token_json).unwrap();
            let created_str: String = row.get(3)?;
            let spent_str: Option<String> = row.get(4)?;
            
            Ok(StoredToken {
                id: row.get(0)?,
                token,
                status: TokenStatus::from_str(&row.get::<_, String>(2)?),
                created_at: DateTime::parse_from_rfc3339(&created_str).unwrap().with_timezone(&Utc),
                spent_at: spent_str.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Utc))),
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
        
        Ok(tokens)
    }

    pub fn mark_tokens_spent(&self, token_ids: &[String]) -> Result<()> {
        let tx = self.conn.unchecked_transaction()?;
        
        for id in token_ids {
            tx.execute(
                "UPDATE tokens SET status = 'spent', spent_at = ?1 WHERE id = ?2",
                params![Utc::now().to_rfc3339(), id],
            )?;
        }
        
        tx.commit()?;
        Ok(())
    }

    pub fn get_balance(&self) -> Result<u64> {
        let mut stmt = self.conn.prepare(
            "SELECT token_data FROM tokens WHERE status = 'available'"
        )?;
        
        let mut total = 0u64;
        let tokens = stmt.query_map([], |row| {
            let token_json: String = row.get(0)?;
            Ok(token_json)
        })?;
        
        for token_result in tokens {
            let token_json = token_result?;
            let token: Token = serde_json::from_str(&token_json)?;
            total += token.denomination;
        }
        
        Ok(total)
    }

    pub fn log_transaction(&self, tx_type: &str, amount: u64, token_count: usize, metadata: Option<String>) -> Result<()> {
        self.conn.execute(
            "INSERT INTO transactions (id, tx_type, amount, token_count, created_at, metadata) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                Uuid::new_v4().to_string(),
                tx_type,
                amount as i64,
                token_count as i64,
                Utc::now().to_rfc3339(),
                metadata,
            ],
        )?;
        
        Ok(())
    }
}
