use log::info;

pub async fn install(
    update_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("安装更新 -> Linux 更新安装: {}", update_path.display());

    let current_exe = std::env::current_exe()?;
    info!("安装更新 -> 当前可执行文件: {}", current_exe.display());

    // 设置更新文件权限
    set_executable_permission(update_path)?;

    // 创建备份
    let backup_path = create_backup(&current_exe)?;

    // 替换可执行文件
    match replace_executable(update_path, &current_exe) {
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
            return Err(e);
        }
    }

    info!("安装更新 -> Linux 安装完成");
    Ok(())
}

fn set_executable_permission(
    update_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("安装更新 -> 设置更新文件可执行权限");

    let _ = std::process::Command::new("chmod")
        .args(["+x"])
        .arg(update_path)
        .output();

    Ok(())
}

fn create_backup(
    current_exe: &std::path::Path,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    let backup_path = current_exe.with_extension("backup");
    info!("安装更新 -> 创建备份: {}", backup_path.display());

    std::fs::copy(current_exe, &backup_path)?;
    Ok(backup_path)
}

fn replace_executable(
    update_path: &std::path::Path,
    current_exe: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("安装更新 -> 替换可执行文件");

    // 复制新文件
    std::fs::copy(update_path, current_exe)?;

    // 设置可执行权限
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(current_exe)?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(current_exe, perms)?;
    info!("安装更新 -> 可执行文件权限设置完成");

    Ok(())
}
