// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use command_group::CommandGroup;
use std::process::Command;
use std::sync::mpsc::{sync_channel, Receiver};
use std::thread;
use tauri::api::process::Command as TCommand;
use tauri::Manager;
use tauri::WindowEvent;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {} from tauri!", name)
}

fn start_backend(receiver: Receiver<i32>) {
    let t = TCommand::new_sidecar("main").expect("failed to create sidecar");
    let mut group = Command::from(t)
        .group_spawn()
        .expect("failed to spawn api process");
    thread::spawn(move || loop {
        let s = receiver.recv();
        if s.unwrap() == -1 {
            group.kill().expect("failed killing api serivce process");
        }
    });
}

fn main() {
    let (tx, rx) = sync_channel(1);
    start_backend(rx);
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            Ok(())
        })
        .on_window_event(move |event| match event.event() {
            WindowEvent::Destroyed => {
                println!("closing api process");
                tx.send(-1).expect("failed to send close signal");
                println!("closed api process");
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
