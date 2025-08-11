use log::info;
use tauri::{
    AppHandle, Manager,
    menu::{MenuItemBuilder, Submenu, SubmenuBuilder},
};

pub fn create_developer_submenu(app: &AppHandle) -> tauri::Result<Submenu<tauri::Wry>> {
    let devtools_item = MenuItemBuilder::new("打开调试器")
        .id("open-devtools")
        .build(app)?;

    let developer_submenu = SubmenuBuilder::new(app, "开发者模式")
        .item(&devtools_item)
        .build()?;

    Ok(developer_submenu)
}

pub fn handle_developer_menu_event(app: &AppHandle, event_id: &str) {
    match event_id {
        "open-devtools" => {
            info!("开发者 -> 打开调试器");
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.open_devtools();
            }
        }
        _ => {}
    }
}
