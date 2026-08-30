use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Emitter, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub body: String,
    pub color: String,
    pub archived: bool,
    pub created_at: String,
    pub updated_at: String,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub monitor_id: Option<String>,
    pub always_on_top: Option<bool>,
    pub desktop_attached: Option<bool>,
}

fn notes_path(app: &AppHandle) -> Result<PathBuf, String> {
    let d = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&d).map_err(|e| e.to_string())?;
    Ok(d.join("notes.json"))
}

fn seed_notes() -> Vec<Note> {
    vec![
        Note { id: "office".into(), title: "Office".into(), body: "- understand all the APIs\n- create tickets for PRD creation".into(), color: "blue".into(), archived: false, created_at: "2026-08-29T18:00:00Z".into(), updated_at: "2026-08-29T18:00:00Z".into(), x: Some(1080), y: Some(250), width: Some(350.0), height: Some(360.0), monitor_id: None, always_on_top: Some(true), desktop_attached: Some(false) },
        Note { id: "groceries".into(), title: "Groceries".into(), body: "- apple\n- 4x banana\n- dry fruits\n- peanuts".into(), color: "mint".into(), archived: false, created_at: "2026-08-29T17:00:00Z".into(), updated_at: "2026-08-29T17:00:00Z".into(), x: Some(1080), y: Some(260), width: Some(350.0), height: Some(360.0), monitor_id: None, always_on_top: Some(true), desktop_attached: Some(false) },
        Note { id: "side-projects".into(), title: "Side-projects".into(), body: "- understand the architecture of the backend api...".into(), color: "yellow".into(), archived: false, created_at: "2026-08-29T16:00:00Z".into(), updated_at: "2026-08-29T16:00:00Z".into(), x: None, y: None, width: None, height: None, monitor_id: None, always_on_top: Some(true), desktop_attached: Some(false) },
    ]
}

#[tauri::command]
fn load_notes(app: AppHandle) -> Result<Vec<Note>, String> {
    let p = notes_path(&app)?;
    if !p.exists() {
        let n = seed_notes();
        fs::write(&p, serde_json::to_vec_pretty(&n).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
        return Ok(n);
    }
    let b = fs::read(&p).map_err(|e| e.to_string())?;
    let mut n: Vec<Note> = serde_json::from_slice(&b).map_err(|e| e.to_string())?;
    for x in &mut n {
        if x.always_on_top.is_none() { x.always_on_top = Some(true); }
        if x.desktop_attached.is_none() { x.desktop_attached = Some(false); }
    }
    Ok(n)
}

#[tauri::command]
fn save_notes(app: AppHandle, notes: Vec<Note>) -> Result<(), String> {
    let p = notes_path(&app)?;
    let t = p.with_extension("json.tmp");
    fs::write(&t, serde_json::to_vec_pretty(&notes).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    fs::rename(t, p).map_err(|e| e.to_string())
}

#[cfg(windows)]
#[tauri::command]
fn set_desktop_mode(app: AppHandle, label: String, enabled: bool) -> Result<(), String> {
    use windows_sys::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrW, SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE, HWND_BOTTOM, HWND_TOP, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, WS_EX_TOOLWINDOW};
    let w = app.get_webview_window(&label).ok_or_else(|| "window not found".to_string())?;
    let h = w.hwnd().map_err(|e| e.to_string())?;
    let raw = h.0;
    unsafe {
        let style = GetWindowLongPtrW(raw, GWL_EXSTYLE);
        let new_style = if enabled { style | WS_EX_TOOLWINDOW as isize } else { style & !(WS_EX_TOOLWINDOW as isize) };
        SetWindowLongPtrW(raw, GWL_EXSTYLE, new_style);
        let insert_after = if enabled { HWND_BOTTOM } else { HWND_TOP };
        SetWindowPos(raw, insert_after, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW);
    }
    Ok(())
}

#[cfg(not(windows))]
#[tauri::command]
fn set_desktop_mode(_app: AppHandle, _label: String, _enabled: bool) -> Result<(), String> { Ok(()) }

fn show_dashboard(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.set_focus();
    }
}

fn new_note(app: &AppHandle) {
    let _ = app.emit("hmn:create-note", ());
    show_dashboard(app);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut b = tauri::Builder::default();

    #[cfg(desktop)]
    {
        b = b.plugin(tauri_plugin_single_instance::init(|app, _, _| show_dashboard(app)));
        b = b.plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, None));
        b = b.plugin(tauri_plugin_global_shortcut::Builder::new().with_handler(|app, shortcut, event| {
            use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};
            let target = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyN);
            if shortcut == &target && event.state() == ShortcutState::Pressed { new_note(app); }
        }).build());
    }

    b.invoke_handler(tauri::generate_handler![load_notes, save_notes, set_desktop_mode])
        .on_window_event(|w, e| {
            if w.label() == "main" {
                if let tauri::WindowEvent::CloseRequested { api, .. } = e {
                    api.prevent_close();
                    let _ = w.hide();
                }
            }
        })
        .setup(|app| {
            load_notes(app.handle().clone()).ok();
            #[cfg(desktop)]
            {
                use tauri::menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem};
                use tauri::tray::TrayIconBuilder;
                let show = MenuItemBuilder::with_id("show", "Show dashboard").build(app)?;
                let new = MenuItemBuilder::with_id("new", "New note   Ctrl+Alt+N").build(app)?;
                let hide = MenuItemBuilder::with_id("hide", "Hide dashboard").build(app)?;
                let sep = PredefinedMenuItem::separator(app)?;
                let quit = MenuItemBuilder::with_id("quit", "Quit Hold My Notes").build(app)?;
                let menu = MenuBuilder::new(app).items(&[&show, &new, &hide, &sep, &quit]).build()?;
                TrayIconBuilder::new().menu(&menu).show_menu_on_left_click(false).on_menu_event(|app, e| match e.id.as_ref() {
                    "show" => show_dashboard(app),
                    "new" => new_note(app),
                    "hide" => { if let Some(w) = app.get_webview_window("main") { let _ = w.hide(); } }
                    "quit" => app.exit(0),
                    _ => {}
                }).build(app)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Hold My Notes");
}
