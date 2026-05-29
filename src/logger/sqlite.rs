use rusqlite::{Connection, Result};
use std::path::Path;

pub struct SqliteLogger {
    conn: Connection,
}

impl SqliteLogger {
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS test_sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                start_time TEXT,
                duration_ms INTEGER,
                status TEXT,
                description TEXT
            );
            
            CREATE TABLE IF NOT EXISTS fault_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id INTEGER,
                timestamp_ms INTEGER,
                fault_type TEXT,
                fault_params TEXT,
                outcome TEXT
            );
            
            CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id INTEGER,
                timestamp_ms INTEGER,
                can_id INTEGER,
                data BLOB
            );
            "#
        )?;
        
        Ok(Self { conn })
    }
    
    pub fn log_message(&self, session_id: i64, timestamp_ms: i64, can_id: u32, data: &[u8]) -> Result<()> {
        self.conn.execute(
            "INSERT INTO messages (session_id, timestamp_ms, can_id, data) VALUES (?, ?, ?, ?)",
            (session_id, timestamp_ms, can_id, data)
        )?;
        Ok(())
    }
    
    pub fn log_fault(&self, session_id: i64, timestamp_ms: i64, fault_type: &str, outcome: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO fault_events (session_id, timestamp_ms, fault_type, fault_params, outcome) VALUES (?, ?, ?, ?, ?)",
            (session_id, timestamp_ms, fault_type, "", outcome)
        )?;
        Ok(())
    }
}