use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct RustPlugin;

impl LanguagePlugin for RustPlugin {
    fn get_order(&self) -> i32 {
        7
    }

    fn get_language_name(&self) -> &'static str {
        "Rust"
    }

    fn get_language_key(&self) -> &'static str {
        "rust"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "rs".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        if let Some(execute_home) = self.get_execute_home() {
            let rustc_bin = std::path::Path::new(&execute_home)
                .join("rustc")
                .join("bin");

            if rustc_bin.exists() {
                format!("{}/rustc --print sysroot", rustc_bin.display())
            } else {
                let bin_dir = std::path::Path::new(&execute_home).join("bin");
                format!("{}/rustc --print sysroot", bin_dir.display())
            }
        } else {
            "rustc --print sysroot".to_string()
        }
    }

    fn get_execute_args(&self, file_path: &str) -> Vec<String> {
        if let Some(config) = self.get_config() {
            if let Some(run_cmd) = &config.run_command {
                let full_cmd = run_cmd.replace("$filename", file_path);

                // Windows 使用 cmd /c，Unix 使用 sh -c
                #[cfg(target_os = "windows")]
                return vec!["/c".to_string(), full_cmd];

                #[cfg(not(target_os = "windows"))]
                return vec!["-c".to_string(), full_cmd];
            }
        }

        // 默认命令
        #[cfg(target_os = "windows")]
        return vec![
            "/c".to_string(),
            format!("rustc {} -o main.exe && main.exe", file_path),
        ];

        #[cfg(not(target_os = "windows"))]
        vec![
            "-c".to_string(),
            format!("rustc {} -o /tmp/main && /tmp/main", file_path),
        ]
    }

    fn get_command(
        &self,
        _file_path: Option<&str>,
        _is_version: bool,
        _file_name: Option<String>,
    ) -> String {
        // Windows 使用 cmd，Unix 使用 sh
        #[cfg(target_os = "windows")]
        return "cmd".to_string();

        #[cfg(not(target_os = "windows"))]
        "sh".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("rust"),
            before_compile: None,
            extension: String::from("rs"),
            execute_home: None,
            run_command: Some(String::from("bash")),
            after_compile: Some(String::from("rm -f /tmp/main")),
            template: Some(String::from("# 在这里输入 Rust 代码")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "/tmp/main".to_string())
    }
}
