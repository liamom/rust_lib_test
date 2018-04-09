extern crate cmake;

fn main() {
//    let dst = cmake::build("cpp_src");
    let dst = cmake::Config::new("cpp_src")
        .generator("MinGW Makefiles")
        .define("CMAKE_MAKE_PROGRAM", "mingw32-make")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=cpp_lib");
}
//MinGW Makefiles