use crate::plugins::{CodeExecutionRequest, ExecutionResult, PluginManager};
use log::{error, info, warn};
use rusqlite::{Connection, params};
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex as StdMutex, OnceLock, mpsc};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex;

// 执行任务结构
#[derive(Debug)]
pub struct ExecutionTask {
    #[allow(dead_code)]
    pub language: String,
    #[allow(dead_code)]
    pub process_id: u32,
    pub stop_flag: Arc<Mutex<bool>>,
}

pub type PluginManagerState = Mutex<PluginManager>;

#[derive(Debug, Serialize)]
pub struct ExecutionHistoryPage {
    pub items: Vec<ExecutionResult>,
    pub total: u64,
}

pub struct ExecutionHistory {
    conn: StdMutex<Connection>,
}

impl ExecutionHistory {
    pub fn new() -> Result<Self, String> {
        let db_path = get_codeforge_db_path()?;
        let conn =
            Connection::open(&db_path).map_err(|e| format!("打开执行历史数据库失败: {}", e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS execution_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                success INTEGER NOT NULL,
                code TEXT NOT NULL,
                stdout TEXT NOT NULL,
                stderr TEXT NOT NULL,
                execution_time INTEGER NOT NULL,
                timestamp INTEGER NOT NULL,
                language TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| format!("初始化执行历史数据库失败: {}", e))?;

        Ok(Self {
            conn: StdMutex::new(conn),
        })
    }

    fn insert(&self, result: &ExecutionResult) -> Result<(), String> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| "执行历史数据库锁错误".to_string())?;

        conn.execute(
            "INSERT INTO execution_history
                (success, code, stdout, stderr, execution_time, timestamp, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                if result.success { 1 } else { 0 },
                &result.code,
                &result.stdout,
                &result.stderr,
                result.execution_time as i64,
                result.timestamp as i64,
                &result.language,
            ],
        )
        .map_err(|e| format!("保存执行历史失败: {}", e))?;

        Ok(())
    }

    fn list(&self) -> Result<Vec<ExecutionResult>, String> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| "执行历史数据库锁错误".to_string())?;
        let mut statement = conn
            .prepare(
                "SELECT success, code, stdout, stderr, execution_time, timestamp, language
                 FROM execution_history
                 ORDER BY id ASC",
            )
            .map_err(|e| format!("读取执行历史失败: {}", e))?;

        let rows = statement
            .query_map([], |row| {
                Ok(ExecutionResult {
                    success: row.get::<_, i64>(0)? != 0,
                    code: row.get(1)?,
                    stdout: row.get(2)?,
                    stderr: row.get(3)?,
                    execution_time: row.get::<_, i64>(4)? as u128,
                    timestamp: row.get::<_, i64>(5)? as u64,
                    language: row.get(6)?,
                })
            })
            .map_err(|e| format!("读取执行历史失败: {}", e))?;

        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("读取执行历史失败: {}", e))
    }

    fn list_page(&self, offset: u64, limit: u64) -> Result<ExecutionHistoryPage, String> {
        let limit = limit.clamp(1, 100);
        let conn = self
            .conn
            .lock()
            .map_err(|_| "执行历史数据库锁错误".to_string())?;

        let total = conn
            .query_row("SELECT COUNT(*) FROM execution_history", [], |row| {
                row.get::<_, i64>(0)
            })
            .map_err(|e| format!("统计执行历史失败: {}", e))? as u64;

        let mut statement = conn
            .prepare(
                "SELECT success, code, stdout, stderr, execution_time, timestamp, language
                 FROM execution_history
                 ORDER BY id DESC
                 LIMIT ?1 OFFSET ?2",
            )
            .map_err(|e| format!("读取执行历史失败: {}", e))?;

        let rows = statement
            .query_map(params![limit as i64, offset as i64], |row| {
                Ok(ExecutionResult {
                    success: row.get::<_, i64>(0)? != 0,
                    code: row.get(1)?,
                    stdout: row.get(2)?,
                    stderr: row.get(3)?,
                    execution_time: row.get::<_, i64>(4)? as u128,
                    timestamp: row.get::<_, i64>(5)? as u64,
                    language: row.get(6)?,
                })
            })
            .map_err(|e| format!("读取执行历史失败: {}", e))?;

        let items = rows
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("读取执行历史失败: {}", e))?;

        Ok(ExecutionHistoryPage { items, total })
    }

    fn clear(&self) -> Result<(), String> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| "执行历史数据库锁错误".to_string())?;
        conn.execute("DELETE FROM execution_history", [])
            .map_err(|e| format!("清空执行历史失败: {}", e))?;
        Ok(())
    }
}

