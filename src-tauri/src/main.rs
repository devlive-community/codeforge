#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod ai;
mod cache;
mod config;
mod custom_plugin_commands;
mod env_commands;
mod env_manager;
mod env_providers;
mod example;
mod execution;
mod filesystem;
mod font;
mod logger;
mod plugin;
mod plugins;
mod setup;
mod update;
mod utils;

use crate::ai::{ai_chat, ai_chat_stream};
use crate::cache::{clear_all_cache, clear_plugins_cache, get_cache_info};
use crate::custom_plugin_commands::{
    add_custom_plugin, get_custom_plugins, remove_custom_plugin, save_custom_icon,
    update_custom_plugin,
};
use crate::env_commands::{
    EnvironmentManagerState, download_and_install_version, get_environment_info,
    get_supported_environment_languages, switch_environment_version, uninstall_environment_version,
};
use crate::env_manager::EnvironmentManager;
use crate::env_providers::{
    ClojureEnvironmentProvider, GoEnvironmentProvider, JavaEnvironmentProvider,
    PhpEnvironmentProvider, RustEnvironmentProvider, ScalaEnvironmentProvider,
};
use crate::execution::{
    ExecutionHistory, PluginManagerState as ExecutionPluginManagerState, clear_execution_history,
    execute_code, get_execution_history, get_execution_history_page, is_execution_running,
    stop_execution,
};
use crate::filesystem::{
    create_directory, create_file, delete_path, get_text_file_meta, list_files,
    read_directory_tree, read_file_lines, read_file_text, rename_path, reveal_path,
    watch_directory, write_file_text,
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
    // 设置系统环境变量
    let _ = fix_path_env::fix();

    // 初始化环境管理器
    let mut env_manager = EnvironmentManager::new();
    env_manager.register_provider(Box::new(ClojureEnvironmentProvider::new()));
    env_manager.register_provider(Box::new(GoEnvironmentProvider::new()));
    env_manager.register_provider(Box::new(JavaEnvironmentProvider::new()));
    env_manager.register_provider(Box::new(PhpEnvironmentProvider::new()));
    env_manager.register_provider(Box::new(RustEnvironmentProvider::new()));
    env_manager.register_provider(Box::new(ScalaEnvironmentProvider::new()));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .manage(ExecutionHistory::new().expect("failed to initialize execution history database"))
        .manage(ExecutionPluginManagerState::new(PluginManager::new()))
        .manage(EnvironmentManagerState::new(env_manager))
        .setup(|app| {
            // 第一步：初始化配置系统
            if let Err(e) = init_config(Some(app.handle())) {
                eprintln!("Failed to initialize config: {}", e);
            }

            // 第二步：加载自定义插件
            use tauri::Manager;
            if let Some(plugin_manager_state) = app.try_state::<ExecutionPluginManagerState>() {
                if let Ok(mut manager) = plugin_manager_state.try_lock() {
                    if let Ok(app_config) = config::get_app_config_internal() {
                        if let Some(custom_plugins) = app_config.custom_plugins {
                            manager.load_custom_plugins(custom_plugins);
                            info!("初始化 -> 已加载自定义插件");
                        }
                    }
                }
            }

            // 第三步：初始化日志系统
            if let Err(e) = logger::setup_logger(app.handle()) {
                eprintln!("Failed to setup logger: {}", e);
            }

            // 初始化应用菜单
            info!("初始化 -> 初始化应用菜单");
            let menu = setup::menu::create_menu(app.handle())?;
            app.set_menu(menu)?;
            setup::menu::setup_menu_handler(app.handle());

            info!(
                "初始化 -> 系统环境变量 {:?}",
                std::env::var(String::from("PATH")).unwrap_or(String::from(""))
            );
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 执行相关命令
            execute_code,
            stop_execution,
            is_execution_running,
            get_execution_history,
            get_execution_history_page,
            clear_execution_history,
            // 信息相关命令
            get_info,
            get_supported_languages,
            // 环境管理相关命令
            get_environment_info,
            download_and_install_version,
            switch_environment_version,
            uninstall_environment_version,
            get_supported_environment_languages,
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
            // 自定义插件相关命令
            add_custom_plugin,
            update_custom_plugin,
            remove_custom_plugin,
            get_custom_plugins,
            save_custom_icon,
            // 缓存相关命令
            get_cache_info,
            clear_plugins_cache,
            clear_all_cache,
            // 更新相关命令
            check_for_updates,
            start_update,
            load_example,
            open_font_picker,
            // 文件系统相关命令
            read_directory_tree,
            read_file_text,
            write_file_text,
            get_text_file_meta,
            read_file_lines,
            create_file,
            create_directory,
            rename_path,
            delete_path,
            reveal_path,
            watch_directory,
            list_files,
            // AI 助手
            ai_chat,
            ai_chat_stream
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
