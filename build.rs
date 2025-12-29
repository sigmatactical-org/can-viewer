fn main() {
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
