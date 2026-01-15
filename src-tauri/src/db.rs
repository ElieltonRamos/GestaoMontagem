// src-tauri/src/db.rs

use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

// ============================================================================
// ESTRUTURAS
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Assembler {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub cpf: Option<String>,
    pub address: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assembly {
    pub id: String,
    #[serde(rename = "assemblerId")]
    pub assembler_id: String,
    #[serde(rename = "assemblerName")]
    pub assembler_name: String,
    #[serde(rename = "orderNumber")]
    pub order_number: String,
    #[serde(rename = "orderValue")]
    pub order_value: f64,
    #[serde(rename = "percentagePaid")]
    pub percentage_paid: f64,
    #[serde(rename = "amountPaid")]
    pub amount_paid: f64,
    #[serde(rename = "furnitureDescription")]
    pub furniture_description: String,
    pub date: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

// Estado global para gerenciar a conexão do banco
pub struct DbConnection(pub Mutex<Connection>);

// ============================================================================
// INICIALIZAÇÃO
// ============================================================================

pub fn init_db(conn: &Connection) -> Result<()> {
    // Tabela de montadores
    conn.execute(
        "CREATE TABLE IF NOT EXISTS assemblers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            phone TEXT NOT NULL,
            cpf TEXT,
            address TEXT,
            created_at TEXT NOT NULL
        )",
        [],
    )?;

    // Tabela de montagens
    conn.execute(
        "CREATE TABLE IF NOT EXISTS assemblies (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            assembler_id INTEGER NOT NULL,
            assembler_name TEXT NOT NULL,
            order_number TEXT NOT NULL,
            order_value REAL NOT NULL,
            percentage_paid REAL NOT NULL,
            amount_paid REAL NOT NULL,
            furniture_description TEXT NOT NULL,
            date TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (assembler_id) REFERENCES assemblers(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Criar índice para melhor performance nas buscas
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_assemblies_assembler_id ON assemblies(assembler_id)",
        [],
    )?;

    Ok(())
}

// ============================================================================
// ASSEMBLER - CRUD
// ============================================================================

#[tauri::command]
pub fn create_assembler(
    state: State<DbConnection>,
    name: String,
    phone: String,
    cpf: Option<String>,
    address: Option<String>,
) -> Result<Assembler, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    let created_at = chrono::Utc::now().to_rfc3339();
    
    conn.execute(
        "INSERT INTO assemblers (name, phone, cpf, address, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![&name, &phone, &cpf, &address, &created_at],
    )
    .map_err(|e| e.to_string())?;
    
    let id = conn.last_insert_rowid();
    
    Ok(Assembler {
        id: id.to_string(),
        name,
        phone,
        cpf,
        address,
        created_at,
    })
}

#[tauri::command]
pub fn get_all_assemblers(state: State<DbConnection>) -> Result<Vec<Assembler>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    let mut stmt = conn
        .prepare("SELECT id, name, phone, cpf, address, created_at FROM assemblers ORDER BY name")
        .map_err(|e| e.to_string())?;
    
    let assemblers = stmt
        .query_map([], |row| {
            Ok(Assembler {
                id: row.get::<_, i64>(0)?.to_string(),
                name: row.get(1)?,
                phone: row.get(2)?,
                cpf: row.get(3)?,
                address: row.get(4)?,
                created_at: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(assemblers)
}

#[tauri::command]
pub fn get_assembler_by_id(state: State<DbConnection>, id: String) -> Result<Assembler, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let id_int: i64 = id.parse().map_err(|_| "ID inválido".to_string())?;
    
    let mut stmt = conn
        .prepare("SELECT id, name, phone, cpf, address, created_at FROM assemblers WHERE id = ?1")
        .map_err(|e| e.to_string())?;
    
    let assembler = stmt
        .query_row([id_int], |row| {
            Ok(Assembler {
                id: row.get::<_, i64>(0)?.to_string(),
                name: row.get(1)?,
                phone: row.get(2)?,
                cpf: row.get(3)?,
                address: row.get(4)?,
                created_at: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;
    
    Ok(assembler)
}

#[tauri::command]
pub fn update_assembler(
    state: State<DbConnection>,
    id: String,
    name: String,
    phone: String,
    cpf: Option<String>,
    address: Option<String>,
) -> Result<Assembler, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let id_int: i64 = id.parse().map_err(|_| "ID inválido".to_string())?;
    
    // Buscar created_at original
    let created_at: String = conn
        .query_row("SELECT created_at FROM assemblers WHERE id = ?1", [id_int], |row| {
            row.get(0)
        })
        .map_err(|e| e.to_string())?;
    
    conn.execute(
        "UPDATE assemblers SET name = ?1, phone = ?2, cpf = ?3, address = ?4 WHERE id = ?5",
        rusqlite::params![&name, &phone, &cpf, &address, id_int],
    )
    .map_err(|e| e.to_string())?;
    
    Ok(Assembler {
        id,
        name,
        phone,
        cpf,
        address,
        created_at,
    })
}

#[tauri::command]
pub fn delete_assembler(state: State<DbConnection>, id: String) -> Result<bool, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let id_int: i64 = id.parse().map_err(|_| "ID inválido".to_string())?;
    
    let rows_affected = conn
        .execute("DELETE FROM assemblers WHERE id = ?1", [id_int])
        .map_err(|e| e.to_string())?;
    
    Ok(rows_affected > 0)
}

// ============================================================================
// ASSEMBLY - CRUD
// ============================================================================

#[tauri::command]
pub fn create_assembly(
    state: State<DbConnection>,
    assembler_id: String,
    assembler_name: String,
    order_number: String,
    order_value: f64,
    percentage_paid: f64,
    amount_paid: f64,
    furniture_description: String,
    date: String,
) -> Result<Assembly, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let assembler_id_int: i64 = assembler_id.parse().map_err(|_| "ID do montador inválido".to_string())?;
    
    let created_at = chrono::Utc::now().to_rfc3339();
    
    conn.execute(
        "INSERT INTO assemblies (assembler_id, assembler_name, order_number, order_value, percentage_paid, amount_paid, furniture_description, date, created_at) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![
            assembler_id_int,
            &assembler_name,
            &order_number,
            order_value,
            percentage_paid,
            amount_paid,
            &furniture_description,
            &date,
            &created_at
        ],
    )
    .map_err(|e| e.to_string())?;
    
    let id = conn.last_insert_rowid();
    
    Ok(Assembly {
        id: id.to_string(),
        assembler_id,
        assembler_name,
        order_number,
        order_value,
        percentage_paid,
        amount_paid,
        furniture_description,
        date,
        created_at,
    })
}

#[tauri::command]
pub fn get_all_assemblies(state: State<DbConnection>) -> Result<Vec<Assembly>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    let mut stmt = conn
        .prepare("SELECT id, assembler_id, assembler_name, order_number, order_value, percentage_paid, amount_paid, furniture_description, date, created_at FROM assemblies ORDER BY date DESC")
        .map_err(|e| e.to_string())?;
    
    let assemblies = stmt
        .query_map([], |row| {
            Ok(Assembly {
                id: row.get::<_, i64>(0)?.to_string(),
                assembler_id: row.get::<_, i64>(1)?.to_string(),
                assembler_name: row.get(2)?,
                order_number: row.get(3)?,
                order_value: row.get(4)?,
                percentage_paid: row.get(5)?,
                amount_paid: row.get(6)?,
                furniture_description: row.get(7)?,
                date: row.get(8)?,
                created_at: row.get(9)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(assemblies)
}

#[tauri::command]
pub fn get_assembly_by_id(state: State<DbConnection>, id: String) -> Result<Assembly, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let id_int: i64 = id.parse().map_err(|_| "ID inválido".to_string())?;
    
    let mut stmt = conn
        .prepare("SELECT id, assembler_id, assembler_name, order_number, order_value, percentage_paid, amount_paid, furniture_description, date, created_at FROM assemblies WHERE id = ?1")
        .map_err(|e| e.to_string())?;
    
    let assembly = stmt
        .query_row([id_int], |row| {
            Ok(Assembly {
                id: row.get::<_, i64>(0)?.to_string(),
                assembler_id: row.get::<_, i64>(1)?.to_string(),
                assembler_name: row.get(2)?,
                order_number: row.get(3)?,
                order_value: row.get(4)?,
                percentage_paid: row.get(5)?,
                amount_paid: row.get(6)?,
                furniture_description: row.get(7)?,
                date: row.get(8)?,
                created_at: row.get(9)?,
            })
        })
        .map_err(|e| e.to_string())?;
    
    Ok(assembly)
}

#[tauri::command]
pub fn update_assembly(
    state: State<DbConnection>,
    id: String,
    assembler_id: String,
    assembler_name: String,
    order_number: String,
    order_value: f64,
    percentage_paid: f64,
    amount_paid: f64,
    furniture_description: String,
    date: String,
) -> Result<Assembly, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let id_int: i64 = id.parse().map_err(|_| "ID inválido".to_string())?;
    let assembler_id_int: i64 = assembler_id.parse().map_err(|_| "ID do montador inválido".to_string())?;
    
    // Buscar created_at original
    let created_at: String = conn
        .query_row("SELECT created_at FROM assemblies WHERE id = ?1", [id_int], |row| {
            row.get(0)
        })
        .map_err(|e| e.to_string())?;
    
    conn.execute(
        "UPDATE assemblies SET assembler_id = ?1, assembler_name = ?2, order_number = ?3, order_value = ?4, percentage_paid = ?5, amount_paid = ?6, furniture_description = ?7, date = ?8 WHERE id = ?9",
        rusqlite::params![
            assembler_id_int,
            &assembler_name,
            &order_number,
            order_value,
            percentage_paid,
            amount_paid,
            &furniture_description,
            &date,
            id_int
        ],
    )
    .map_err(|e| e.to_string())?;
    
    Ok(Assembly {
        id,
        assembler_id,
        assembler_name,
        order_number,
        order_value,
        percentage_paid,
        amount_paid,
        furniture_description,
        date,
        created_at,
    })
}

#[tauri::command]
pub fn delete_assembly(state: State<DbConnection>, id: String) -> Result<bool, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let id_int: i64 = id.parse().map_err(|_| "ID inválido".to_string())?;
    
    let rows_affected = conn
        .execute("DELETE FROM assemblies WHERE id = ?1", [id_int])
        .map_err(|e| e.to_string())?;
    
    Ok(rows_affected > 0)
}

// ============================================================================
// VALIDAÇÃO - ASSEMBLERS
// ============================================================================

#[tauri::command]
pub fn assembler_exists_by_name(state: State<DbConnection>, name: String) -> Result<bool, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let normalized = name.trim().to_lowercase();
    
    let exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM assemblers WHERE LOWER(TRIM(name)) = ?1)",
            [normalized],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    
    Ok(exists)
}

#[tauri::command]
pub fn assembler_exists_by_name_except_id(
    state: State<DbConnection>,
    name: String,
    exclude_id: String,
) -> Result<bool, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let normalized = name.trim().to_lowercase();
    let exclude_id_int: i64 = exclude_id.parse().map_err(|_| "ID inválido".to_string())?;
    
    let exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM assemblers WHERE LOWER(TRIM(name)) = ?1 AND id != ?2)",
            rusqlite::params![normalized, exclude_id_int],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    
    Ok(exists)
}

