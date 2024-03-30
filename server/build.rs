use std::process::Command;
use oxipng::optimize;
use walkdir::WalkDir;

fn main() {
    println!("cargo:rerun-if-changed=templates/");
    // Run tailwind for the templates
    Command::new("node_modules/.bin/tailwindcss")
        .args(["-i", "frontend/tailwind.css", "-o", "public/main.css"])
        .status()
        .unwrap();

    println!("cargo:rerun-if-changed=frontend/main.ts");
    // Run esbuild for the frontend
    Command::new("node_modules/.bin/esbuild")
        .args(["frontend/main.ts", "--bundle", "--sourcemap", "--outfile=public/main.js"])
        .status()
        .unwrap();

    println!("cargo:rerun-if-changed=content/images/");
    // Optimize images using oxipng to public/images directory
    // Ensure the public/images directory exists
    std::fs::create_dir_all("public/images").unwrap();
    let public_images_dir = std::path::Path::new("public/images");
    for entry in WalkDir::new("content/images") {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            let entry = entry.path();
            let in_path = oxipng::InFile::Path(entry.to_path_buf());
            let output = public_images_dir.join(entry.file_name().unwrap());
            // If the file already exists and is newer than the input, skip it
            if output.exists() {
                let in_meta = entry.metadata().unwrap();
                let out_meta = output.metadata().unwrap();
                if out_meta.modified().unwrap() > in_meta.modified().unwrap() {
                    continue;
                }
            }
            let out_path = oxipng::OutFile::Path {
                path: Some(output.clone()),
                preserve_attrs: false,
            };
            optimize(&in_path, &out_path, &oxipng::Options {
                strip: oxipng::StripChunks::Safe,
                ..Default::default()
            }).unwrap();
        }
    }

    // Production stuff
    #[cfg(not(debug_assertions))]
    {
        // Watch for changes in the frontend main.js and main.css
        println!("cargo:rerun-if-changed=public/main.js");
        println!("cargo:rerun-if-changed=public/main.css");
        // Run esbuild for minified output
        Command::new("node_modules/.bin/esbuild")
            .args([
                "frontend/main.ts",
                "--bundle",
                "--minify",
                "--outfile=public/main.min.js",
            ])
            .status()
            .unwrap();
        // Run tailwind for the templates for release
        Command::new("node_modules/.bin/esbuild")
            .args([
                "public/main.css",
                "--bundle",
                "--minify",
                "--outfile=public/main.min.css",
            ])
            .status()
            .unwrap();
    }
}
