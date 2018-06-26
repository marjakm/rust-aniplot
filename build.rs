extern crate gcc;
extern crate bindgen;

use std::env;
use std::path::Path;


fn main(){
    let out_dir = env::var("OUT_DIR").expect("No OUT_DIR env variable");
    let bind_fle = Path::new(&out_dir).join("generated.rs");

    println!("cargo:warning={}", bindgen::clang_version().full);
    bindgen::Builder::default()
        .enable_cxx_namespaces()
        .layout_tests(false)
        .header("wrapper.h")
        .opaque_type("std::.*")
        .opaque_type("MipBuf_t")
        .whitelist_type("ImVec2")
        .whitelist_type("ImVec4")
        .whitelist_type("ImVec2d")
        .whitelist_type("MipBuf_t")
        .whitelist_type("ImguiTextwrap")
        .whitelist_type("GraphVisualFlags_")
        .whitelist_type("PortalRect")
        .whitelist_type("GraphChannel")
        .whitelist_type("GraphVisual")
        .whitelist_type("GraphWidget")
        .whitelist_function("GraphChannel_append_sample")
        .whitelist_function("GraphChannel_append_sample_minmaxavg")
        .whitelist_function("GraphChannel_delGraphChannel")
        .whitelist_function("GraphWidget_delGraphWidget")
        .whitelist_function("assign_to_cpp_string")
        .whitelist_function("get_cpp_string_value")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        .clang_arg("-Iimgui-rs/imgui-sys/third-party/cimgui/imgui")
        .clang_arg("-Ianiplot")
        .generate()
        .expect("Failed to generate aniplot bindings")
        .write_to_file(bind_fle)
        .expect("Could not write aniplot bindings");

    gcc::Build::new().file("wrapper.cpp")
                      .include("aniplot")
                      .include("imgui-rs/imgui-sys/third-party/cimgui/imgui")
                      .cpp(true)
                      .flag("-std=c++11")
                      .compile("libaniplot.a");
}
