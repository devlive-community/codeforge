use log::info;
use tauri::{
    AppHandle, Manager,
    menu::{MenuItemBuilder, Submenu, SubmenuBuilder},
};

pub fn create_developer_submenu(app: &AppHandle) -> tauri::Result<Submenu<tauri::Wry>> {
    let reload_item = MenuItemBuilder::new("重新加载")
        .id("reload-window")
        .accelerator("CmdOrCtrl+Shift+R")
        .build(app)?;

    let devtools_item = MenuItemBuilder::new("打开调试器")
        .id("open-devtools")
        .accelerator("CmdOrCtrl+Shift+I")
        .build(app)?;

    let developer_submenu = SubmenuBuilder::new(app, "开发者模式")
        .item(&reload_item)
        .item(&devtools_item)
        .build()?;

    Ok(developer_submenu)
}

pub fn handle_developer_menu_event(app: &AppHandle, event_id: &str) {
    match event_id {
        "reload-window" => {
            info!("开发者 -> 重新加载");
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.reload();
            }
        }
        "open-devtools" => {
            info!("开发者 -> 打开调试器");
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.open_devtools();
            }
        }
        _ => {}
    }
}
