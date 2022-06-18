use std::process::Command;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=static/sass");
    // Call the sass compiler installed on the machine
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", "sass static/sass/:static/css/"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("sass static/sass/:static/css/")
                .output()
                .expect("failed to execute process")
    };
    let epic = std::str::from_utf8(output.stdout.as_slice()).unwrap();
    println!("{}", epic);
    println!("Compiled SASS");
}