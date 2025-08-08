use super::{python::PythonPlugin, LanguagePlugin};
use std::collections::HashMap;

pub struct PluginManager {
    plugins: HashMap<String, Box<dyn LanguagePlugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        let mut plugins: HashMap<String, Box<dyn LanguagePlugin>> = HashMap::new();

        plugins.insert("python".to_string(), Box::new(PythonPlugin));

        Self { plugins }
    }

    pub fn get_plugin(&self, language: &str) -> Option<&Box<dyn LanguagePlugin>> {
        self.plugins.get(language)
    }

    pub fn get_supported_languages(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }

    pub fn register_plugin(&mut self, language: String, plugin: Box<dyn LanguagePlugin>) {
        self.plugins.insert(language, plugin);
    }

    pub fn unregister_plugin(&mut self, language: &str) -> Option<Box<dyn LanguagePlugin>> {
        self.plugins.remove(language)
    }

    pub fn is_language_supported(&self, language: &str) -> bool {
        self.plugins.contains_key(language)
    }

    pub fn get_plugin_info(&self, language: &str) -> Option<PluginInfo> {
        self.get_plugin(language).map(|plugin| PluginInfo {
            name: plugin.get_language_name().to_string(),
            file_extension: plugin.get_file_extension().to_string(),
            available_commands: plugin
                .get_commands()
                .iter()
                .map(|s| s.to_string())
                .collect(),
        })
    }

    pub fn get_all_plugin_info(&self) -> Vec<PluginInfo> {
        self.plugins
            .values()
            .map(|plugin| PluginInfo {
                name: plugin.get_language_name().to_string(),
                file_extension: plugin.get_file_extension().to_string(),
                available_commands: plugin
                    .get_commands()
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            })
            .collect()
    }
}

#[derive(Debug, serde::Serialize)]
pub struct PluginInfo {
    pub name: String,
    pub file_extension: String,
    pub available_commands: Vec<String>,
}
