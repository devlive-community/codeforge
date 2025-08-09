use chrono::Local;
use log::LevelFilter;
use std::fs;
use tauri::{AppHandle, Manager};

pub fn setup_logger(app: &AppHandle) -> Result<(), fern::InitError> {
    // 获取应用数据目录
    let app_data_dir = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir");

    // 创建日志目录
    let log_dir = app_data_dir.join("logs");
    fs::create_dir_all(&log_dir).expect("Failed to create log directory");

    // 生成当天的日志文件名
    let today = Local::now().format("%Y-%m-%d").to_string();
    let log_file = log_dir.join(format!("codeforge-{}.log", today));

    // 配置日志
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] [{}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                message
            ))
        })
        .level(LevelFilter::Debug) // 设置日志级别
        .level_for("hyper", LevelFilter::Warn) // 减少第三方库的日志
        .level_for("reqwest", LevelFilter::Warn)
        .chain(std::io::stdout()) // 同时输出到控制台
        .chain(fern::log_file(&log_file)?) // 输出到文件
        .apply()?;

    log::info!("日志系统初始化完成");
    log::info!("日志文件: {:?}", log_file);

    Ok(())
}
