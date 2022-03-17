use std::process::Command;

fn main() {
    #[cfg(debug_assertions)]
    let output_dir = "target/debug/";

    #[cfg(not(debug_assertions))]
    let output_dir = "target/release/";
    let compiled_output_name = "sqlite3_sys";

    Command::new("cc")
        .arg("-fpic")
        .arg("-D_POSIX_THREAD_SAFE_FUNCTIONS")
        .arg("-c")
        .arg("-I")
        .arg("c_source")
        .arg("c_source/sqlite3.c")
        .arg("-o")
        .arg(output_dir.to_owned() + compiled_output_name + ".o")
        .output()
        .expect("error-message1");

    Command::new("cc")
        .arg("-shared")
        .arg(output_dir.to_owned() + "sqlite3_sys.o" + "")
        .arg("-o")
        .arg(output_dir.to_owned() + "lib" + compiled_output_name + ".so")
        .output()
        .expect("error-message2");

    // Command::new("ldconfig").output().expect("error-message-3");

    // Command::new("ldconfig")
    //     .arg("-p")
    //     .output()
    //     .expect("error-message-4");
}
