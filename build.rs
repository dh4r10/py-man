// Genera `pvm_bytes.rs` en OUT_DIR con el contenido de pvm.exe.
// Si pvm.exe aún no existe, genera un slice vacío; el instalador mostrará
// un error claro en runtime en lugar de fallar en tiempo de compilación.

use std::path::PathBuf;

fn main() {
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    // CARGO_MANIFEST_DIR apunta a la raíz del workspace (donde está Cargo.toml)
    let pvm_exe = PathBuf::from(&manifest)
        .join("target")
        .join("release")
        .join("pvm.exe");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_file = PathBuf::from(&out_dir).join("pvm_bytes.rs");

    println!("cargo:rerun-if-changed={}", pvm_exe.display());
    println!("cargo:rerun-if-env-changed=CARGO_MANIFEST_DIR");

    let content = if pvm_exe.exists() {
        // Usar barras normales: funciona en Windows en include_bytes!
        let path = pvm_exe.to_string_lossy().replace('\\', "/");
        format!(r#"pub static PVM_EXE: &[u8] = include_bytes!("{path}");"#)
    } else {
        // pvm.exe todavía no compilado — el instalador lo detecta en runtime
        r#"pub static PVM_EXE: &[u8] = &[];"#.to_string()
    };

    std::fs::write(&out_file, content).unwrap();
}
