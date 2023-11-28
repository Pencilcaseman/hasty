use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let dst = cmake::Config::new("hasty_blas_c").build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=hasty_blas_c");

    // Open the file
    let file = File::open(format!("{}/build/blas_config.txt", dst.display()))
        .expect("Failed to open blas_libraries.txt");

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line.expect("Could not read line from file").trim().to_string();

        match line.as_str() {
            "accelerate" => {
                println!("cargo:rustc-link-lib=framework=Accelerate");
            }
            _ => {
                panic!("Unknown BLAS library: {}", line);
            }
        }
    }

    let bindings = bindgen::Builder::default()
        .header("./hasty_blas_c/include/hasty_blas_c.h")
        .header("./hasty_blas_c/include/level3/gemm.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
