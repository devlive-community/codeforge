use log::info;
use tauri::AppHandle;

#[tauri::command]
pub async fn open_font_picker(_app_handle: AppHandle) -> Result<Option<String>, String> {
    info!("编辑器设置 -> 打开字体选择器");

    let file_handle = rfd::FileDialog::new()
        .add_filter("字体文件", &["ttf", "otf", "woff", "woff2"])
        .set_title("选择字体文件")
        .pick_file();

    match file_handle {
        Some(path) => {
            let font_name = path
                .file_stem() // 只取不带扩展名的文件名
                .ok_or_else(|| "无法获取字体名".to_string())?
                .to_str()
                .ok_or_else(|| "字体名包含无效字符".to_string())?
                .to_string();

            info!("编辑器设置 -> 用户选择了字体: {}", font_name);
            Ok(Some(font_name))
        }
        None => {
            info!("编辑器设置 -> 用户取消了字体选择");
            Ok(None)
        }
    }
}
