// src-tauri/src/db.rs

use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

// ============================================================================
// ESTRUTURAS
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Assembler {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub document: Option<String>,
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
            document TEXT,
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
    document: Option<String>,
    address: Option<String>,
) -> Result<Assembler, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    let created_at = chrono::Utc::now().to_rfc3339();
    
    conn.execute(
        "INSERT INTO assemblers (name, phone, document, address, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![&name, &phone, &document, &address, &created_at],
    )
    .map_err(|e| e.to_string())?;
    
    let id = conn.last_insert_rowid();
    
    Ok(Assembler {
        id: id.to_string(),
        name,
        phone,
        document,
        address,
        created_at,
    })
}

#[tauri::command]
pub fn get_all_assemblers(state: State<DbConnection>) -> Result<Vec<Assembler>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    let mut stmt = conn
        .prepare("SELECT id, name, phone, document, address, created_at FROM assemblers ORDER BY name")
        .map_err(|e| e.to_string())?;
    
    let assemblers = stmt
        .query_map([], |row| {
            Ok(Assembler {
                id: row.get::<_, i64>(0)?.to_string(),
                name: row.get(1)?,
                phone: row.get(2)?,
                document: row.get(3)?,
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
        .prepare("SELECT id, name, phone, document, address, created_at FROM assemblers WHERE id = ?1")
        .map_err(|e| e.to_string())?;
    
    let assembler = stmt
        .query_row([id_int], |row| {
            Ok(Assembler {
                id: row.get::<_, i64>(0)?.to_string(),
                name: row.get(1)?,
                phone: row.get(2)?,
                document: row.get(3)?,
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
    document: Option<String>,
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
        "UPDATE assemblers SET name = ?1, phone = ?2, document = ?3, address = ?4 WHERE id = ?5",
        rusqlite::params![&name, &phone, &document, &address, id_int],
    )
    .map_err(|e| e.to_string())?;
    
    Ok(Assembler {
        id,
        name,
        phone,
        document,
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
        .prepare("
            SELECT 
                a.id, 
                a.assembler_id, 
                assembler.name as assembler_name, 
                a.order_number, 
                a.order_value, 
                a.percentage_paid, 
                a.amount_paid, 
                a.furniture_description, 
                a.date, 
                a.created_at 
            FROM assemblies a
            JOIN assemblers assembler ON a.assembler_id = assembler.id
            ORDER BY a.date DESC
        ")
        .map_err(|e| e.to_string())?;
    
    let assemblies = stmt
        .query_map([], |row| {
            Ok(Assembly {
                id: row.get::<_, i64>(0)?.to_string(),
                assembler_id: row.get::<_, i64>(1)?.to_string(),
                assembler_name: row.get(2)?,  // Agora vem do JOIN
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
        .prepare("
            SELECT a.id, a.assembler_id, assembler.name AS assembler_name, 
                   a.order_number, a.order_value, a.percentage_paid, a.amount_paid, 
                   a.furniture_description, a.date, a.created_at 
            FROM assemblies a 
            JOIN assemblers assembler ON a.assembler_id = assembler.id 
            WHERE a.id = ?1
        ")
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

// ============================================================================
// BACKUP
// ============================================================================

#[tauri::command]
pub fn create_backup(app_handle: tauri::AppHandle) -> Result<String, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    
    let db_path = app_dir.join("database.db");
    
    if !db_path.exists() {
        return Err("Banco de dados não encontrado".to_string());
    }
    
    // Criar pasta de backups
    let backup_dir = app_dir.join("backups");
    fs::create_dir_all(&backup_dir).map_err(|e| e.to_string())?;
    
    // Nome do arquivo com timestamp
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let backup_filename = format!("backup_{}.db", timestamp);
    let backup_path = backup_dir.join(&backup_filename);
    
    // Copiar arquivo
    fs::copy(&db_path, &backup_path).map_err(|e| e.to_string())?;
    
    Ok(backup_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn list_backups(app_handle: tauri::AppHandle) -> Result<Vec<BackupInfo>, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    
    let backup_dir = app_dir.join("backups");
    
    if !backup_dir.exists() {
        return Ok(vec![]);
    }
    
    let mut backups = Vec::new();
    
    for entry in fs::read_dir(backup_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("db") {
            let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;
            let size = metadata.len();
            let created = metadata
                .created()
                .map_err(|e| e.to_string())?;
            
            let created_timestamp = created
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|e| e.to_string())?
                .as_secs();
            
            backups.push(BackupInfo {
                filename: path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string(),
                path: path.to_string_lossy().to_string(),
                size,
                created_at: created_timestamp,
            });
        }
    }
    
    // Ordenar por data (mais recente primeiro)
    backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    
    Ok(backups)
}

#[tauri::command]
pub fn restore_backup(
    app_handle: tauri::AppHandle,
    backup_path: String,
) -> Result<String, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    
    let db_path = app_dir.join("database.db");
    let backup = PathBuf::from(&backup_path);
    
    if !backup.exists() {
        return Err("Backup não encontrado".to_string());
    }
    
    // Fazer backup do banco atual antes de restaurar
    let safety_backup = app_dir.join("database_before_restore.db");
    if db_path.exists() {
        fs::copy(&db_path, &safety_backup).map_err(|e| e.to_string())?;
    }
    
    // Restaurar backup
    fs::copy(&backup, &db_path).map_err(|e| e.to_string())?;
    
    Ok("Backup restaurado com sucesso".to_string())
}

#[tauri::command]
pub fn delete_backup(backup_path: String) -> Result<bool, String> {
    let backup = PathBuf::from(&backup_path);
    
    if !backup.exists() {
        return Err("Backup não encontrado".to_string());
    }
    
    fs::remove_file(&backup).map_err(|e| e.to_string())?;
    
    Ok(true)
}

#[tauri::command]
pub fn export_backup_to_location(
    app_handle: tauri::AppHandle,
    destination: String,
) -> Result<String, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    
    let db_path = app_dir.join("database.db");
    let dest_path = PathBuf::from(&destination);
    
    if !db_path.exists() {
        return Err("Banco de dados não encontrado".to_string());
    }
    
    fs::copy(&db_path, &dest_path).map_err(|e| e.to_string())?;
    
    Ok(dest_path.to_string_lossy().to_string())
}

#[derive(Debug, Serialize, Deserialize)]

pub struct BackupInfo {
    pub filename: String,
    pub path: String,
    pub size: u64,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
}

#[tauri::command]
pub fn import_backup_from_location(
    app_handle: tauri::AppHandle,
    source: String,
) -> Result<String, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    
    let source_path = PathBuf::from(&source);
    
    if !source_path.exists() {
        return Err("Arquivo de backup não encontrado".to_string());
    }
    
    // Verificar se é um arquivo .db válido
    if source_path.extension().and_then(|s| s.to_str()) != Some("db") {
        return Err("Arquivo inválido. Selecione um arquivo .db".to_string());
    }
    
    // Criar pasta de backups
    let backup_dir = app_dir.join("backups");
    fs::create_dir_all(&backup_dir).map_err(|e| e.to_string())?;
    
    // Copiar para a pasta de backups com timestamp
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let backup_filename = format!("imported_{}.db", timestamp);
    let backup_path = backup_dir.join(&backup_filename);
    
    fs::copy(&source_path, &backup_path).map_err(|e| e.to_string())?;
    
    Ok(format!("Backup importado: {}", backup_filename))
}

#[tauri::command]
pub fn create_auto_backup(app_handle: tauri::AppHandle) -> Result<String, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    
    let db_path = app_dir.join("database.db");
    
    if !db_path.exists() {
        return Err("Banco de dados não encontrado".to_string());
    }
    
    let backup_dir = app_dir.join("backups");
    fs::create_dir_all(&backup_dir).map_err(|e| e.to_string())?;
    
    // Verificar se já existe backup de hoje
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let mut has_today_backup = false;
    
    if let Ok(entries) = fs::read_dir(&backup_dir) {
        for entry in entries.flatten() {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.starts_with("backup_") && filename.contains(&today) {
                    has_today_backup = true;
                    break;
                }
            }
        }
    }
    
    // Se já tem backup de hoje, não cria outro
    if has_today_backup {
        return Ok("Backup de hoje já existe".to_string());
    }
    
    // Criar novo backup
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let backup_filename = format!("backup_{}.db", timestamp);
    let backup_path = backup_dir.join(&backup_filename);
    
    fs::copy(&db_path, &backup_path).map_err(|e| e.to_string())?;
    
    // Limpar backups antigos (manter apenas os 2 mais recentes)
    cleanup_old_backups(&backup_dir, 2)?;
    
    Ok(backup_filename)
}

fn cleanup_old_backups(backup_dir: &PathBuf, keep_count: usize) -> Result<(), String> {
    let mut backups: Vec<(PathBuf, std::time::SystemTime)> = Vec::new();
    
    // Coletar todos os backups com suas datas de criação
    if let Ok(entries) = fs::read_dir(backup_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("db") {
                if let Ok(metadata) = fs::metadata(&path) {
                    if let Ok(created) = metadata.created() {
                        backups.push((path, created));
                    }
                }
            }
        }
    }
    
    // Ordenar por data (mais recente primeiro)
    backups.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Deletar os backups excedentes
    if backups.len() > keep_count {
        for (path, _) in backups.iter().skip(keep_count) {
            let _ = fs::remove_file(path);
        }
    }
    
    Ok(())
}
