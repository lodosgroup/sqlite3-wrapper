use std::{path::Path, process::Command};

fn main() {
    let output_dir = Path::new("../target");
    let compiled_output_name = "sqlite3_sys";

    let lib_status = Command::new("ld")
        .arg("-lsqlite3_sys")
        .arg("-o")
        .arg(output_dir.clone().join("tmp.o"))
        .output()
        .expect(
            &format!(
                "Got an error while executing `ld -lsqlite3_sys -o {}`",
                output_dir.clone().join("tmp.o").display()
            )
            .to_owned(),
        )
        .status;

    if lib_status.success() {
        println!(
            "cargo:warning={}",
            "libsqlte3_sys already exists on system. Process will safely exit."
        );
        std::process::exit(0);
    }

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
        .expect("Couldn't compile c_source into object file.");

    let dylib_path = output_dir
        .clone()
        .join("lib".to_owned() + compiled_output_name + ".so");
    let ofile_path = output_dir
        .clone()
        .join(compiled_output_name.to_owned() + ".o");
    Command::new("cc")
        .arg("-shared")
        .arg(&ofile_path)
        .arg("-o")
        .arg(&dylib_path)
        .output()
        .expect(&format!(
            "Couldn't create shared object from {}",
            ofile_path.display()
        ));

    // set library permission as read-only
    let mut lib_permissions = std::fs::metadata(&dylib_path).unwrap().permissions();
    lib_permissions.set_readonly(true);
    std::fs::set_permissions(&dylib_path, lib_permissions).expect("TODO");
}
