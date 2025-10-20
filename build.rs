// build.rs
use cmake::Config;
use bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cmake_dir = manifest_dir.join("cmake");

    let mut forborrowchecker = Config::new(&cmake_dir);
    let mut config = forborrowchecker
        .define("WITH_LIBRARY", "ON")
        .define("WITH_BINARY", "OFF")
        .define("BUILD_TESTING", "OFF")
        .very_verbose(true);

    // Android-specific configuration
    if target.contains("android") {
        let ndk = env::var("ANDROID_NDK").unwrap_or(
            env::var("NDK_HOME")
            .expect("ANDROID_NDK or NDK_HOME must be set"));
        let abi = if target.contains("aarch64") {
            "arm64-v8a"
        } else if target.contains("armv7") {
            "armeabi-v7a"
        } else if target.contains("x86_64") {
            "x86_64"
        } else if target.contains("i686") {
            "x86"
        } else {
            panic!("Unknown Android ABI for target: {}", target);
        };

        let platform = env::var("ANDROID_PLATFORM").unwrap_or_else(|_| "21".to_string());

        config = config
            .define("CMAKE_TOOLCHAIN_FILE", format!("{}/build/cmake/android.toolchain.cmake", ndk))
            .define("ANDROID_ABI", abi)
            .define("ANDROID_PLATFORM", format!("android-{}", platform))
            .define("ANDROID_STL", "c++_static");
    } else {
        // Non-Android platforms: use default toolchain
        config = config
            .env("MAKE", "make")
            .env("PATH", format!("{}:{}", env::var("PATH").unwrap_or_default(), cmake_dir.display()));
    }

    let dst = config.build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/build/_deps/libressl-build/crypto", dst.display());
    println!("cargo:rustc-link-search=native={}/build/_deps/libressl-build/ssl", dst.display());

    println!("cargo:rustc-link-lib=static=i2pd");
    println!("cargo:rustc-link-lib=static=ssl");
    println!("cargo:rustc-link-lib=static=crypto");
    println!("cargo:rustc-link-lib=static=boost_program_options");

    // Link C++ standard library
    if target.contains("android") {
        println!("cargo:rustc-link-lib=static=c++_static");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }

    // Link zlib statically on all non-Windows platforms, including Android
    if target.contains("windows") {
        println!("cargo:rustc-link-lib=static=zlibstatic");
    } else {
        println!("cargo:rustc-link-lib=static=z");
    }

    println!("cargo:include={}/include", dst.display());

    // Generate Rust bindings from the C API header
    let bindings = bindgen::Builder::default()
        .header(format!("{}/include/i2pd/libi2pd_wrapper/capi.h", dst.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindi2pd.rs")
        .expect("Couldn't write bindings!");
}