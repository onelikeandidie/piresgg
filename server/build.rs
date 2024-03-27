use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=templates/");
    // Run tailwind for the templates for debug
    #[cfg(debug_assertions)]
    {
        Command::new("node_modules/.bin/tailwindcss")
            .args(["-i", "frontend/tailwind.css", "-o", "public/main.css"])
            .status()
            .unwrap();
    }
    // Run tailwind for the templates for release
    #[cfg(not(debug_assertions))]
    {
        Command::new("node_modules/.bin/tailwindcss")
            .args([
                "-i",
                "frontend/tailwind.css",
                "-o",
                "public/main.css",
                "--minify",
            ])
            .status()
            .unwrap();
    }

    println!("cargo:rerun-if-changed=frontend/main.ts");
    // Run esbuild for the frontend for debug
    #[cfg(debug_assertions)]
    {
        Command::new("node_modules/.bin/esbuild")
            .args(["frontend/main.ts", "--bundle", "--sourcemap", "--outfile=public/main.js"])
            .status()
            .unwrap();
    }
    // Run esbuild for the frontend for release
    #[cfg(not(debug_assertions))]
    {
        Command::new("node_modules/.bin/esbuild")
            .args([
                "frontend/main.ts",
                "--bundle",
                "--minify",
                "--outfile=public/main.js",
            ])
            .status()
            .unwrap();
    }
}
