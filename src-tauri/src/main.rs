#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod example;
mod execution;
mod font;
mod logger;
mod plugin;
mod plugins;
mod setup;
mod update;
mod utils;

use crate::execution::{
    ExecutionHistory, PluginManagerState as ExecutionPluginManagerState, clear_execution_history,
    execute_code, get_execution_history, is_execution_running, stop_execution,
};
use crate::plugin::{get_info, get_supported_languages};
use crate::setup::app::get_app_info;
use crate::utils::logger::{
    clear_logs, get_log_directory, get_log_files, reset_log_directory, set_log_directory,
};
use config::{get_app_config, get_config_path, init_config, update_app_config};
use example::load_example;
use font::open_font_picker;
use log::info;
use plugins::PluginManager;
use update::{check_for_updates, start_update};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .manage(ExecutionHistory::default())
        .manage(ExecutionPluginManagerState::new(PluginManager::new()))
        .setup(|app| {
            // 第一步：初始化配置系统
            if let Err(e) = init_config(Some(app.handle())) {
                eprintln!("Failed to initialize config: {}", e);
            }

            // 第二步：初始化日志系统
            if let Err(e) = logger::setup_logger(app.handle()) {
                eprintln!("Failed to setup logger: {}", e);
            }

            // 初始化应用菜单
            info!("初始化 -> 初始化应用菜单");
            let menu = setup::menu::create_menu(app.handle())?;
            app.set_menu(menu)?;
            setup::menu::setup_menu_handler(app.handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 执行相关命令
            execute_code,
            stop_execution,
            is_execution_running,
            get_execution_history,
            clear_execution_history,
            // 信息相关命令
            get_info,
            get_supported_languages,
            // 应用信息命令
            get_app_info,
            // 日志相关命令
            get_log_directory,
            set_log_directory,
            reset_log_directory,
            get_log_files,
            clear_logs,
            // 配置相关命令
            get_app_config,
            update_app_config,
            get_config_path,
            // 更新相关命令
            check_for_updates,
            start_update,
            load_example,
            open_font_picker
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