// ============================================================================
// VALIDAÇÃO - ASSEMBLIES
// ============================================================================

#[tauri::command]
pub fn assembly_exists_by_order_number(
    state: State<DbConnection>,
    order_number: String,
) -> Result<bool, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let normalized = order_number.trim();
    
    let exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM assemblies WHERE TRIM(order_number) = ?1)",
            [normalized],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    
    Ok(exists)
}

#[tauri::command]
pub fn assembly_exists_by_order_number_except_id(
    state: State<DbConnection>,
    order_number: String,
    exclude_id: String,
) -> Result<bool, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let normalized = order_number.trim();
    let exclude_id_int: i64 = exclude_id.parse().map_err(|_| "ID inválido".to_string())?;
    
    let exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM assemblies WHERE TRIM(order_number) = ?1 AND id != ?2)",
            rusqlite::params![normalized, exclude_id_int],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    
    Ok(exists)
}

// ============================================================================
// DASHBOARD STATS
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    #[serde(rename = "totalAssemblies")]
    pub total_assemblies: i64,
    #[serde(rename = "totalValue")]
    pub total_value: f64,
    #[serde(rename = "activeAssemblers")]
    pub active_assemblers: i64,
}

#[tauri::command]
pub fn get_dashboard_stats(state: State<DbConnection>) -> Result<DashboardStats, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    let total_assemblies: i64 = conn
        .query_row("SELECT COUNT(*) FROM assemblies", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    
    let total_value: f64 = conn
        .query_row("SELECT COALESCE(SUM(amount_paid), 0.0) FROM assemblies", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    
    let active_assemblers: i64 = conn
        .query_row("SELECT COUNT(*) FROM assemblers", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    
    Ok(DashboardStats {
        total_assemblies,
        total_value,
        active_assemblers,
    })
}
