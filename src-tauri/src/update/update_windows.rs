use log::info;

pub async fn install(
    update_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("安装更新 -> Windows 更新安装: {}", update_path.display());

    let extension = update_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    match extension {
        "msi" => install_msi(update_path).await,
        "exe" => install_exe(update_path).await,
        _ => Err("安装更新 -> 不支持的 Windows 更新文件格式".into()),
    }
}

async fn install_msi(
    update_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("安装更新 -> 使用 MSI 静默安装");

    let output = std::process::Command::new("msiexec")
        .args(["/i", &update_path.to_string_lossy(), "/quiet", "/norestart"])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "安装更新 -> MSI 安装失败: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    info!("安装更新 -> MSI 安装完成");
    Ok(())
}

async fn install_exe(
    update_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("安装更新 -> 使用 EXE 文件替换");

    let current_exe = std::env::current_exe()?;
    info!("安装更新 -> 当前可执行文件: {}", current_exe.display());

    // 创建备份
    let backup_path = current_exe.with_extension("exe.backup");
    info!("安装更新 -> 创建备份: {}", backup_path.display());
    std::fs::copy(&current_exe, &backup_path)?;

    // 尝试替换
    match std::fs::copy(update_path, &current_exe) {
        Ok(_) => {
            info!("安装更新 -> 文件替换成功");
            // 删除备份
            let _ = std::fs::remove_file(backup_path);
        }
        Err(e) => {
            info!("安装更新 -> 文件替换失败，恢复备份");
            // 恢复备份
            let _ = std::fs::copy(&backup_path, &current_exe);
            let _ = std::fs::remove_file(backup_path);
            return Err(e.into());
        }
    }

    info!("安装更新 -> EXE 安装完成");
    Ok(())
}
