use std::{env::var, path::Path, process::Command};

fn main() {
    let output_dir = var("OUT_DIR").unwrap();
    let output_dir = Path::new(&output_dir);

    let lib_status = Command::new("ld")
        .arg("-lsqlite3_sys")
        .arg("-o")
        .arg(output_dir.clone().join("tmp.o"))
        .output()
        .unwrap()
        .status;

    if lib_status.success() {
        std::process::exit(0);
    }

    let compiled_output_name = "sqlite3_sys";

    Command::new("cc")
        .arg("-fpic")
        .arg("-D_POSIX_THREAD_SAFE_FUNCTIONS")
        .arg("-c")
        .arg("-I")
        .arg("c_source")
        .arg("c_source/sqlite3.c")
        .arg("-o")
        .arg(
            output_dir
                .clone()
                .join(compiled_output_name.to_owned() + ".o"),
        )
        .output()
        .expect("error-message1");

    Command::new("cc")
        .arg("-shared")
        .arg(
            output_dir
                .clone()
                .join(compiled_output_name.to_owned() + ".o"),
        )
        .arg("-o")
        .arg(
            output_dir
                .clone()
                .join("lib".to_owned() + compiled_output_name + ".so"),
        )
        .output()
        .expect("error-message2");
}
