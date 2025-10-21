# i2pt
This is gui application that has libi2pd statically compiled in.
I redid build system for i2pd, see crates/i2pd-sys/cmake directory,
basically now we are building libi2pd with libressl.

so crates/i2pd-sys is building libi2pd using rust cargo thing
(I made sure to support cross-compilation)

crates/i2pd is "safe" rust wrapper around it,

and i2pt uses crates/i2pd to link libi2pd in and run it in tauri project.

ensure to have installed:
    1. cmake
    1. rust
    1. c++ compiler

for android:
    ANDROID_HOME and NDK_HOME system environment variables
    android sdk and ndk (of course)

open i2pt directory and run `npm install` to install deps;
run `npm run tauri dev` to build and open the app.

## rust crates
probably? this repo can be used as package source for i2pd and i2pd-sys
packages, since I've set up cargo workspace in the root of the repo.
It is fully decoupled from main tauri project, feel free to use in
any of your projects.