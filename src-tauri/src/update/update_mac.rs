use log::info;

pub async fn install(
    update_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("安装更新 -> macOS 更新安装: {}", update_path.display());

    // 移除隔离属性
    remove_quarantine(update_path).await?;

    // 挂载 DMG
    let mount_point = mount_dmg(update_path).await?;

    // 安装应用
    install_from_mount_point(&mount_point).await
}

async fn remove_quarantine(
    update_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("安装更新 -> 移除隔离属性");

    let _ = std::process::Command::new("xattr")
        .args(["-d", "com.apple.quarantine"])
        .arg(update_path)
        .output();

    Ok(())
}

async fn mount_dmg(
    update_path: &std::path::Path,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    info!("安装更新 -> 挂载 DMG");

    let mount_output = std::process::Command::new("hdiutil")
        .args([
            "attach",
            "-plist",
            "-nobrowse",
            &update_path.to_string_lossy(),
        ])
        .output()?;

    if !mount_output.status.success() {
        return Err(format!(
            "安装更新 -> 无法挂载 DMG: {}",
            String::from_utf8_lossy(&mount_output.stderr)
        )
        .into());
    }

    let mount_info = String::from_utf8_lossy(&mount_output.stdout);
    info!("安装更新 -> DMG 挂载信息: {}", mount_info);

    // 解析挂载点
    let mount_point = parse_mount_point(&mount_info, update_path).await?;
    info!("安装更新 -> DMG 挂载点: {}", mount_point);

    // 验证挂载点
    verify_mount_point(&mount_point).await
}

async fn parse_mount_point(
    mount_info: &str,
    update_path: &std::path::Path,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // 解析 plist 输出找到挂载点
    if let Some(volumes_start) = mount_info.find("/Volumes/") {
        let after_volumes = &mount_info[volumes_start..];
        let line_end = after_volumes.find('\n').unwrap_or(after_volumes.len());
        let mount_path = after_volumes[..line_end].trim();

        // 移除可能的 XML 标签
        let clean_path = mount_path
            .replace("<string>", "")
            .replace("</string>", "")
            .trim()
            .to_string();

        if clean_path.starts_with("/Volumes/") {
            return Ok(clean_path);
        }
    }

    // 备用方案：尝试不使用 -plist 参数
    info!("安装更新 -> 尝试备用挂载方式");
    let mount_output2 = std::process::Command::new("hdiutil")
        .args([
            "attach",
            "-quiet",
            "-nobrowse",
            &update_path.to_string_lossy(),
        ])
        .output()?;

    if !mount_output2.status.success() {
        return Err(format!(
            "安装更新 -> 备用挂载方式失败: {}",
            String::from_utf8_lossy(&mount_output2.stderr)
        )
        .into());
    }

    let mount_info2 = String::from_utf8_lossy(&mount_output2.stdout);
    info!("安装更新 -> 备用挂载信息: {}", mount_info2);

    // 使用制表符分割方式解析
    mount_info2
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 3 && parts[2].starts_with("/Volumes/") {
                Some(parts[2].trim().to_string())
            } else {
                // 尝试空格分割
                let parts: Vec<&str> = line.split_whitespace().collect();
                parts
                    .iter()
                    .find(|part| part.starts_with("/Volumes/"))
                    .map(|s| s.to_string())
            }
        })
        .next()
        .ok_or("安装更新 -> 无法找到 DMG 挂载点".into())
}

async fn verify_mount_point(
    mount_point: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mount_path = std::path::Path::new(mount_point);

    if mount_path.exists() {
        return Ok(mount_point.to_string());
    }

    // 如果挂载点不存在，搜索 /Volumes/ 目录
    info!("安装更新 -> 挂载点不存在，搜索 /Volumes/ 目录");
    let volumes_dir = std::path::Path::new("/Volumes");

    if let Ok(entries) = std::fs::read_dir(volumes_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let dir_name = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("");

                if dir_name.contains("CodeForge")
                    || dir_name.contains("codeforge")
                    || path.join("CodeForge.app").exists()
                {
                    info!("安装更新 -> 找到可能的挂载点: {}", path.display());
                    return Ok(path.to_string_lossy().to_string());
                }
            }
        }
    }

    Err(format!(
        "安装更新 -> 挂载点 {} 不存在且未找到替代挂载点",
        mount_point
    )
    .into())
}

async fn install_from_mount_point(
    mount_point: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mount_path = std::path::Path::new(mount_point);

    // 查找 .app 文件
    let app_file = std::fs::read_dir(mount_path)?
        .filter_map(|entry| entry.ok())
        .find(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("app"))
        .ok_or("安装更新 -> 在 DMG 中找不到 .app 文件")?;

    let app_path = app_file.path();
    info!("安装更新 -> 找到应用: {}", app_path.display());

    // 获取当前应用路径
    let current_app = get_current_app_path()?;
    info!("安装更新 -> 当前应用路径: {}", current_app.display());

    // 创建并执行更新脚本
    create_and_run_update_script(&app_path, &current_app, mount_path).await
}

fn get_current_app_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    let current_exe = std::env::current_exe()?;
    info!("安装更新 -> 当前可执行文件路径: {}", current_exe.display());

    // macOS 路径结构: /path/to/CodeForge.app/Contents/MacOS/CodeForge
    let current_app = current_exe
        .parent() // MacOS
        .and_then(|p| p.parent()) // Contents
        .and_then(|p| p.parent()) // CodeForge.app
        .ok_or("安装更新 -> 无法找到当前应用的 .app 包")?;

    Ok(current_app.to_path_buf())
}

async fn create_and_run_update_script(
    new_app_path: &std::path::Path,
    current_app_path: &std::path::Path,
    mount_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let script_content = format!(
        r#"#!/bin/bash
set -e
echo "安装更新 -> 等待应用退出..."
sleep 3

echo "安装更新 -> 备份当前应用..."
if [ -d "{}.backup" ]; then
    rm -rf "{}.backup"
fi
mv "{}" "{}.backup"

echo "安装更新 -> 复制新应用..."
cp -R "{}" "{}"

echo "安装更新 -> 移除隔离属性..."
xattr -dr com.apple.quarantine "{}" || true

echo "安装更新 -> 设置权限..."
chmod -R 755 "{}"

echo "安装更新 -> 清理挂载..."
hdiutil detach "{}" -quiet || true

echo "安装更新 -> 启动新应用..."
open "{}"

echo "安装更新 -> 清理脚本..."
rm "$0"
"#,
        current_app_path.display(), // backup check
        current_app_path.display(), // source for backup
        current_app_path.display(), // target for backup
        current_app_path.display(), // backup name
        new_app_path.display(),     // source for copy
        current_app_path.display(), // target for copy
        current_app_path.display(), // target for xattr
        current_app_path.display(), // target for chmod
        mount_path.display(),       // volume to detach
        current_app_path.display()  // app to open
    );

    let script_path = std::env::temp_dir().join("codeforge_update.sh");
    std::fs::write(&script_path, script_content)?;

    // 设置脚本可执行权限
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&script_path, perms)?;
    }

    info!("安装更新 -> 启动更新脚本: {}", script_path.display());

    // 启动更新脚本
    std::process::Command::new(&script_path).spawn()?;

    // 等待脚本启动后退出应用
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    info!("安装更新 -> 退出当前应用以完成更新");
    std::process::exit(0);
}
