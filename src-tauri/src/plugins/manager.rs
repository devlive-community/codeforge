use super::{LanguagePlugin, PluginConfig};
use crate::plugins::applescript::AppleScriptPlugin;
use crate::plugins::c::CPlugin;
use crate::plugins::cangjie::CangjiePlugin;
use crate::plugins::clojure::ClojurePlugin;
use crate::plugins::cpp::CppPlugin;
use crate::plugins::css::CssPlugin;
use crate::plugins::csv::CsvPlugin;
use crate::plugins::custom::CustomPlugin;
use crate::plugins::dart::DartPlugin;
use crate::plugins::go::GoPlugin;
use crate::plugins::groovy::GroovyPlugin;
use crate::plugins::haskell::HaskellPlugin;
use crate::plugins::html::HtmlPlugin;
use crate::plugins::java::JavaPlugin;
use crate::plugins::javascript_browser::JavaScriptBrowserPlugin;
use crate::plugins::javascript_jquery::JavaScriptJQueryPlugin;
use crate::plugins::javascript_nodejs::JavaScriptNodeJsPlugin;
use crate::plugins::json::JsonPlugin;
use crate::plugins::julia::JuliaPlugin;
use crate::plugins::kotlin::KotlinPlugin;
use crate::plugins::less::LessPlugin;
use crate::plugins::lua::LuaPlugin;
use crate::plugins::markdown::MarkdownPlugin;
use crate::plugins::nodejs::NodeJSPlugin;
use crate::plugins::objective_c::ObjectiveCPlugin;
use crate::plugins::objective_cpp::ObjectiveCppPlugin;
use crate::plugins::ocaml::OCamlPlugin;
use crate::plugins::perl::PerlPlugin;
use crate::plugins::php::PHPPlugin;
use crate::plugins::powershell::PowerShellPlugin;
use crate::plugins::python2::Python2Plugin;
use crate::plugins::python3::Python3Plugin;
use crate::plugins::r::RPlugin;
use crate::plugins::react::ReactPlugin;
use crate::plugins::ruby::RubyPlugin;
use crate::plugins::rust::RustPlugin;
use crate::plugins::scala::ScalaPlugin;
use crate::plugins::shell::ShellPlugin;
use crate::plugins::sql::SqlPlugin;
use crate::plugins::svg::SvgPlugin;
use crate::plugins::swift::SwiftPlugin;
use crate::plugins::tcl::TclPlugin;
use crate::plugins::text::TextPlugin;
use crate::plugins::tsv::TsvPlugin;
use crate::plugins::typescript::TypeScriptPlugin;
use crate::plugins::typescript_browser::TypeScriptBrowserPlugin;
use crate::plugins::typescript_nodejs::TypeScriptNodeJsPlugin;
use crate::plugins::vue::VuePlugin;
use crate::plugins::xlsx::XlsxPlugin;
use crate::plugins::xml::XmlPlugin;
use crate::plugins::yaml::YamlPlugin;
use std::collections::HashMap;

pub struct PluginManager {
    plugins: HashMap<String, Box<dyn LanguagePlugin>>,
    builtin_languages: Vec<String>,
}

impl PluginManager {
    pub fn new() -> Self {
        let mut plugins: HashMap<String, Box<dyn LanguagePlugin>> = HashMap::new();
        let mut builtin_languages = Vec::new();

        let builtin_plugins: Vec<(String, Box<dyn LanguagePlugin>)> = vec![
            ("python2".to_string(), Box::new(Python2Plugin)),
            ("python3".to_string(), Box::new(Python3Plugin)),
            ("nodejs".to_string(), Box::new(NodeJSPlugin)),
            ("go".to_string(), Box::new(GoPlugin)),
            ("java".to_string(), Box::new(JavaPlugin)),
            ("shell".to_string(), Box::new(ShellPlugin)),
            ("powershell".to_string(), Box::new(PowerShellPlugin)),
            ("rust".to_string(), Box::new(RustPlugin)),
            ("swift".to_string(), Box::new(SwiftPlugin)),
            ("scala".to_string(), Box::new(ScalaPlugin)),
            ("kotlin".to_string(), Box::new(KotlinPlugin)),
            ("clojure".to_string(), Box::new(ClojurePlugin)),
            ("c".to_string(), Box::new(CPlugin)),
            ("ruby".to_string(), Box::new(RubyPlugin)),
            ("dart".to_string(), Box::new(DartPlugin)),
            ("perl".to_string(), Box::new(PerlPlugin)),
            ("julia".to_string(), Box::new(JuliaPlugin)),
            ("applescript".to_string(), Box::new(AppleScriptPlugin)),
            ("typescript".to_string(), Box::new(TypeScriptPlugin)),
            ("react".to_string(), Box::new(ReactPlugin)),
            ("vue".to_string(), Box::new(VuePlugin)),
            ("cpp".to_string(), Box::new(CppPlugin)),
            ("groovy".to_string(), Box::new(GroovyPlugin)),
            ("html".to_string(), Box::new(HtmlPlugin)),
            ("css".to_string(), Box::new(CssPlugin)),
            ("less".to_string(), Box::new(LessPlugin)),
            ("svg".to_string(), Box::new(SvgPlugin)),
            ("json".to_string(), Box::new(JsonPlugin)),
            ("xml".to_string(), Box::new(XmlPlugin)),
            ("yaml".to_string(), Box::new(YamlPlugin)),
            ("markdown".to_string(), Box::new(MarkdownPlugin)),
            ("text".to_string(), Box::new(TextPlugin)),
            ("csv".to_string(), Box::new(CsvPlugin)),
            ("tsv".to_string(), Box::new(TsvPlugin)),
            ("xlsx".to_string(), Box::new(XlsxPlugin)),
            ("sql".to_string(), Box::new(SqlPlugin)),
            ("php".to_string(), Box::new(PHPPlugin)),
            ("r".to_string(), Box::new(RPlugin)),
            ("cangjie".to_string(), Box::new(CangjiePlugin)),
            ("haskell".to_string(), Box::new(HaskellPlugin)),
            ("ocaml".to_string(), Box::new(OCamlPlugin)),
            ("tcl".to_string(), Box::new(TclPlugin)),
            ("lua".to_string(), Box::new(LuaPlugin)),
            ("objective-c".to_string(), Box::new(ObjectiveCPlugin)),
            ("objective-cpp".to_string(), Box::new(ObjectiveCppPlugin)),
            (
                "javascript-nodejs".to_string(),
                Box::new(JavaScriptNodeJsPlugin),
            ),
            (
                "typescript-nodejs".to_string(),
                Box::new(TypeScriptNodeJsPlugin),
            ),
            (
                "typescript-browser".to_string(),
                Box::new(TypeScriptBrowserPlugin),
            ),
            (
                "javascript-browser".to_string(),
                Box::new(JavaScriptBrowserPlugin),
            ),
            (
                "javascript-jquery".to_string(),
                Box::new(JavaScriptJQueryPlugin),
            ),
        ];

        for (key, plugin) in builtin_plugins {
            builtin_languages.push(key.clone());
            plugins.insert(key, plugin);
        }

        Self {
            plugins,
            builtin_languages,
        }
    }

    pub fn load_custom_plugins(&mut self, custom_configs: Vec<PluginConfig>) {
        for config in custom_configs {
            if !self.builtin_languages.contains(&config.language) {
                let custom_plugin = CustomPlugin::new(config.clone());
                self.plugins
                    .insert(config.language.clone(), Box::new(custom_plugin));
            }
        }
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
