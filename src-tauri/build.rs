fn main() {
    // 设置构建时间
    let build_time = chrono::Utc::now()
        .format("%Y-%m-%d %H:%M:%S UTC")
        .to_string();
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);

    // 重新构建触发条件
    println!("cargo:rerun-if-changed=build.rs");

    // Tauri 的构建脚本
    tauri_build::build()
}
