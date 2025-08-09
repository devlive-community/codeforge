use super::menus;

use tauri::{
    AppHandle,
    menu::{Menu, MenuBuilder},
};

pub fn create_menu(app: &AppHandle) -> tauri::Result<Menu<tauri::Wry>> {
    // 应用菜单
    let app_submenu = menus::app::create_app_submenu(app)?;

    // 编辑菜单
    let edit_submenu = menus::edit::create_edit_submenu(app)?;

    let menu = MenuBuilder::new(app)
        .items(&[&app_submenu, &edit_submenu])
        .build()?;

    Ok(menu)
}

pub fn setup_menu_handler(app: &AppHandle) {
    app.on_menu_event(move |app, event| {
        menus::app::handle_app_menu_event(app, event.id().as_ref());
        menus::edit::handle_edit_menu_event(app, event.id().as_ref());
    });
}
