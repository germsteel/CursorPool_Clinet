use tauri::{
    // Emitter,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App,
    AppHandle,
    Manager,
};

// use crate::cursor_reset::reset_machine_id;

fn show_and_focus_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

pub fn setup_system_tray(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let show = MenuItem::with_id(app, "显示", "显示", true, None::<&str>)?;
    // let switch_account_item = MenuItem::with_id(app, "switch_account", "一键换号", true, None::<&str>)?;
    // let switch_account_manual = MenuItem::with_id(app, "switch_account_manual", "切换账号", true, None::<&str>)?;
    // let switch_machine = MenuItem::with_id(app, "switch_machine", "换机器码", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            // &switch_account_item,
            // &switch_account_manual,
            // &switch_machine,
            &show, &quit,
        ],
    )?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .tooltip("Cursor Pool")
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_and_focus_window(tray.app_handle());
            }
        })
        .on_menu_event(move |app, event| {
            match event.id.as_ref() {
                "quit" => {
                    app.exit(0);
                }
                "show" => {
                    show_and_focus_window(app.app_handle());
                }
                // "switch_account" => {
                //     // 一键换号
                //     match get_current_account() {
                //         Ok(account_info) => {
                //             if let Some(email) = account_info.get("email").and_then(|v| v.as_str()) {
                //                 if let Some(token) = account_info.get("access_token").and_then(|v| v.as_str()) {
                //                     let email = email.to_string();
                //                     let token = token.to_string();
                //                     tauri::async_runtime::spawn(async move {
                //                         if let Err(e) = switch_account(email, token).await {
                //                             println!("切换账号失败: {}", e);
                //                         }
                //                     });
                //                 }
                //             }
                //         }
                //         Err(e) => println!("获取当前账户信息失败: {}", e),
                //     }
                // }
                // "switch_account_manual" => {
                //     // 手动换号 - 显示窗口
                //     show_and_focus_window(app.app_handle());
                // }
                // "switch_machine" => {
                //     // 换机器码
                //     let app_handle = app.app_handle().clone();
                //     tauri::async_runtime::spawn(async move {
                //         match reset_machine_id(true).await {
                //             Ok(_) => {
                //                 if let Err(e) = app_handle.emit("machine-id-changed", ()) {
                //                     eprintln!("发送机器码变更事件失败: {}", e);
                //                 }
                //             }
                //             Err(e) => eprintln!("重置机器码失败: {}", e),
                //         }
                //     });
                // }
                _ => (),
            }
        })
        .build(app)?;

    Ok(())
}
