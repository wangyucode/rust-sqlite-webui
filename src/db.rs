use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<serde_json::Value>>,
}

pub fn get_tables(db_path: &str) -> Result<Vec<String>> {
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'")?;
    let rows = stmt.query_map([], |row| row.get(0))?;
    
    let mut tables = Vec::new();
    for table in rows {
        tables.push(table?);
    }
    Ok(tables)
}

pub fn execute_query(db_path: &str, sql: &str) -> Result<QueryResult> {
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare(sql)?;
    
    let column_names: Vec<String> = stmt
        .column_names()
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    let rows = stmt.query_map([], |row| {
        let mut row_values = Vec::new();
        for i in 0..column_names.len() {
            let val: rusqlite::types::Value = row.get(i)?;
            row_values.push(sqlite_value_to_json(val));
        }
        Ok(row_values)
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }

    Ok(QueryResult {
        columns: column_names,
        rows: results,
    })
}

pub fn execute_update(db_path: &str, sql: &str) -> Result<usize> {
    let conn = Connection::open(db_path)?;
    conn.execute(sql, [])
}

fn sqlite_value_to_json(val: rusqlite::types::Value) -> serde_json::Value {
    match val {
        rusqlite::types::Value::Null => serde_json::Value::Null,
        rusqlite::types::Value::Integer(i) => serde_json::Value::Number(i.into()),
        rusqlite::types::Value::Real(f) => {
            if let Some(num) = serde_json::Number::from_f64(f) {
                serde_json::Value::Number(num)
            } else {
                serde_json::Value::Null
            }
        }
        rusqlite::types::Value::Text(s) => serde_json::Value::String(s),
        rusqlite::types::Value::Blob(b) => serde_json::Value::String(format!("blob: {} bytes", b.len())),
    }
}
