// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use tauri::{Manager, SystemTray};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    let tray = SystemTray::new().with_menu(tray_menu);

    let mut app = tauri::Builder::default()
        // Tauri OAuth redirect URL plugin
        .plugin(tauri_plugin_oauth::init())
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = match app.get_window("main") {
                    Some(window) => match window.is_visible().expect("winvis") {
                        true => {
                            window.hide().expect("winhide");
                            return;
                        }
                        false => window,
                    },
                    None => return,
                };

                #[cfg(not(target_os = "macos"))]
                {
                    window.show().unwrap();
                }
                window.set_focus().unwrap();
            }
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        // .invoke_handler(tauri::generate_handler![start_server])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    // Prevent app closing by closing the last window
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    app.run(|_app_handle, _event| {});
}
