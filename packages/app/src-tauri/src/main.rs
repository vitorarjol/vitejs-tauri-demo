#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{
  api::process::{Command},
  api::path
};

fn main() {
  tauri::Builder::default()
    // .setup(|_| {
    //   let repo_path = path::data_dir().unwrap().join("Demo");
    //   println!("{}", format!("--repo-path={}", repo_path.to_str().unwrap()));

    //   tauri::async_runtime::spawn(async move {
    //     let (mut rx, _) = Command::new_sidecar("mintterd")
    //       .expect("failed to setup `mintterd` sidecar")
    //       .args(&[format!("--repo-path={}", repo_path.to_str().unwrap())])
    //       .spawn()
    //       .expect("Failed to spawn packaged node");

    //     while let Some(event) = rx.recv().await {
    //       println!("{:?}", event);
    //     }
    //   });

    //   Ok(())
    // })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
