use super::{
    LanguagePlugin, PluginConfig, go::GoPlugin, nodejs::NodeJSPlugin, python2::Python2Plugin,
    python3::Python3Plugin,
};
use crate::plugins::java::JavaPlugin;
use crate::plugins::rust::RustPlugin;
use crate::plugins::shell::ShellPlugin;
use std::collections::HashMap;

pub struct PluginManager {
    plugins: HashMap<String, Box<dyn LanguagePlugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        let mut plugins: HashMap<String, Box<dyn LanguagePlugin>> = HashMap::new();

        plugins.insert("python2".to_string(), Box::new(Python2Plugin));
        plugins.insert("python3".to_string(), Box::new(Python3Plugin));
        plugins.insert("nodejs".to_string(), Box::new(NodeJSPlugin));
        plugins.insert("go".to_string(), Box::new(GoPlugin));
        plugins.insert("java".to_string(), Box::new(JavaPlugin));
        plugins.insert("shell".to_string(), Box::new(ShellPlugin));
        plugins.insert("rust".to_string(), Box::new(RustPlugin));

        Self { plugins }
    }

    pub fn get_plugin(&self, language: &str) -> Option<&dyn LanguagePlugin> {
        self.plugins.get(language).map(|plugin| plugin.as_ref())
    }

    pub fn get_supported_languages(&self) -> Vec<serde_json::Value> {
        let mut plugins: Vec<_> = self.plugins.iter().collect();
        plugins.sort_by_key(|(_, plugin)| plugin.get_order());

        plugins
            .into_iter()
            .map(|(key, plugin)| {
                serde_json::json!({
                    "name": plugin.get_language_name(),
                    "value": key
                })
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn register_plugin(&mut self, language: String, plugin: Box<dyn LanguagePlugin>) {
        self.plugins.insert(language, plugin);
    }

    #[allow(dead_code)]
    pub fn unregister_plugin(&mut self, language: &str) -> Option<Box<dyn LanguagePlugin>> {
        self.plugins.remove(language)
    }

    #[allow(dead_code)]
    pub fn is_language_supported(&self, language: &str) -> bool {
        self.plugins.contains_key(language)
    }

    #[allow(dead_code)]
    pub fn get_plugin_info(&self, language: &str) -> Option<PluginInfo> {
        self.get_plugin(language).map(|plugin| PluginInfo {
            name: plugin.get_language_name().to_string(),
            file_extension: plugin.get_file_extension(),
            available_commands: vec![plugin.get_command(None).to_string()],
        })
    }

    #[allow(dead_code)]
    pub fn get_all_plugin_info(&self) -> Vec<PluginInfo> {
        self.plugins
            .values()
            .map(|plugin| PluginInfo {
                name: plugin.get_language_name().to_string(),
                file_extension: plugin.get_file_extension(),
                available_commands: vec![plugin.get_command(None).to_string()],
            })
            .collect()
    }

    pub fn get_all_plugin_default_config(&self) -> Vec<PluginConfig> {
        self.plugins
            .values()
            .map(|plugin| plugin.get_default_config())
            .collect()
    }
}

#[derive(Debug, serde::Serialize)]
pub struct PluginInfo {
    pub name: String,
    pub file_extension: String,
    pub available_commands: Vec<String>,
}
