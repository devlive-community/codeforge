use super::{LanguagePlugin, PluginConfig};
use crate::plugins::applescript::AppleScriptPlugin;
use crate::plugins::c::CPlugin;
use crate::plugins::cangjie::CangjiePlugin;
use crate::plugins::clojure::ClojurePlugin;
use crate::plugins::cpp::CppPlugin;
use crate::plugins::css::CssPlugin;
use crate::plugins::go::GoPlugin;
use crate::plugins::groovy::GroovyPlugin;
use crate::plugins::html::HtmlPlugin;
use crate::plugins::java::JavaPlugin;
use crate::plugins::javascript_browser::JavaScriptBrowserPlugin;
use crate::plugins::javascript_jquery::JavaScriptJQueryPlugin;
use crate::plugins::javascript_nodejs::JavaScriptNodeJsPlugin;
use crate::plugins::kotlin::KotlinPlugin;
use crate::plugins::nodejs::NodeJSPlugin;
use crate::plugins::php::PHPPlugin;
use crate::plugins::python2::Python2Plugin;
use crate::plugins::python3::Python3Plugin;
use crate::plugins::r::RPlugin;
use crate::plugins::ruby::RubyPlugin;
use crate::plugins::rust::RustPlugin;
use crate::plugins::scala::ScalaPlugin;
use crate::plugins::shell::ShellPlugin;
use crate::plugins::svg::SvgPlugin;
use crate::plugins::swift::SwiftPlugin;
use crate::plugins::typescript::TypeScriptPlugin;
use crate::plugins::typescript_browser::TypeScriptBrowserPlugin;
use crate::plugins::typescript_nodejs::TypeScriptNodeJsPlugin;
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
        plugins.insert("swift".to_string(), Box::new(SwiftPlugin));
        plugins.insert("scala".to_string(), Box::new(ScalaPlugin));
        plugins.insert("kotlin".to_string(), Box::new(KotlinPlugin));
        plugins.insert("clojure".to_string(), Box::new(ClojurePlugin));
        plugins.insert("c".to_string(), Box::new(CPlugin));
        plugins.insert("ruby".to_string(), Box::new(RubyPlugin));
        plugins.insert("applescript".to_string(), Box::new(AppleScriptPlugin));
        plugins.insert("typescript".to_string(), Box::new(TypeScriptPlugin));
        plugins.insert("cpp".to_string(), Box::new(CppPlugin));
        plugins.insert("groovy".to_string(), Box::new(GroovyPlugin));
        plugins.insert("html".to_string(), Box::new(HtmlPlugin));
        plugins.insert("css".to_string(), Box::new(CssPlugin));
        plugins.insert("svg".to_string(), Box::new(SvgPlugin));
        plugins.insert("php".to_string(), Box::new(PHPPlugin));
        plugins.insert("r".to_string(), Box::new(RPlugin));
        plugins.insert("cangjie".to_string(), Box::new(CangjiePlugin));
        plugins.insert(
            "javascript-nodejs".to_string(),
            Box::new(JavaScriptNodeJsPlugin),
        );
        plugins.insert(
            "typescript-nodejs".to_string(),
            Box::new(TypeScriptNodeJsPlugin),
        );
        plugins.insert(
            "typescript-browser".to_string(),
            Box::new(TypeScriptBrowserPlugin),
        );
        plugins.insert(
            "javascript-browser".to_string(),
            Box::new(JavaScriptBrowserPlugin),
        );
        plugins.insert(
            "javascript-jquery".to_string(),
            Box::new(JavaScriptJQueryPlugin),
        );

        Self { plugins }
    }

    pub fn get_plugin(&self, language: &str) -> Option<&dyn LanguagePlugin> {
        self.plugins.get(language).map(|plugin| plugin.as_ref())
    }

    pub fn get_supported_languages(&self) -> Vec<serde_json::Value> {
        let mut plugins: Vec<_> = self.plugins.iter().collect();
        plugins.sort_by_key(|(_, plugin)| plugin.get_language_key());

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
            available_commands: vec![plugin.get_command(None, true, None).to_string()],
        })
    }

    #[allow(dead_code)]
    pub fn get_all_plugin_info(&self) -> Vec<PluginInfo> {
        self.plugins
            .values()
            .map(|plugin| PluginInfo {
                name: plugin.get_language_name().to_string(),
                file_extension: plugin.get_file_extension(),
                available_commands: vec![plugin.get_command(None, true, None).to_string()],
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
