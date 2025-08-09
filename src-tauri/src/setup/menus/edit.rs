use tauri::menu::PredefinedMenuItem;

use tauri::{
    AppHandle, Manager,
    menu::{MenuItemBuilder, Submenu, SubmenuBuilder},
};

pub fn create_edit_submenu(app: &AppHandle) -> tauri::Result<Submenu<tauri::Wry>> {
    let undo_item = MenuItemBuilder::new("撤销")
        .id("undo")
        .accelerator("CmdOrCtrl+Z")
        .build(app)?;

    let redo_item = MenuItemBuilder::new("重做")
        .id("redo")
        .accelerator("CmdOrCtrl+Shift+Z")
        .build(app)?;

    let cut_item = MenuItemBuilder::new("剪切")
        .id("cut")
        .accelerator("CmdOrCtrl+X")
        .build(app)?;

    let copy_item = MenuItemBuilder::new("复制")
        .id("copy")
        .accelerator("CmdOrCtrl+C")
        .build(app)?;

    let select_all_item = MenuItemBuilder::new("全选")
        .id("select_all")
        .accelerator("CmdOrCtrl+A")
        .build(app)?;

    let edit_submenu = SubmenuBuilder::new(app, "编辑")
        .item(&undo_item)
        .item(&redo_item)
        .separator()
        .item(&cut_item)
        .item(&copy_item)
        .item(&PredefinedMenuItem::paste(app, Option::from("粘贴"))?)
        .separator()
        .item(&select_all_item)
        .build()?;

    Ok(edit_submenu)
}

pub fn handle_edit_menu_event(app: &AppHandle, event_id: &str) {
    let binding = app.webview_windows();
    let webview = binding.values().next().unwrap();

    match event_id {
        "undo" => {
            webview.eval("document.execCommand('undo')").ok();
        }
        "redo" => {
            webview.eval("document.execCommand('redo')").ok();
        }
        "cut" => {
            webview.eval("document.execCommand('cut')").ok();
        }
        "copy" => {
            webview.eval("document.execCommand('copy')").ok();
        }
        "select_all" => {
            webview.eval("document.execCommand('selectAll')").ok();
        }
        _ => {}
    }
}
