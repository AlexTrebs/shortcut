use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let target = env::var("TARGET").unwrap();

    let ui_dest = PathBuf::from(manifest_dir)
        .join("target")
        .join(target)
        .join(profile)
        .join("ui");
    let ui_src = Path::new("ui");
    println!("us_dest: {:?}", ui_dest.to_str());

    println!("cargo:info=Copying UI folder to build output...");

    if !ui_src.exists() {
        panic!("cargo:error=UI directory does not exist! Make sure it is in the project root.");
    }

    fs::create_dir_all(&ui_dest).expect("cargo:error=Failed to create UI directory.");

    copy_dir(ui_src, &ui_dest).expect("cargo:error=Failed to copy UI files.");

    println!("cargo:info=UI folder copied successfully!");

    // Apply OS-specific setup after copying files
    apply_os_specific_setup(&ui_dest);
}

/// Recursively copies all files and subdirectories
fn copy_dir(src: &Path, dest: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if path.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_dir(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}

/// OS-specific setup function
fn apply_os_specific_setup(ui_dest: &Path) {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    match target_os.as_str() {
        "windows" => {
            println!("cargo:rerun-if-changed=app_icon.rc");
            println!("cargo:rustc-link-arg=app_icon.res");
            println!("cargo:info=Applying Windows-specific setup...");
            let _ = std::process::Command::new("attrib")
                .args(["+R", ui_dest.to_str().unwrap()])
                .status();
        }
        "linux" => {
            println!("cargo:info=Applying Linux-specific setup...");
            let _ = std::process::Command::new("chmod")
                .args(["-R", "755", ui_dest.to_str().unwrap()])
                .status();
        }
        "macos" => {
            println!("cargo:info=Applying macOS-specific setup...");
            let _ = std::process::Command::new("chmod")
                .args(["-R", "755", ui_dest.to_str().unwrap()])
                .status();
        }
        _ => {
            println!("cargo:error=Unknown target OS: {}", target_os);
        }
    }
}
