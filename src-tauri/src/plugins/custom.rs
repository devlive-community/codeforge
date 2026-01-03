use super::{LanguagePlugin, PluginConfig};

pub struct CustomPlugin {
    config: PluginConfig,
}

impl CustomPlugin {
    pub fn new(config: PluginConfig) -> Self {
        Self { config }
    }
}

impl LanguagePlugin for CustomPlugin {
    fn get_language_name(&self) -> &'static str {
        Box::leak(self.config.language.clone().into_boxed_str())
    }

    fn get_language_key(&self) -> &'static str {
        Box::leak(self.config.language.clone().into_boxed_str())
    }

    fn get_file_extension(&self) -> String {
        self.config.extension.clone()
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        // 从 run_command 中提取第一个命令(如 "bash $filename" -> "bash")
        self.config
            .run_command
            .as_ref()
            .and_then(|cmd| cmd.split_whitespace().next())
            .map(|s| format!("which {}", s))
            .unwrap_or_else(|| "echo unknown".to_string())
    }

    fn get_command(
        &self,
        file_path: Option<&str>,
        is_version: bool,
        _file_name: Option<String>,
    ) -> String {
        // 当获取版本信息时或没有file_path时,返回 run_command 的第一个命令
        if is_version || file_path.is_none() {
            return self
                .config
                .run_command
                .as_ref()
                .and_then(|cmd| cmd.split_whitespace().next())
                .map(|s| s.to_string())
                .unwrap_or_else(|| self.config.language.clone());
        }

        // 执行代码时,返回完整的命令
        if let Some(run_cmd) = &self.config.run_command {
            if let Some(path) = file_path {
                return run_cmd.replace("$filename", path);
            }
        }

        self.get_default_command()
    }

    fn get_default_config(&self) -> PluginConfig {
        self.config.clone()
    }

    fn get_default_command(&self) -> String {
        self.config
            .run_command
            .clone()
            .unwrap_or_else(|| self.config.language.clone())
    }
}
