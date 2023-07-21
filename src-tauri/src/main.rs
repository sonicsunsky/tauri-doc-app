// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::Manager;

mod db;
use db::Database;
use db::Doc;
use db::User;

mod menu;
mod tray;

struct AppState {
    db: Mutex<Database>,
}

impl AppState {
    pub fn new(db: Mutex<Database>) -> Self {
        Self { db }
    }
}

fn main() {
    let db = Database::new("./docs.db").expect("Unable to create database connection");
    let app_state = AppState::new(Mutex::new(db));
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                // window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_doc,
            add_user,
            update_doc,
            update_user,
            delete_doc,
            delete_user,
            get_all_docs,
            get_docs_by_stmt,
            get_all_users,
            get_user_by_name
        ])
        .manage(app_state)
        .menu(menu::init(&context))
        .on_menu_event(menu::handler)
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .run(context)
        .expect("error while running tauri application");
}

#[tauri::command]
fn update_doc(state: tauri::State<AppState>, id: i32, name: &str) -> Result<(), String> {
    state
        .db
        .lock()
        .unwrap()
        .update_doc(id, name)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn update_user(state: tauri::State<AppState>, id: i32, user_name: &str) -> Result<(), String> {
    state
        .db
        .lock()
        .unwrap()
        .update_user(id, user_name)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn get_all_users(state: tauri::State<AppState>) -> Result<Vec<User>, String> {
    state
        .db
        .lock()
        .unwrap()
        .get_all_users()
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn get_all_docs(state: tauri::State<AppState>) -> Result<Vec<Doc>, String> {
    state
        .db
        .lock()
        .unwrap()
        .get_all_docs()
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn get_docs_by_stmt(state: tauri::State<AppState>, stmt: &str) -> Result<Vec<Doc>, String> {
    state
        .db
        .lock()
        .unwrap()
        .get_docs_by_stmt(stmt)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn delete_doc(state: tauri::State<AppState>, id: i32) -> Result<(), String> {
    state
        .db
        .lock()
        .unwrap()
        .delete_doc(id)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn delete_user(state: tauri::State<AppState>, id: i32) -> Result<(), String> {
    state
        .db
        .lock()
        .unwrap()
        .delete_user(id)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn add_doc(state: tauri::State<AppState>, doc: Doc) -> Result<(), String> {
    state
        .db
        .lock()
        .unwrap()
        .add_doc(&doc)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn add_user(state: tauri::State<AppState>, user: User) -> Result<(), String> {
    state
        .db
        .lock()
        .unwrap()
        .add_user(&user)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn get_user_by_name(state: tauri::State<AppState>, name: String) -> Result<Vec<User>, String> {
    state
        .db
        .lock()
        .unwrap()
        .get_user_by_name(name)
        .map_err(|err| err.to_string())
}
