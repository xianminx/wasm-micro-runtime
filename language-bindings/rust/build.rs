use bindgen;
use std::env;
use std::fs;
use std::path::PathBuf;
////////////////////////////////
/// Generate Rust bindings for the C++ API by consuming the C++ header files defined at
///  "../../core/iwasm/include/".
fn main() {
    let building_dir;
    let library_name;
    if cfg!(target_os = "linux") {
        building_dir = "../../product-mini/platforms/linux/build";
        library_name = "libiwasm.so";
    } else if cfg!(target_os = "windows") {
        building_dir = "../../product-mini/platforms/windows/build";
        library_name = "iwasm.dll";
    } else if cfg!(target_os = "macos") {
        building_dir = "../../product-mini/platforms/darwin/build";
        library_name = "libiwasm.dylib";
    } else if cfg!(target_os = "ios") {
        building_dir = "../../product-mini/platforms/darwin/build";
        library_name = "libiwasm.dylib";
    } else if cfg!(target_os = "android") {
        building_dir = "../../product-mini/platforms/android/build";
        library_name = "libiwasm.so";
    } else {
        panic!("Unsupported target OS!"); 
    }



    println!("cargo:rustc-link-search={}/{}", building_dir, library_name);
    println!("cargo:rustc-link-lib=iwasm");
    let mut builder = bindgen::Builder::default();

    let include_header_path = "../../core/iwasm/include/";
    let header_files = fs::read_dir(include_header_path).unwrap();
    for header in header_files {
        let header_path = header.unwrap().path();
        let header_file_name = header_path.file_name().unwrap().to_str().unwrap();
        if header_file_name.ends_with(".h") {
            println!("cargo:rerun-if-changed={}", header_path.to_str().unwrap());
            builder = builder.header(header_path.to_str().unwrap());
        }
    }

    let bindings = builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
