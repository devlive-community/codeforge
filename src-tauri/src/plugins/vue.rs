use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct VuePlugin;

impl LanguagePlugin for VuePlugin {
    fn get_order(&self) -> i32 {
        15
    }

    fn get_language_name(&self) -> &'static str {
        "Vue"
    }

    fn get_language_key(&self) -> &'static str {
        "vue"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "vue".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--"]
    }

    fn get_path_command(&self) -> String {
        // 浏览器内运行时预览，无需本机依赖（与 HTML 一致）
        "--".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("vue"),
            before_compile: None,
            extension: String::from("vue"),
            execute_home: None,
            // 浏览器预览：CDN 引入 Vue 3 全局构建（含运行时模板编译器），
            // 用户代码用 Vue.createApp(...).mount('#app') 直接挂载。
            run_command: Some(String::from(
                "echo <div id=\"app\"></div>\n<script src=\"https://unpkg.com/vue@3/dist/vue.global.js\"></script>\n<script src=\"file://$filename\"></script>",
            )),
            after_compile: None,
            template: Some(String::from(
                "const { createApp, ref } = Vue\n\ncreateApp({\n  setup() {\n    const msg = ref('Hello, Vue!')\n    return { msg }\n  },\n  template: `<h1>{{ msg }}</h1>`\n}).mount('#app')",
            )),
            timeout: Some(30),
            console_type: Some(String::from("web")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "echo".to_string())
    }
}
