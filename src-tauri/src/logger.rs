use chrono::Local;
use log::{LevelFilter, info, warn};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

static LOG_DIRECTORY: Mutex<Option<PathBuf>> = Mutex::new(None);

pub fn setup_logger(app: &AppHandle) -> Result<(), fern::InitError> {
    // 获取日志目录（可能是自定义的，也可能是默认的）
    let log_dir = get_effective_log_directory(app);

    // 创建日志目录
    if let Err(e) = fs::create_dir_all(&log_dir) {
        eprintln!("Failed to create log directory: {}", e);
        // 如果自定义目录创建失败，回退到默认目录
        let default_dir = get_default_log_directory(app);
        fs::create_dir_all(&default_dir).expect("Failed to create default log directory");

        // 更新为默认目录
        {
            let mut guard = LOG_DIRECTORY.lock().unwrap();
            *guard = None; // 清除自定义设置
        }

        warn!("日志目录创建失败，使用默认目录: {:?}", default_dir);
    } else {
        info!("日志目录: {:?}", log_dir);
    }

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
        .level_for("tauri", LevelFilter::Info) // Tauri 框架日志
        .chain(std::io::stdout()) // 同时输出到控制台
        .chain(fern::log_file(&log_file)?) // 输出到文件
        .apply()?;

    info!("CodeForge 应用启动");
    info!("应用版本: {}", env!("CARGO_PKG_VERSION"));
    info!("日志文件: {:?}", log_file);

    Ok(())
}

// 获取有效的日志目录（考虑自定义设置）
fn get_effective_log_directory(app: &AppHandle) -> PathBuf {
    // 首先检查是否有自定义日志目录
    let custom_dir = {
        let guard = LOG_DIRECTORY.lock().unwrap();
        guard.clone()
    };

    match custom_dir {
        Some(dir) => {
            info!("使用自定义日志目录: {:?}", dir);
            dir
        }
        None => {
            let default_dir = get_default_log_directory(app);
            info!("使用默认日志目录: {:?}", default_dir);
            default_dir
        }
    }
}

// 获取默认日志目录
fn get_default_log_directory(app: &AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .join("logs")
}

// 公共函数，供其他模块调用
pub fn get_log_directory(app: &AppHandle) -> PathBuf {
    get_effective_log_directory(app)
}

// 内部函数，用于配置系统调用（不保存到配置文件）
pub fn set_log_directory_internal(path: String) -> Result<(), String> {
    let new_path = PathBuf::from(&path);
    info!("设置日志 -> 内部设置日志目录为: {}", path);

    // 验证目录是否存在，如果不存在则创建
    if !new_path.exists() {
        std::fs::create_dir_all(&new_path).map_err(|e| format!("无法创建目录: {}", e))?;
    }

    // 验证是否为目录
    if !new_path.is_dir() {
        return Err("指定的路径不是目录".to_string());
    }

    // 更新全局日志目录
    {
        let mut guard = LOG_DIRECTORY.lock().unwrap();
        *guard = Some(new_path);
    }

    info!("设置日志 -> 日志目录已内部设置为: {}", path);
    Ok(())
}

// 重新初始化日志系统（当日志目录改变时调用）
pub fn reinit_logger(app: &AppHandle) -> Result<(), String> {
    // 重新设置日志系统
    setup_logger(app).map_err(|e| format!("重新初始化日志系统失败: {}", e))?;
    info!("日志系统已重新初始化");
    Ok(())
}
