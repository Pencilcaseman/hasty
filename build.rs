use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[allow(unreachable_code)]
fn cmake_blas_args() -> Option<(String, String)> {
    #[cfg(feature = "build_openblas")]
    {
        use std::io::Write;

        println!("cargo:warning=Compiling the C library. This might take a few minutes...");
        std::io::stdout().flush().unwrap();

        return Some((
            String::from("HASTY_BLAS_C_BUILD_OPENBLAS"),
            String::from("ON"),
        ));
    }

    #[cfg(feature = "prebuilt_openblas")]
    {
        use std::io::Write;

        println!("cargo:warning=Using prebuilt OpenBLAS library");
        std::io::stdout().flush().unwrap();

        return Some((String::from("HASTY_BLAS_C_GET_BLAS"), String::from("ON")));
    }

    return None;
}

fn main() {
    let mut cmaker = cmake::Config::new("hasty_blas_c");

    if let Ok(path) = env::var("HASTY_BLAS_PATH") {
        cmaker.define("HASTY_BLAS_PATH", path);
    }

    // Define CMake arguments based on features
    if let Some((key, value)) = cmake_blas_args() {
        cmaker.define(key, value);
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
