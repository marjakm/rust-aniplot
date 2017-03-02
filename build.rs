extern crate gcc;
extern crate bindgen;


fn main(){
    println!("cargo:warning={}", bindgen::clang_version().full);
    let aniplot_bindings = bindgen::Builder::default()
                                            .no_unstable_rust()
                                            .enable_cxx_namespaces()
                                            .header("aniplot/aniplotlib.h")
                                            .whitelisted_type("ImVec2")
                                            .whitelisted_type("ImVec4")
                                            .whitelisted_type("ImVec2d")
                                            .whitelisted_type("MipBuf_t")
                                            .whitelisted_type("ImguiTextwrap")
                                            .whitelisted_type("GraphVisualFlags_")
                                            .whitelisted_type("PortalRect")
                                            .whitelisted_type("GraphChannel")
                                            .whitelisted_type("GraphVisual")
                                            .whitelisted_type("GraphWidget")
                                            .clang_arg("-x")
                                            .clang_arg("c++")
                                            .clang_arg("-std=c++11")
                                            .clang_arg("-Ianiplot/lib/imgui")
                                            .generate()
                                            .expect("Failed to generate aniplot bindings");
    aniplot_bindings.write_to_file("src/generated.rs")
                    .expect("Could not write aniplot bindings");
    gcc::Config::new().file("aniplot/aniplotlib.cpp")
                      .include("aniplot/lib/imgui")
                      .cpp(true)
                      .compile("libaniplot.a");
}
