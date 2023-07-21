use tauri::api::dialog::message;
use tauri::utils::assets::EmbeddedAssets;
use tauri::{AboutMetadata, Context, CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent};

// 应用菜单处理事件
pub fn handler(event: WindowMenuEvent) {
    let win = Some(event.window());
    let menu_handle = win.unwrap().menu_handle();
    match event.menu_item_id() {
        "new_file" => {
            // 新开一个线程用于处理更新操作，否则应用会卡死
            std::thread::spawn(move || {
                menu_handle
                    .get_item("new_file") // 获取菜单项 id
                    .set_title("Create File") // 设置新的菜单名
                    .unwrap();
            });
            dbg!("new file");
        }
        "edit_file" => {
            message(win, "Eidt File", "TODO");
        }
        "undo" => {
            dbg!("undo");
        }
        "redo" => {
            dbg!("redo");
        }
        _ => {}
    }
}

pub fn init(context: &Context<EmbeddedAssets>) -> Menu {
    // 应用名称
    let name = &context.package_info().name;
    // tauri::Menu::os_default(name)
    // 应用主菜单
    let app_menu = Submenu::new(
        "",
        // MenuItem::About 为原生菜单
        Menu::new().add_native_item(MenuItem::About(name.into(), AboutMetadata::new())),
    );
    // 文件菜单（自定义菜单）
    let file_menu = Submenu::new(
        "File",
        Menu::new()
            .add_item(CustomMenuItem::new("new_file".to_string(), "New File"))
            .add_item(CustomMenuItem::new("edit_file".to_string(), "Edit File")),
    );
    // 编辑菜单（自定义菜单）
    let edit_menu = Submenu::new(
        "Edit",
        Menu::new()
            .add_item(CustomMenuItem::new("undo".to_string(), "Undo"))
            .add_item(CustomMenuItem::new("redo".to_string(), "Redo")),
    );

    Menu::new()
        .add_submenu(app_menu)
        .add_submenu(file_menu)
        .add_submenu(edit_menu)
}
