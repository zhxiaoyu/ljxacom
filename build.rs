use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    if target_os == "linux" {
        // Linux 构建
        cc::Build::new()
            .cpp(true)
            .flag("-fPIC")
            .flag("-g")
            .flag("-Wall")
            .flag("-pthread")
            .include("src/clib/linux/include")
            .file("src/clib/linux/libsrc/ProfileDataConvert.cpp")
            .file("src/clib/linux/libsrc/LJX8_IF_Linux.cpp")
            .file("src/clib/linux/libsrc/LJXA_ACQ.cpp")
            .compile("ljxacom");
        println!("cargo:rustc-link-lib=ljxacom");
        // Linux 系统使用 bindgen 生成绑定
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        let bindings_path = out_path.join("bindings.rs");

        let output = Command::new("bindgen")
            .arg("src/clib/wrapper.hpp")
            .arg("--output")
            .arg(&bindings_path)
            .arg("--")
            .arg("-x")
            .arg("c++")
            .arg("-I")
            .arg("src/clib/linux/include")
            .output()
            .expect("无法执行 bindgen");

        if !output.status.success() {
            panic!("Bindgen 失败: {}", String::from_utf8_lossy(&output.stderr));
        }
    } else if target_os == "windows" {
        // Windows 构建
        let arch_dir = if target_arch == "x86_64" {
            "x64"
        } else {
            "x86"
        };

        cc::Build::new()
        .cpp(true)
        .define("LJXA_ACQ_API_EXPORT", None)
        .include("src/clib/windows/include")
        .file("src/clib/windows/libsrc/LJXA_ACQ.cpp")
        .compile("LJXA_ACQ");
        println!("cargo:rustc-link-lib=LJXA_ACQ");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        let bindings_path = out_path.join("bindings.rs");

        let output = Command::new("bindgen")
            .arg("src/clib/wrapper.hpp")
            .arg("--output")
            .arg(&bindings_path)
            .arg("--")
            .arg("-x")
            .arg("c++")
            .output()
            .expect("无法执行 bindgen");

        if !output.status.success() {
            panic!("Bindgen 失败: {}", String::from_utf8_lossy(&output.stderr));
        }

        println!("cargo:rustc-link-search=src/clib/windows/{}", arch_dir);
        println!("cargo:rustc-link-lib=LJX8_IF");

        // 复制 LJX8_IF.dll 到 target 目录
        let target_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("target")
            .join(if cfg!(debug_assertions) {
                "debug"
            } else {
                "release"
            });
        let dll_src = PathBuf::from(format!("src/clib/windows/{}/LJX8_IF.dll", arch_dir));
        let dll_dest = target_dir.join("LJX8_IF.dll");

        if let Err(e) = std::fs::copy(&dll_src, &dll_dest) {
            panic!("无法复制 LJX8_IF.dll 到 target 目录: {}", e);
        }

        println!(
            "cargo:rerun-if-changed=src/clib/windows/{}/LJX8_IF.dll",
            arch_dir
        );
    } else {
        panic!("不支持的操作系统");
    }
}
