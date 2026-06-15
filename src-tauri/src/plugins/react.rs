use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct ReactPlugin;

impl LanguagePlugin for ReactPlugin {
    fn get_order(&self) -> i32 {
        14
    }

    fn get_language_name(&self) -> &'static str {
        "React (JSX)"
    }

    fn get_language_key(&self) -> &'static str {
        "react"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "jsx".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--"]
    }

    fn get_path_command(&self) -> String {
        "which node".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: self.get_language_key().to_string(),
            before_compile: None,
            extension: String::from("jsx"),
            execute_home: None,
            // 浏览器预览：CDN 引入 React/ReactDOM + Babel，以 text/babel 在浏览器内转译 JSX。
            // 用户代码可向 #root 渲染：ReactDOM.createRoot(document.getElementById('root')).render(...)
            run_command: Some(String::from(
                "echo <div id=\"root\"></div>\n<script crossorigin src=\"https://unpkg.com/react@18/umd/react.development.js\"></script>\n<script crossorigin src=\"https://unpkg.com/react-dom@18/umd/react-dom.development.js\"></script>\n<script src=\"https://unpkg.com/@babel/standalone/babel.min.js\"></script>\n<script type=\"text/babel\" src=\"file://$filename\"></script>",
            )),
            after_compile: None,
            template: Some(String::from(
                "function App() {\n  return <h1>Hello, React!</h1>;\n}\n\nReactDOM.createRoot(document.getElementById('root')).render(<App />);",
            )),
            timeout: Some(30),
            console_type: Some(String::from("web")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "node".to_string())
    }
}
