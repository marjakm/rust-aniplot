extern crate gcc;
extern crate bindgen;


fn main(){
    println!("cargo:warning={}", bindgen::clang_version().full);
    let aniplot_bindings = bindgen::Builder::default()
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
                                            // .whitelist_function("GraphChannel_GraphChannel")
                                            .whitelist_function("GraphChannel_append_sample")
                                            .whitelist_function("GraphChannel_append_sample_minmaxavg")
                                            // .whitelist_function("GraphVisual_GraphVisual")
                                            // .whitelist_function("noinline_str_replace")
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
                                            .expect("Failed to generate aniplot bindings");

    aniplot_bindings.write_to_file("src/generated.rs")
                    .expect("Could not write aniplot bindings");
    gcc::Build::new().file("wrapper.cpp")
                      .include("aniplot")
                      .include("imgui-rs/imgui-sys/third-party/cimgui/imgui")
                      .cpp(true)
                      .flag("-std=c++11")
                      .compile("libaniplot.a");
}
