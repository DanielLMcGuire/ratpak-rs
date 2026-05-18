use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root_dir = PathBuf::from(manifest_dir);
    let target = env::var("TARGET").unwrap();
    let compat_header = root_dir.join("bridge").join("compat.h");

    let mut build = cc::Build::new();
    build.cpp(true);

    if target.contains("msvc") {
        build.flag("/std:c++17");
        build.flag(&format!("/FI{}", compat_header.display()));
    } else {
        build.flag("-std=c++17");
        build.flag("-include").flag(&compat_header.display().to_string());
    }

    build.include(root_dir.join("thirdparty"))
         .include(root_dir.join("bridge"));

    let ratpack_files = vec![
        "thirdparty/Ratpack/basex.cpp",
        "thirdparty/Ratpack/conv.cpp",
        "thirdparty/Ratpack/exp.cpp",
        "thirdparty/Ratpack/fact.cpp",
        "thirdparty/Ratpack/itrans.cpp",
        "thirdparty/Ratpack/itransh.cpp",
        "thirdparty/Ratpack/logic.cpp",
        "thirdparty/Ratpack/num.cpp",
        "thirdparty/Ratpack/rat.cpp",
        "thirdparty/Ratpack/support.cpp",
        "thirdparty/Ratpack/trans.cpp",
        "thirdparty/Ratpack/transh.cpp",
    ];

    for file in ratpack_files {
        build.file(root_dir.join(file));
    }

    build.file("bridge/bridge.cpp");
    build.compile("ratpak_bridge");

    println!("cargo:rerun-if-changed=bridge/bridge.cpp");
    println!("cargo:rerun-if-changed=bridge/bridge.h");
    println!("cargo:rerun-if-changed=bridge/compat.h");
}