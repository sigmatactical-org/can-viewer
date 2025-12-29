use std::process::Command;

fn main() {
    // Auto-build frontend in release mode
    if std::env::var("PROFILE").unwrap_or_default() == "release" {
        println!("cargo:warning=Building frontend for release...");
        let status = Command::new("npm")
            .args(["run", "build"])
            .status()
            .expect("Failed to run npm. Is Node.js installed?");
        if !status.success() {
            panic!("Frontend build failed");
        }
    }

    // Fail early if dist is missing
    if !std::path::Path::new("dist/index.html").exists() {
        panic!(
            "\n\
            ══════════════════════════════════════════════════════════════\n\
            ERROR: dist/ folder missing\n\
            \n\
            Run `npm run build` first to build the frontend.\n\
            ══════════════════════════════════════════════════════════════\n"
        );
    }

    tauri_build::build()
}
