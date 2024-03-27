use std::process::Command;

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
