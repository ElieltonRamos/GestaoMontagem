// // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     tauri::Builder::default()
//         .plugin(tauri_plugin_opener::init())
//         .invoke_handler(tauri::generate_handler![greet])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

// src-tauri/src/lib.rs

mod db; // Importa o módulo de banco de dados

use rusqlite::Connection;
use tauri::Manager;

// Comando de exemplo existente
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Configuração do banco de dados
            let app_dir = app.path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            
            std::fs::create_dir_all(&app_dir)
                .expect("Failed to create app data dir");
            
            let db_path = app_dir.join("database.db");
            println!("Database path: {:?}", db_path);
            
            let conn = Connection::open(db_path)
                .expect("Failed to connect to database");
            
            db::init_db(&conn)
                .expect("Failed to initialize database");
            
            app.manage(db::DbConnection(std::sync::Mutex::new(conn)));
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,              // Comando existente
            // Assemblers
            db::create_assembler,
            db::get_all_assemblers,
            db::get_assembler_by_id,
            db::update_assembler,
            db::delete_assembler,
            db::assembler_exists_by_name_except_id,
            db::assembler_exists_by_name,
            // Assemblies
            db::create_assembly,
            db::get_all_assemblies,
            db::get_assembly_by_id,
            db::update_assembly,
            db::delete_assembly,
            db::assembly_exists_by_order_number,
            db::assembly_exists_by_order_number_except_id,
            //stats
            db::get_dashboard_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}