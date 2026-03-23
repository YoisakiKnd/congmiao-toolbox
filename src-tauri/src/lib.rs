use std::sync::Mutex;
use sysinfo::System;

mod peek_server;
mod usage_tracker;

struct AppState {
    sys: Mutex<System>,
}

#[derive(serde::Serialize)]
struct SystemStats {
    cpu: f32,
    memory_used: u64,
    memory_total: u64,
}

#[tauri::command]
fn get_system_stats(state: tauri::State<'_, AppState>) -> SystemStats {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu_usage();
    sys.refresh_memory();
    SystemStats {
        cpu: sys.global_cpu_usage(),
        memory_used: sys.used_memory() / 1024 / 1024,
        memory_total: sys.total_memory() / 1024 / 1024,
    }
}

#[tauri::command]
async fn start_peek_server() -> Result<String, String> {
    peek_server::run_server().await;
    let my_local_ip = local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string());
    Ok(format!("http://{}:3000", my_local_ip))
}

#[tauri::command]
async fn stop_peek_server() -> Result<(), String> {
    peek_server::stop_server();
    Ok(())
}

#[tauri::command]
fn get_peek_status() -> bool {
    peek_server::IS_RUNNING.load(std::sync::atomic::Ordering::SeqCst)
}

#[tauri::command]
fn toggle_privacy() -> bool {
    let current = peek_server::PRIVACY_MODE.load(std::sync::atomic::Ordering::SeqCst);
    peek_server::PRIVACY_MODE.store(!current, std::sync::atomic::Ordering::SeqCst);
    !current
}

#[tauri::command]
fn get_privacy_status() -> bool {
    peek_server::PRIVACY_MODE.load(std::sync::atomic::Ordering::SeqCst)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut sys = System::new_all();
    sys.refresh_cpu_usage();
    
    tauri::Builder::default()
        .setup(|app| {
            usage_tracker::init(app.handle().clone());
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            sys: Mutex::new(sys),
        })
        .invoke_handler(tauri::generate_handler![
            get_system_stats,
            start_peek_server,
            stop_peek_server,
            get_peek_status,
            toggle_privacy,
            get_privacy_status,
            usage_tracker::get_app_usage
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
