#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use shortcut::run;
use auto_launch::AutoLaunch;
use std::env;

#[tokio::main]
async fn main() {
    let binding = env::current_exe().unwrap();
    let app_path = binding.to_str().unwrap();
    let app_name = "shortcut";
    let args = &["--minimized"];

    #[cfg(target_os = "windows")] {
        let auto = AutoLaunch::new(app_name, app_path, args);
        setup_auto_launch(auto);
    }

    #[cfg(target_os = "macos")] {
        let auto = AutoLaunch::new(app_name, app_path, false, args);
        setup_auto_launch(auto);
    }

    #[cfg(target_os = "linux")] {
        let auto = AutoLaunch::new(app_name, app_path, args);
        setup_auto_launch(auto);
    }

    run().await;
}

fn setup_auto_launch(auto: AutoLaunch) {
    // enable the auto launch
    let _ =auto.enable().is_ok();
    auto.is_enabled().unwrap();
}