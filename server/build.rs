use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=templates/");
    // Run tailwind for the templates
    Command::new("npx")
        .args([
            "--yes",
            "tailwindcss",
            "-i",
            "frontend/tailwind.css",
            "-o",
            "public/main.css",
        ])
        .status()
        .unwrap();

    println!("cargo:rerun-if-changed=frontend/main.js");
    Command::new("npx")
        .args([
            "esbuild",
            "frontend/main.js",
            "--bundle",
            "--outfile=public/main.js"
        ])
        .status()
        .unwrap();
}
