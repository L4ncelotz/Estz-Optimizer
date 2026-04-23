mod action_store;
mod bloatware;
mod hardware_validator;
mod optimizer;
mod powershell;
mod process_manager;
mod registry;
mod restore_point;
mod system_info;

use action_store::ActionStore;
use std::sync::Mutex;
use tauri::State;

struct AppState {
    action_store: Mutex<ActionStore>,
    sys: Mutex<sysinfo::System>,
}

// === SYSTEM INFO COMMANDS ===

#[tauri::command]
fn get_system_overview() -> system_info::SystemOverview {
    system_info::get_system_overview()
}

#[tauri::command]
async fn get_live_metrics(state: State<'_, AppState>) -> Result<system_info::LiveMetrics, String> {
    let mut sys = state.sys.lock().map_err(|e| e.to_string())?;
    Ok(system_info::get_live_metrics(&mut sys))
}

// === HARDWARE COMMANDS ===

#[tauri::command]
fn get_hardware_info() -> hardware_validator::HardwareInfo {
    hardware_validator::detect_hardware()
}

#[tauri::command]
fn validate_tweak(tweak_id: String) -> hardware_validator::ValidationResult {
    let hw = hardware_validator::detect_hardware();
    hardware_validator::validate_for_tweak(&tweak_id, &hw)
}

// === OPTIMIZER COMMANDS ===

#[tauri::command]
fn get_optimization_catalog() -> Vec<optimizer::OptimizationItem> {
    optimizer::get_catalog()
}

#[tauri::command]
fn get_safe_optimizations() -> Vec<String> {
    optimizer::get_safe_optimization_ids()
}

#[tauri::command]
fn apply_optimization(id: String, state: State<AppState>) -> optimizer::OptimizationResult {
    let mut store = state.action_store.lock().unwrap();
    optimizer::apply(&id, &mut store)
}

#[tauri::command]
fn revert_optimization(id: String, state: State<AppState>) -> optimizer::OptimizationResult {
    let mut store = state.action_store.lock().unwrap();
    optimizer::revert(&id, &mut store)
}

#[tauri::command]
fn detect_optimization_state(id: String) -> bool {
    optimizer::detect_state(&id)
}

// === RESTORE POINT COMMANDS ===

#[tauri::command]
fn create_restore_point(description: String) -> restore_point::RestorePointResult {
    restore_point::create_restore_point(&description)
}

#[tauri::command]
fn get_latest_restore_point() -> Option<String> {
    restore_point::get_latest_restore_point()
}

// === ACTION HISTORY COMMANDS ===

#[tauri::command]
fn get_action_history(state: State<AppState>) -> Vec<action_store::ActionRecord> {
    let store = state.action_store.lock().unwrap();
    store.get_history()
}

#[tauri::command]
fn clear_action_history(state: State<AppState>) {
    let mut store = state.action_store.lock().unwrap();
    store.clear_history();
}

// === PROCESS MANAGER COMMANDS ===

#[tauri::command]
async fn list_processes() -> Vec<process_manager::ProcessInfo> {
    process_manager::list_processes()
}

#[tauri::command]
fn get_cpu_topology() -> process_manager::CpuTopology {
    process_manager::get_cpu_topology()
}

#[tauri::command]
fn set_process_affinity(pid: u32, mask: u64) -> Result<String, String> {
    process_manager::set_process_affinity(pid, mask)
}

#[tauri::command]
fn pin_to_p_cores(pid: u32) -> Result<String, String> {
    process_manager::pin_to_p_cores(pid)
}

#[tauri::command]
fn pin_to_e_cores(pid: u32) -> Result<String, String> {
    process_manager::pin_to_e_cores(pid)
}

#[tauri::command]
fn reset_affinity(pid: u32) -> Result<String, String> {
    process_manager::reset_affinity(pid)
}

// === BLOATWARE COMMANDS ===

#[tauri::command]
async fn scan_bloatware() -> Vec<bloatware::BloatwareApp> {
    bloatware::scan_bloatware()
}

#[tauri::command]
fn remove_bloatware(packages: Vec<String>) -> Vec<(String, bool)> {
    bloatware::remove_packages(packages)
}

// === POWERSHELL COMMANDS ===

#[tauri::command]
fn get_power_plan() -> String {
    powershell::get_active_power_plan()
}

#[tauri::command]
fn execute_powershell(script: String) -> powershell::PsOutput {
    powershell::execute(&script)
}

// === APP ENTRY ===

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    
    let app_state = AppState {
        action_store: Mutex::new(ActionStore::load()),
        sys: Mutex::new(sys),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // System
            get_system_overview,
            get_live_metrics,
            get_hardware_info,
            validate_tweak,
            get_power_plan,
            // Optimization
            get_optimization_catalog,
            get_safe_optimizations,
            apply_optimization,
            revert_optimization,
            detect_optimization_state,
            // Restore Point
            create_restore_point,
            get_latest_restore_point,
            // History
            get_action_history,
            clear_action_history,
            // Process Manager
            list_processes,
            get_cpu_topology,
            set_process_affinity,
            pin_to_p_cores,
            pin_to_e_cores,
            reset_affinity,
            // Bloatware
            scan_bloatware,
            remove_bloatware,
            // PowerShell
            execute_powershell,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Estz");
}
