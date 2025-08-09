use tauri::{
    AppHandle, Emitter,
    menu::{Menu, MenuBuilder, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder},
};

pub fn create_menu(app: &AppHandle) -> tauri::Result<Menu<tauri::Wry>> {
    let about_item = MenuItemBuilder::new("关于 CodeForge")
        .id("about")
        .build(app)?;

    // 应用菜单
    let app_submenu = SubmenuBuilder::new(app, "CodeForge")
        .item(&about_item)
        .build()?;

    // 编辑菜单
    let edit_submenu = SubmenuBuilder::new(app, "编辑")
        .item(&PredefinedMenuItem::undo(app, Option::from("撤销"))?)
        .item(&PredefinedMenuItem::redo(app, Option::from("重做"))?)
        .separator()
        .item(&PredefinedMenuItem::cut(app, Option::from("剪切"))?)
        .item(&PredefinedMenuItem::copy(app, Option::from("复制"))?)
        .item(&PredefinedMenuItem::paste(app, Option::from("粘贴"))?)
        .separator()
        .item(&PredefinedMenuItem::select_all(app, Option::from("全选"))?)
        .build()?;

    let menu = MenuBuilder::new(app)
        .items(&[&app_submenu, &edit_submenu])
        .build()?;

    Ok(menu)
}

pub fn setup_menu_handler(app: &AppHandle) {
    app.on_menu_event(move |app, event| match event.id().as_ref() {
        "about" => {
            let _event = app.emit("show-about", ());
        }
        _ => {}
    });
}
