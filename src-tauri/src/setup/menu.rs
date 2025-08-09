use tauri::{
    menu::{Menu, MenuBuilder, MenuItemBuilder, SubmenuBuilder}, AppHandle,
    Emitter,
};

pub fn create_menu(app: &AppHandle) -> tauri::Result<Menu<tauri::Wry>> {
    let about_item = MenuItemBuilder::new("关于 CodeForge")
        .id("about")
        .build(app)?;

    let app_submenu = SubmenuBuilder::new(app, "CodeForge")
        .item(&about_item)
        .build()?;

    let menu = MenuBuilder::new(app).items(&[&app_submenu]).build()?;

    Ok(menu)
}

pub fn setup_menu_handler(app: &AppHandle) {
    app.on_menu_event(move |app, event| {
        if event.id().as_ref() == "about" {
            let _event = app.emit("show-about", ());
        }
    });
}
