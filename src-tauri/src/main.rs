#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod ai;
mod ai_history;
mod cache;
mod config;
mod custom_plugin_commands;
mod db;
mod db_connections;
mod env_commands;
mod env_manager;
mod env_providers;
mod example;
mod execution;
mod filesystem;
mod font;
mod geo;
mod kv;
mod logger;
mod lsp;
mod plugin;
mod plugins;
mod setup;
mod snippets;
mod terminal;
mod update;
mod utils;

use crate::ai::{ai_chat, ai_chat_stream, stop_ai_stream};
use crate::ai_history::{
    AiHistory, delete_ai_conversation, get_ai_conversation, list_ai_conversation_ids,
    save_ai_conversation,
};
use crate::cache::{clear_all_cache, clear_plugins_cache, get_cache_info};
use crate::custom_plugin_commands::{
    add_custom_plugin, get_custom_plugins, remove_custom_plugin, save_custom_icon,
    update_custom_plugin,
};
use crate::db::{run_sql, run_sql_paged};
use crate::db_connections::{
    DbConnStore, db_connection_delete, db_connection_save, db_connections_list,
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
    create_directory, create_file, delete_path, get_text_file_meta, git_blame, git_branch_create,
    git_branch_delete, git_branch_rename, git_branches, git_checkout, git_checkout_track,
    git_cherry_pick, git_clean, git_clean_preview, git_clone, git_commit, git_compare,
    git_delete_remote_branch, git_diff, git_discard, git_fetch, git_file_head, git_get_identity,
    git_ignore_add, git_init, git_log, git_log_file, git_merge, git_op_abort, git_op_continue,
    git_op_skip, git_op_state, git_pull, git_pull_rebase, git_push, git_push_force, git_push_tags,
    git_reflog, git_remote_add, git_remote_branches, git_remote_remove, git_remotes, git_reset,
    git_restore_file, git_revert, git_set_identity, git_set_upstream, git_show, git_stage,
    git_stash_apply, git_stash_drop, git_stash_list, git_stash_pop, git_stash_push, git_stash_show,
    git_status, git_tag_create, git_tag_delete, git_tags, git_unstage, list_files,
    read_directory_tree, read_file_lines, read_file_text, rename_path, replace_in_files,
    reveal_path, search_in_files, watch_directory, write_file_text,
};
use crate::kv::{KvStore, kv_delete, kv_get_all, kv_set};
use crate::lsp::{
    LspState, lsp_available, lsp_install, lsp_send, lsp_server_list, lsp_start, lsp_stop,
};
use crate::plugin::{get_info, get_supported_languages};
use crate::setup::app::get_app_info;
use crate::snippets::{Snippets, delete_snippet, get_snippets, save_snippet};
use crate::terminal::{
    TerminalState, terminal_create, terminal_kill, terminal_resize, terminal_write,
};
use crate::utils::logger::{
    clear_logs, get_log_directory, get_log_files, reset_log_directory, set_log_directory,
};
use config::{get_app_config, get_config_path, init_config, update_app_config};
use example::load_example;
use font::open_font_picker;
use geo::fetch_area_geojson;
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
        .manage(AiHistory::new().expect("failed to initialize ai history database"))
        .manage(Snippets::new().expect("failed to initialize snippets database"))
        .manage(KvStore::new().expect("failed to initialize kv store database"))
        .manage(DbConnStore::new().expect("failed to initialize db connections database"))
        .manage(TerminalState::new())
        .manage(LspState::new())
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
            fetch_area_geojson,
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
            search_in_files,
            replace_in_files,
            git_diff,
            git_status,
            git_stage,
            git_unstage,
            git_discard,
            git_commit,
            git_push,
            git_pull,
            git_fetch,
            git_pull_rebase,
            git_push_force,
            git_push_tags,
            git_delete_remote_branch,
            git_branches,
            git_checkout,
            git_branch_create,
            git_branch_delete,
            git_branch_rename,
            git_remote_branches,
            git_checkout_track,
            git_merge,
            git_blame,
            git_file_head,
            git_log,
            git_log_file,
            git_show,
            git_revert,
            git_reset,
            git_restore_file,
            git_reflog,
            git_cherry_pick,
            git_compare,
            git_op_state,
            git_op_abort,
            git_op_continue,
            git_op_skip,
            git_tags,
            git_tag_create,
            git_tag_delete,
            git_get_identity,
            git_set_identity,
            git_remotes,
            git_remote_add,
            git_remote_remove,
            git_set_upstream,
            git_init,
            git_ignore_add,
            git_clone,
            git_clean_preview,
            git_clean,
            git_stash_list,
            git_stash_push,
            git_stash_pop,
            git_stash_drop,
            git_stash_apply,
            git_stash_show,
            // AI 助手
            ai_chat,
            ai_chat_stream,
            stop_ai_stream,
            // AI 对话历史（绑定执行记录）
            save_ai_conversation,
            list_ai_conversation_ids,
            get_ai_conversation,
            delete_ai_conversation,
            // 代码片段
            get_snippets,
            save_snippet,
            delete_snippet,
            // 通用键值存储（替代 localStorage）
            kv_get_all,
            kv_set,
            kv_delete,
            // 数据库连接（独立表）
            db_connections_list,
            db_connection_save,
            db_connection_delete,
            // 集成终端
            terminal_create,
            terminal_write,
            terminal_resize,
            terminal_kill,
            // SQL 执行
            run_sql,
            run_sql_paged,
            // LSP 桥接
            lsp_available,
            lsp_start,
            lsp_send,
            lsp_stop,
            lsp_server_list,
            lsp_install
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
