use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct LuaPlugin;

impl LanguagePlugin for LuaPlugin {
    fn get_order(&self) -> i32 {
        26
    }

    fn get_language_name(&self) -> &'static str {
        "Lua"
    }

    fn get_language_key(&self) -> &'static str {
        "lua"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "lua".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["-v"]
    }

    fn get_path_command(&self) -> String {
        "which lua".to_string()
    }

    fn get_command(
        &self,
        _file_path: Option<&str>,
        _is_version: bool,
        _file_name: Option<String>,
    ) -> String {
        if _is_version {
            let lua_command = if self.get_execute_home().is_some() {
                "./lua"
            } else {
                "lua"
            };

            return lua_command.to_string();
        }

        // 执行代码时
        if let Some(config) = self.get_config() {
            if let Some(run_cmd) = &config.run_command {
                return if let Some(file_name) = _file_name {
                    run_cmd.replace("$filename", &file_name)
                } else {
                    run_cmd.clone()
                };
            }
        }
        self.get_default_command()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("lua"),
            before_compile: None,
            extension: String::from("lua"),
            execute_home: None,
            run_command: Some(String::from("lua $filename")),
            after_compile: None,
            template: Some(String::from("-- Lua 示例代码 - CodeForge 代码执行环境\n\n")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "lua".to_string())
    }
}
