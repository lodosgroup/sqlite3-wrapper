use std::{fs, path::Path, process::Command};

fn main() {
    let home_path = std::env::var("HOME").expect("HOME environment variable is not set.");
    let output_dir = Path::new("../target");
    let compiled_output_name = "min_sqlite3_sys";

    let target_dir = Path::new(&home_path).join(".local/share/min_sqlite3_sys");
    let target_dylib_path = target_dir.join("lib".to_owned() + compiled_output_name + ".so");

    if target_dylib_path.exists() {
        println!(
            "cargo:warning={}",
            "libmin_sqlite3_sys already exists on system. Process will safely exit."
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
    let mut lib_permissions = std::fs::metadata(&dylib_path)
        .expect(&format!("Error reading {} permissions.", &dylib_path.display()).to_owned())
        .permissions();
    lib_permissions.set_readonly(true);
    std::fs::set_permissions(&dylib_path, lib_permissions).expect(
        &format!(
            "Got an error while setting the file permission of {} as read-only",
            &dylib_path.display()
        )
        .to_owned(),
    );

    fs::create_dir_all(&target_dir)
        .expect(&format!("{} could not create.", &target_dir.display()).to_owned());

    fs::copy(&dylib_path, &target_dylib_path).expect(
        &format!(
            "{} could not copy into {}",
            dylib_path.display(),
            target_dir.display()
        )
        .to_owned(),
    );
}
