fn main() {
    let dst = cmake::Config::new("vendor/MicroTeX")
        .build_target("LaTeX")
        .define("UNIX", "on")
        .very_verbose(true)
        .build();

    let cairo_config = pkg_config::Config::new().probe("cairomm-1.0").unwrap();
    let pango_config = pkg_config::Config::new().probe("pangomm-1.4").unwrap();
    pkg_config::Config::new().probe("fontconfig").unwrap();
    pkg_config::Config::new().probe("tinyxml2").unwrap();
    cc::Build::new()
        .file("src/ffi.cpp")
        .cpp(true)
        .includes(cairo_config.include_paths)
        .includes(pango_config.include_paths)
        .include("vendor/MicroTeX/src")
        .define("BUILD_GTK", None)
        .compile("ffi");
    println!("{:?}", cairo_config.libs);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/ffi.cpp");
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=LaTeX");
}
