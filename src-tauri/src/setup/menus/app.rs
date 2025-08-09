use tauri::{
    AppHandle, Emitter,
    menu::{MenuItemBuilder, Submenu, SubmenuBuilder},
};

pub fn create_app_submenu(app: &AppHandle) -> tauri::Result<Submenu<tauri::Wry>> {
    let about_item = MenuItemBuilder::new("关于 CodeForge")
        .id("about")
        .build(app)?;

    let quit_item = MenuItemBuilder::new("退出 CodeForge")
        .id("quit")
        .accelerator("CmdOrCtrl+Q")
        .build(app)?;

    let settings_item = MenuItemBuilder::new("设置")
        .id("settings")
        .accelerator("CmdOrCtrl+,")
        .build(app)?;

    let app_submenu = SubmenuBuilder::new(app, "CodeForge")
        .item(&about_item)
        .separator()
        .item(&settings_item)
        .separator()
        .item(&quit_item)
        .build()?;

    Ok(app_submenu)
}

pub fn handle_app_menu_event(app: &AppHandle, event_id: &str) {
    match event_id {
        "about" => {
            let _event = app.emit("show-about", ());
        }
        "settings" => {
            let _event = app.emit("show-settings", ());
        }
        "quit" => {
            app.exit(0);
        }
        _ => {}
    }
}
