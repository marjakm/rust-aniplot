extern crate imgui;
extern crate glium;
extern crate imgui_glium_renderer;

#[allow(dead_code, non_snake_case, non_camel_case_types, improper_ctypes)]
mod generated;
pub mod imgui_support { include!("../imgui-rs/examples/support/mod.rs"); }

pub use generated::root::TempContainer;

