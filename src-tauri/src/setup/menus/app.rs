use log::info;
use tauri::{
    AppHandle, Emitter,
    menu::{MenuItemBuilder, Submenu, SubmenuBuilder},
};

pub fn create_app_submenu(app: &AppHandle) -> tauri::Result<Submenu<tauri::Wry>> {
    let about_item = MenuItemBuilder::new("关于 CodeForge")
        .id("about")
        .build(app)?;

    let update_item = MenuItemBuilder::new("检查更新").id("update").build(app)?;

    let settings_item = MenuItemBuilder::new("设置")
        .id("settings")
        .accelerator("CmdOrCtrl+,")
        .build(app)?;

    #[cfg(target_os = "macos")]
    let hide_item = MenuItemBuilder::new("隐藏 CodeForge")
        .id("hide")
        .accelerator("CmdOrCtrl+H")
        .build(app)?;

    #[cfg(not(target_os = "macos"))]
    let hide_item = MenuItemBuilder::new("最小化 CodeForge")
        .id("hide")
        .accelerator("CmdOrCtrl+M")
        .build(app)?;

    let restart_item = MenuItemBuilder::new("重启 CodeForge")
        .id("restart")
        .accelerator("CmdOrCtrl+R")
        .build(app)?;

    let quit_item = MenuItemBuilder::new("退出 CodeForge")
        .id("quit")
        .accelerator("CmdOrCtrl+Q")
        .build(app)?;

    let app_submenu = SubmenuBuilder::new(app, "CodeForge")
        .item(&about_item)
        .separator()
        .item(&update_item)
        .separator()
        .item(&settings_item)
        .separator()
        .item(&hide_item)
        .separator()
        .item(&restart_item)
        .item(&quit_item)
        .build()?;

    Ok(app_submenu)
}

pub fn handle_app_menu_event(app: &AppHandle, event_id: &str) {
    match event_id {
        "about" => {
            let _event = app.emit("show-about", ());
        }
        "update" => {
            let _event = app.emit("show-update", ());
        }
        "settings" => {
            let _event = app.emit("show-settings", ());
        }
        "hide" => {
            #[cfg(target_os = "macos")]
            {
                info!("隐藏应用 CodeForge");
                let _ = app.hide();
            }

            #[cfg(not(target_os = "macos"))]
            {
                info!("最小化应用 CodeForge");
                // 获取主窗口并最小化
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.minimize();
                } else {
                    // 如果找不到 main 窗口，尝试获取第一个可用窗口
                    let windows = app.webview_windows();
                    if let Some((_, window)) = windows.iter().next() {
                        let _ = window.minimize();
                    }
                }
            }
        }
        "restart" => {
            info!("CodeForge 应用重启");
            app.restart();
        }
        "quit" => {
            info!("CodeForge 应用关闭");
            app.exit(0);
        }
        _ => {}
    }
}