// 全局任务管理器
type TaskManager = Arc<Mutex<HashMap<String, ExecutionTask>>>;
static TASK_MANAGER: OnceLock<TaskManager> = OnceLock::new();

// 初始化任务管理器
fn init_task_manager() -> TaskManager {
    TASK_MANAGER
        .get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
        .clone()
}

// 获取 .codeforge 缓存目录
fn get_codeforge_cache_dir(language: &str) -> Result<PathBuf, String> {
    let home_dir = dirs::home_dir().ok_or("无法获取用户主目录")?;
    let cache_dir = home_dir
        .join(".codeforge")
        .join("cache")
        .join("plugins")
        .join(language);

    // 确保目录存在
    fs::create_dir_all(&cache_dir).map_err(|e| format!("创建缓存目录失败: {}", e))?;

    Ok(cache_dir)
}

fn get_codeforge_db_path() -> Result<PathBuf, String> {
    let home_dir = dirs::home_dir().ok_or("无法获取用户主目录")?;
    let codeforge_dir = home_dir.join(".codeforge");
    fs::create_dir_all(&codeforge_dir).map_err(|e| format!("创建配置目录失败: {}", e))?;
    let db_path = codeforge_dir.join("codeforge.sqlite");
    let old_db_path = codeforge_dir.join("execution_history.sqlite");

    if !db_path.exists() && old_db_path.exists() {
        fs::rename(&old_db_path, &db_path).map_err(|e| format!("迁移执行历史数据库失败: {}", e))?;
    }

    Ok(db_path)
}

// 检查是否应该过滤 stderr 行
fn should_filter_stderr_line(language: &str, line: &str) -> bool {
    match language {
        "clojure" => {
            // 过滤 Clojure 的常见警告信息
            line.contains("WARNING: Implicit use of clojure.main")
                || line.contains("WARNING: name already refers to:")
        }
        "scala" => {
            // 过滤 Scala 的编译信息
            line.contains("[0m[0m[33mCompiling project")
                || line.contains("[0m[0m")
                || line.starts_with("\u{001b}")
        }
        _ => false,
    }
}

// 停止执行命令（按 task_id 停止指定的运行任务）
#[tauri::command]
pub async fn stop_execution(task_id: String) -> Result<bool, String> {
    let task_manager = init_task_manager();
    let mut guard = task_manager.lock().await;

    if let Some(task) = guard.remove(&task_id) {
        // 设置停止标志
        {
            let mut stop_flag = task.stop_flag.lock().await;
            *stop_flag = true;
        }
        info!("停止执行 -> 成功设置停止标志给任务 [ {} ]", task_id);
        Ok(true)
    } else {
        warn!("停止执行 -> 任务 [ {} ] 没有正在运行", task_id);
        Ok(false)
    }
}

// 检查指定任务是否正在运行
#[tauri::command]
pub async fn is_execution_running(task_id: String) -> Result<bool, String> {
    let task_manager = init_task_manager();
    let guard = task_manager.lock().await;
    Ok(guard.contains_key(&task_id))
}

