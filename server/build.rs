use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=templates/");
    // Run tailwind for the templates
    Command::new("node_modules/.bin/tailwindcss")
        .args(["-i", "frontend/tailwind.css", "-o", "public/main.css"])
        .status()
        .unwrap();

    println!("cargo:rerun-if-changed=frontend/main.ts");
    Command::new("node_modules/.bin/esbuild")
        .args(["frontend/main.ts", "--bundle", "--outfile=public/main.js"])
        .status()
        .unwrap();
}
