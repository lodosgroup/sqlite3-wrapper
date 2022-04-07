use std::{
    env, fs,
    path::Path,
    process::{self, Command},
};

fn main() {
    if env::var("DOCS_RS").is_ok() {
        println!("cargo:warning=docs.rs build detected. Process will safely exit.");
        process::exit(0);
    }

    let home_path =
        env::var("HOME").unwrap_or_else(|_| panic!("HOME environment variable is not set."));
    let output_dir = Path::new(".");
    let compiled_output_name = "min_sqlite3_sys";

    let target_dir = Path::new(&home_path).join(".local/share/min_sqlite3_sys");
    let target_dylib_path = target_dir.join("lib".to_owned() + compiled_output_name + ".so");

    if target_dylib_path.exists() {
        println!(
            "cargo:warning=libmin_sqlite3_sys already exists on system. Process will safely exit."
        );
        process::exit(0);
    }

    #[allow(clippy::clone_double_ref)]
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
        .unwrap_or_else(|_| panic!("Couldn't compile c_source into object file."));

    #[allow(clippy::clone_double_ref)]
    let dylib_path = output_dir
        .clone()
        .join("lib".to_owned() + compiled_output_name + ".so");

    #[allow(clippy::clone_double_ref)]
    let ofile_path = output_dir
        .clone()
        .join(compiled_output_name.to_owned() + ".o");
    Command::new("cc")
        .arg("-shared")
        .arg(&ofile_path)
        .arg("-o")
        .arg(&dylib_path)
        .output()
        .unwrap_or_else(|_| {
            panic!(
                "Couldn't create shared object from {}",
                ofile_path.display()
            )
        });

    // set library permission as read-only
    let mut lib_permissions = fs::metadata(&dylib_path)
        .unwrap_or_else(|_| panic!("Error reading {} permissions.", &dylib_path.display()))
        .permissions();
    lib_permissions.set_readonly(true);
    fs::set_permissions(&dylib_path, lib_permissions).unwrap_or_else(|_| {
        panic!(
            "Got an error while setting the file permission of {} as read-only",
            &dylib_path.display()
        )
    });

    fs::create_dir_all(&target_dir)
        .unwrap_or_else(|_| panic!("{} could not create.", &target_dir.display()));

    fs::copy(&dylib_path, &target_dylib_path).unwrap_or_else(|_| {
        panic!(
            "{} could not copy into {}",
            dylib_path.display(),
            target_dir.display()
        )
    });
}
