fn main() {
    // 设置构建时间
    let utc_time = chrono::Utc::now();
    let beijing_time = utc_time + chrono::Duration::hours(8);
    let build_time = beijing_time.format("%Y年%m月%d日 %H:%M:%S").to_string();
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);

    // 重新构建触发条件
    println!("cargo:rerun-if-changed=build.rs");

    // Tauri 的构建脚本
    tauri_build::build()
}
