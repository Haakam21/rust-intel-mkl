fn main() {
    println!("cargo:rustc-link-search=/opt/intel/oneapi/mkl/2024.0/lib");

    println!("cargo:rustc-link-lib=mkl_intel_ilp64");
    println!("cargo:rustc-link-lib=mkl_intel_thread");
    println!("cargo:rustc-link-lib=mkl_core");
    println!("cargo:rustc-link-lib=iomp5");

    let bindings = bindgen::Builder::default()
        .header("lib/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Failed to generate bindings");

    bindings
        .write_to_file("src/sys.rs")
        .expect("Failed to write bindings");
}
