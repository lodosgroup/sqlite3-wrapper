use std::{path::Path, env};

fn main() {
    println!("cargo:rustc-link-lib=dylib=sqlite3_sys");

    let home_path = env::var("HOME").expect("HOME environment variable is not set.");
    let target_dir = Path::new(&home_path).join(".local/share/sqlite3_sys");
    println!("cargo:rustc-link-search={}", target_dir.display());
}
