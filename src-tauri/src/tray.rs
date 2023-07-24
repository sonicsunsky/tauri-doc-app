use tauri::{
    AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    SystemTraySubmenu,
};

use tauri::Manager;

// 托盘菜单
pub fn menu() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let change_ico = CustomMenuItem::new("change_ico".to_string(), "Change Icon");
    let tray_menu = SystemTrayMenu::new()
        .add_submenu(SystemTraySubmenu::new(
            "Language", // 语言菜单
            SystemTrayMenu::new()
                .add_item(CustomMenuItem::new("lang_english".to_string(), "English"))
                .add_item(CustomMenuItem::new("lang_zh_CN".to_string(), "简体中文"))
                .add_item(CustomMenuItem::new("lang_zh_HK".to_string(), "繁体中文")),
        ))
        .add_native_item(SystemTrayMenuItem::Separator) // 分割线
        .add_item(change_ico)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu) //添加右键打开托盘菜单
}

// 托盘事件
pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            // let item_handle = app.tray_handle().get_item(&id);
            match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                "change_ico" => {
                    // 更新托盘图标
                    app.tray_handle()
                        .set_icon(tauri::Icon::Raw(
                            include_bytes!("../icons/icon.png").to_vec(),
                        ))
                        .unwrap();
                }
                lang if lang.contains("lang_") => {
                    // 选择语言，匹配 id 前缀包含 `lang_` 的事件
                    Lang::new(
                        app,
                        id, // 点击菜单的 id
                        vec![
                            Lang {
                                name: "English",
                                id: "lang_english",
                            },
                            Lang {
                                name: "繁体中文",
                                id: "lang_zh_HK",
                            },
                            Lang {
                                name: "简体中文",
                                id: "lang_zh_CN",
                            },
                        ],
                    );
                }
                _ => {}
            }
        }
        _ => {}
    }
}

struct Lang<'a> {
    name: &'a str,
    id: &'a str,
}

impl Lang<'static> {
    fn new(app: &AppHandle, id: String, langs: Vec<Lang>) {
        // 获取点击的菜单项的句柄
        // 注意 `tray_handle` 可以在任何地方调用，只需在 setup 钩子上使用 `app.handle()` 获取 `AppHandle` 实例，将其移动到另一个函数或线程
        langs.iter().for_each(|lang| {
            let handle = app.tray_handle().get_item(lang.id);
            if lang.id.to_string() == id.as_str() {
                // 设置菜单名称
                handle.set_title(format!("🥳 {}", lang.name)).unwrap();
                // 还可以使用 `set_selected`、`set_enabled` 和 `set_native_image`（仅限 macOS）
                handle.set_selected(true).unwrap();
            } else {
                handle.set_title(lang.name).unwrap();
                handle.set_selected(false).unwrap();
            }
        });
    }
}
