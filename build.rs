use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[cfg(feature = "blas")]
#[allow(unreachable_code)]
fn cmake_blas_args() -> Option<(String, String)> {
    #[cfg(feature = "build_openblas")]
    {
        use std::io::Write;

        println!("cargo:warning=Compiling the C library. This might take a few minutes...");
        std::io::stdout().flush().unwrap();

        return Some((
            String::from("HASTY_IMPL_BUILD_OPENBLAS"),
            String::from("ON"),
        ));
    }

    #[cfg(feature = "prebuilt_openblas")]
    {
        use std::io::Write;

        println!("cargo:warning=Using prebuilt OpenBLAS library");
        std::io::stdout().flush().unwrap();

        return Some((String::from("HASTY_IMPL_GET_BLAS"), String::from("ON")));
    }

    #[cfg(feature = "blas_acml")]
    {
        return Some((String::from("HASTY_IMPL_ACML"), String::from("ON")));
    }

    #[cfg(feature = "blas_accelerate")]
    {
        return Some((String::from("HASTY_IMPL_ACCELERATE"), String::from("ON")));
    }

    #[cfg(feature = "blas_arm")]
    {
        return Some((String::from("HASTY_IMPL_ARM"), String::from("ON")));
    }

    #[cfg(feature = "blas_atlas")]
    {
        return Some((String::from("HASTY_IMPL_atlas"), String::from("ON")));
    }

    #[cfg(feature = "blas_blis")]
    {
        return Some((String::from("HASTY_IMPL_BLIS"), String::from("ON")));
    }

    #[cfg(feature = "blas_openblas")]
    {
        return Some((String::from("HASTY_IMPL_OPENBLAS"), String::from("ON")));
    }

    #[cfg(feature = "mkl")]
    {
        return Some((String::from("HASTY_IMPL_MKL"), String::from("ON")));
    }

    None
}

fn main() {
    let mut cmaker = cmake::Config::new("hasty_impl");

    #[cfg(feature = "blas")]
    {
        cmaker.define("HASTY_USE_BLAS", "ON");

        if let Ok(path) = env::var("HASTY_BLAS_PATH") {
            cmaker.define("HASTY_BLAS_PATH", path);
        }

        // Define CMake arguments based on features
        if let Some((key, value)) = cmake_blas_args() {
            println!("Key: {key}, Value: {value}");
            cmaker.define(key, value);
        }
    }

    #[cfg(feature = "opencl")]
    {
        cmaker.define("HASTY_USE_OPENCL", "ON");
    }

    let dst = cmaker.build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=hasty_impl");

    // Open the file
    let file = File::open(format!("{}/build/library_config.txt", dst.display()))
        .expect("Failed to open library_config.txt");

    // Create a buffered reader
    let reader = BufReader::new(file);

    println!("cargo:rustc-link-lib=c++");
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
            "opencl" => {
                println!("cargo:rustc-link-lib=framework=OpenCL");
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

    let mut builder = bindgen::Builder::default();

    #[cfg(feature = "blas")]
    {
        builder = builder.header("./hasty_impl/include/helper/define_blas.h");
    }

    #[cfg(feature = "opencl")]
    {
        builder = builder.header("./hasty_impl/include/helper/define_opencl.h");
    }

    builder = builder.header("./hasty_impl/include/hasty_impl.h")
        .header("./hasty_impl/include/hasty_blas.h")
        .header("./hasty_impl/include/level2/gemv.h")
        .header("./hasty_impl/include/level3/gemm.h")
        .header("./hasty_impl/include/hasty_opencl.h")
        .header("./hasty_impl/include/opencl/opencl_configure.h");

    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
