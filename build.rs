use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() {
    let mut cmaker = cmake::Config::new("hasty_blas_c");

    // Read HASTY_BLAS_PATH environment variable if it exists
    if let Ok(path) = env::var("HASTY_BLAS_PATH") {
        // panic!("Defined");
        cmaker.define("HASTY_BLAS_PATH", path);
    }

    let dst = cmaker.build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=hasty_blas_c");

    // Open the file
    let file = File::open(format!("{}/build/blas_config.txt", dst.display()))
        .expect("Failed to open blas_libraries.txt");

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line
            .expect("Could not read line from file")
            .trim()
            .to_string();

        match line.as_str() {
            "accelerate" => {
                println!("cargo:rustc-link-lib=framework=Accelerate");
            }
            _ => {
                // println!("cargo:rustc-link-lib={}", line);

                // Get path and filename
                let path = PathBuf::from(line);
                let filename = path.file_name().unwrap().to_str().unwrap();

                // Strip 'lib' prefix and any extension
                let filename = filename.strip_prefix("lib").unwrap_or(filename);
                let filename = filename.split(".").next().unwrap_or(filename);

                println!(
                    "cargo:rustc-link-search=native={}",
                    path.parent().unwrap().display()
                );
                println!("cargo:rustc-link-lib=static={}", filename);
            }
        }
    }

    let bindings = bindgen::Builder::default()
        .header("./hasty_blas_c/include/hasty_blas_c.h")
        .header("./hasty_blas_c/include/level3/gemm.h")
        .header("./hasty_blas_c/include/level2/gemv.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