// 通用的代码执行函数
#[tauri::command]
pub async fn execute_code(
    request: CodeExecutionRequest,
    history: State<'_, ExecutionHistory>,
    plugin_manager: State<'_, PluginManagerState>,
    app: AppHandle,
) -> Result<ExecutionResult, String> {
    let task_id = request.task_id.clone();
    info!(
        "执行代码 -> 调用插件 [ {} ] 任务 [ {} ] 开始",
        request.language, task_id
    );

    let manager = plugin_manager.lock().await;
    let plugin = manager
        .get_plugin(&request.language)
        .ok_or_else(|| format!("Unsupported language: {}", request.language))?;

    // 决定运行的文件与工作目录：
    // - 提供了 file_path：就地运行该文件，工作目录为其所在目录（多文件 import/相对路径正确）
    // - 否则：写入 .codeforge/cache/plugins/<language> 临时目录后运行
    let (file_path, cwd): (PathBuf, Option<PathBuf>) = if let Some(fp) = request.file_path.clone() {
        let p = PathBuf::from(&fp);
        let dir = p.parent().map(|d| d.to_path_buf());
        (p, dir)
    } else {
        let temp_dir = get_codeforge_cache_dir(&request.language)?;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let file_work = format!("Codeforge_{}_{}", request.language, timestamp);
        let work_dir = temp_dir.join(&file_work);
        fs::create_dir_all(&work_dir).map_err(|e| format!("创建工作目录失败: {}", e))?;
        let file_name = format!("{}.{}", file_work, plugin.get_file_extension());
        let fp = work_dir.join(&file_name);

        // 写入代码到临时文件
        fs::write(&fp, &request.code)
            .map_err(|e| format!("Failed to write temporary file: {}", e))?;

        let home = plugin.get_execute_home().map(PathBuf::from);
        (fp, home)
    };

    let _processed_code = plugin
        .pre_execute_hook(&request.code, file_path.to_str().unwrap())
        .map_err(|e| {
            error!(
                "执行代码 -> 调用插件 [ {} ] pre_execute_hook 出现错误 {:?}",
                request.language, e
            );
            format!("Pre-execution hook failed: {}", e)
        })?;

    let start_time = std::time::Instant::now();

    let cmd = plugin.get_command(None, false, Some(file_path.to_string_lossy().to_string()));
    let args = plugin.get_execute_args(file_path.to_str().unwrap());
    info!(
        "执行代码 -> 调用插件 [ {} ] 执行命令 {} 携带参数 {}",
        request.language,
        cmd,
        args.join(" ")
    );

    // 发送执行开始事件
    let _ = app.emit(
        "code-execution-start",
        serde_json::json!({
            "language": request.language,
            "task_id": task_id
        }),
    );

    // 启动子进程
    let mut command = Command::new(&cmd);
    command
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // 设置工作目录（就地运行为文件目录，否则为插件 execute_home）
    if let Some(dir) = &cwd {
        command.current_dir(dir);
    }

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(e) => {
            let _execution_time = start_time.elapsed().as_millis();
            let _timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            // let _ = fs::remove_file(&file_path);
            let _ = app.emit(
                "code-execution-complete",
                serde_json::json!({
                    "language": request.language,
                    "task_id": task_id,
                    "success": false
                }),
            );

            error!("执行代码 -> 调用插件 [ {} ] 失败: {}", request.language, e);
            return Err(format!(
                "{} interpreter not found. Please install {} and ensure it's in your PATH.\n\nError: {}",
                request.language, request.language, e
            ));
        }
    };

    // 创建停止标志
    let stop_flag = Arc::new(tokio::sync::Mutex::new(false));

    // 将任务添加到管理器
    let task_manager = init_task_manager();
    {
        let mut guard = task_manager.lock().await;
        guard.insert(
            task_id.clone(),
            ExecutionTask {
                language: request.language.clone(),
                process_id: child.id(),
                stop_flag: stop_flag.clone(),
            },
        );
    }

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let (stdout_tx, stdout_rx) = mpsc::channel::<String>();
    let (stderr_tx, stderr_rx) = mpsc::channel::<String>();

    // 读取 stdout
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
            if stdout_tx.send(line).is_err() {
                break;
            }
        }
    });

    // 读取 stderr
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines().map_while(Result::ok) {
            if stderr_tx.send(line).is_err() {
                break;
            }
        }
    });

    let mut stdout_lines = Vec::new();
    let mut stderr_lines = Vec::new();
    let timeout = std::time::Duration::from_secs(plugin.get_timeout());

    // 主执行循环
    loop {
        // 检查停止标志
        {
            let stop_guard = stop_flag.lock().await;
            if *stop_guard {
                info!(
                    "执行代码 -> 收到停止信号，终止语言 [ {} ] 的执行",
                    request.language
                );
                let _ = child.kill();
                let _ = child.wait();
                // let _ = fs::remove_file(&file_path);

                // 从任务管理器中移除
                {
                    let mut guard = task_manager.lock().await;
                    guard.remove(&task_id);
                }

                let _ = app.emit(
                    "code-execution-stopped",
                    serde_json::json!({
                        "language": request.language,
                        "task_id": task_id
                    }),
                );

                return Err("代码执行被用户停止".to_string());
            }
        }

        // 检查超时
        if start_time.elapsed() > timeout {
            let _ = child.kill();
            let _ = child.wait();
            // let _ = fs::remove_file(&file_path);

            // 从任务管理器中移除
            {
                let mut guard = task_manager.lock().await;
                guard.remove(&task_id);
            }

            let _ = app.emit(
                "code-execution-timeout",
                serde_json::json!({
                    "language": request.language,
                    "task_id": task_id
                }),
            );

            error!(
                "执行代码 -> 超时 ({} 秒)，终止语言 [ {} ] 的执行",
                plugin.get_timeout(),
                request.language
            );
            return Err(format!("代码执行超时（{} 秒）", plugin.get_timeout()));
        }

        // 读取并发送 stdout
        while let Ok(line) = stdout_rx.try_recv() {
            stdout_lines.push(line.clone());
            let _ = app.emit(
                "code-output",
                serde_json::json!({
                    "type": "stdout",
                    "content": line,
                    "language": request.language,
                    "task_id": task_id
                }),
            );
        }

        // 读取并发送 stderr
        while let Ok(line) = stderr_rx.try_recv() {
            stderr_lines.push(line.clone());
            // 过滤掉特定语言的警告信息
            if !should_filter_stderr_line(&request.language, &line) {
                let _ = app.emit(
                    "code-output",
                    serde_json::json!({
                        "type": "stderr",
                        "content": line,
                        "language": request.language,
                        "task_id": task_id
                    }),
                );
            }
        }

        // 检查进程是否结束
        match child.try_wait() {
            Ok(Some(status)) => {
                // 进程已结束，读取剩余输出
                while let Ok(line) = stdout_rx.try_recv() {
                    stdout_lines.push(line.clone());
                    let _ = app.emit(
                        "code-output",
                        serde_json::json!({
                            "type": "stdout",
                            "content": line,
                            "language": request.language,
                            "task_id": task_id
                        }),
                    );
                }
                while let Ok(line) = stderr_rx.try_recv() {
                    stderr_lines.push(line.clone());
                    // 过滤掉特定语言的警告信息
                    if !should_filter_stderr_line(&request.language, &line) {
                        let _ = app.emit(
                            "code-output",
                            serde_json::json!({
                                "type": "stderr",
                                "content": line,
                                "language": request.language,
                                "task_id": task_id
                            }),
                        );
                    }
                }

                let execution_time = start_time.elapsed().as_millis();
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                // let _ = fs::remove_file(&file_path);

                // 从任务管理器中移除
                {
                    let mut guard = task_manager.lock().await;
                    guard.remove(&task_id);
                }

                let mut result = ExecutionResult {
                    success: status.success(),
                    code: request.code.clone(),
                    stdout: stdout_lines.join("\n"),
                    stderr: stderr_lines.join("\n"),
                    execution_time,
                    timestamp,
                    language: request.language.clone(),
                };

                let _ = plugin.post_execute_hook(&mut result);

                if result.success
                    && result.stdout == *"END-NO-OUTPUT"
                    && result.stderr == *"END-NO-OUTPUT"
                {
                    let _ = app.emit(
                        "code-output",
                        serde_json::json!({
                            "type": "stdout",
                            "content": "代码执行成功 (无输出)",
                            "language": request.language,
                            "task_id": task_id
                        }),
                    );
                }

                let _ = app.emit(
                    "code-execution-complete",
                    serde_json::json!({
                        "language": request.language,
                        "task_id": task_id,
                        "success": result.success,
                        "execution_time": result.execution_time
                    }),
                );

                drop(manager);
                history.insert(&result)?;

                info!("执行代码 -> 调用插件 [ {} ] 完成", request.language);
                return Ok(result);
            }
            Ok(None) => {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
            Err(e) => {
                let _ = child.kill();
                let _ = child.wait();
                // let _ = fs::remove_file(&file_path);

                // 从任务管理器中移除
                {
                    let mut guard = task_manager.lock().await;
                    guard.remove(&task_id);
                }

                let _ = app.emit(
                    "code-execution-error",
                    serde_json::json!({
                        "language": request.language,
                        "task_id": task_id,
                        "error": e.to_string()
                    }),
                );

                return Err(format!("检查进程状态失败: {}", e));
            }
        }
    }
}

// 获取执行历史
#[tauri::command]
pub async fn get_execution_history(
    history: State<'_, ExecutionHistory>,
) -> Result<Vec<ExecutionResult>, String> {
    history.list()
}

// 分页获取执行历史
#[tauri::command]
pub async fn get_execution_history_page(
    offset: u64,
    limit: u64,
    history: State<'_, ExecutionHistory>,
) -> Result<ExecutionHistoryPage, String> {
    history.list_page(offset, limit)
}

// 清空执行历史
#[tauri::command]
pub async fn clear_execution_history(history: State<'_, ExecutionHistory>) -> Result<(), String> {
    history.clear()
}
