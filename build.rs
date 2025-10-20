// build.rs
use cmake::Config;
use bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cmake_dir = manifest_dir.join("cmake");

    let mut config = Config::new(&cmake_dir);
    config
        .define("WITH_LIBRARY", "ON")
        .define("WITH_BINARY", "OFF")
        .define("BUILD_TESTING", "OFF")
        .very_verbose(true);

    if target.contains("android") {
        // Accept either ANDROID_NDK or NDK_HOME
        let ndk = env::var("ANDROID_NDK")
            .or_else(|_| env::var("NDK_HOME"))
            .expect("Either ANDROID_NDK or NDK_HOME must be set");

        let abi = if target.contains("aarch64") {
            "arm64-v8a"
        } else if target.contains("armv7") {
            "armeabi-v7a"
        } else if target.contains("x86_64") {
            "x86_64"
        } else if target.contains("i686") {
            "x86"
        } else {
            panic!("Unsupported Android target: {}", target);
        };

        // Configure CMake to use Android NDK toolchain
        config
            .define("CMAKE_TOOLCHAIN_FILE", format!("{}/build/cmake/android.toolchain.cmake", ndk))
            .define("ANDROID_ABI", abi)
            .define("ANDROID_PLATFORM", "24")
            .define("ANDROID_STL", "c++_static");

        // Linker flags for Rust
        let host_tag = match std::env::consts::OS {
            "linux" => "linux-x86_64",
            "macos" => "darwin-x86_64",
            "windows" => "windows-x86_64",
            _ => panic!("Unsupported host OS for Android NDK: {}", std::env::consts::OS),
        };

        let cxx_lib_path = format!(
            "{}/toolchains/llvm/prebuilt/{}/sysroot/usr/lib/{}/24",
            ndk, host_tag, abi
        );

        let ndk_bin = format!("{}/toolchains/llvm/prebuilt/{}/bin", ndk, host_tag);
        config
            .env("PATH", format!("{}:{}", env::var("PATH").unwrap_or_default(), ndk_bin));

        println!("cargo:rustc-link-search=native={}", cxx_lib_path);
        println!("cargo:rustc-link-lib=c++_static");
        println!("cargo:rustc-link-lib=log");
        println!("cargo:rustc-link-lib=dl");
    } else {
        // Non-Android: keep original env setup
        config
            .env("MAKE", "make")
            .env("PATH", format!("{}:{}", env::var("PATH").unwrap_or_default(), cmake_dir.display()));
    }

    let dst = config.build();

    // Link i2pd and its dependencies
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/build/_deps/libressl-build/crypto", dst.display());
    println!("cargo:rustc-link-search=native={}/build/_deps/libressl-build/ssl", dst.display());

    println!("cargo:rustc-link-lib=static=i2pd");
    println!("cargo:rustc-link-lib=static=ssl");
    println!("cargo:rustc-link-lib=static=crypto");
    println!("cargo:rustc-link-lib=static=boost_program_options");

    // C++ standard library — already handled for Android above
    if !target.contains("android") {
        println!("cargo:rustc-link-lib=stdc++");
    }

    // Zlib
    if target.contains("windows") {
        println!("cargo:rustc-link-lib=static=zlibstatic");
    } else {
        println!("cargo:rustc-link-lib=static=z");
    }

    println!("cargo:include={}/include", dst.display());

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header(format!("{}/include/i2pd/libi2pd_wrapper/capi.h", dst.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindi2pd.rs")
        .expect("Couldn't write bindings!");
}