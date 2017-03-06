#[macro_use] extern crate glium;
#[macro_use] extern crate imgui;
extern crate imgui_glium_renderer;
extern crate aniplot;

mod imgui_support;

use imgui::*;
use self::imgui_support::Support;
use aniplot::TempContainer;

const CLEAR_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 1.0);

fn main() {
    let mut support = Support::init();

    loop {
        support.render(CLEAR_COLOR, hello_world);
        let active = support.update_events();
        if !active {
            break;
        }
    }
}

fn hello_world<'a>(ui: &Ui<'a>) {
    ui.window(im_str!("Hello world"))
        .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
            // let mut tmp = unsafe { TempContainer::new() };
            ui.text(im_str!("Hello world!"));
            ui.text(im_str!("This...is...imgui-rs!"));
            if ui.button(im_str!("Tere"), ImVec2::new(100.,50.)) {
                println!("Tere vajutatud");
            }
            ui.separator();
            let mouse_pos = ui.imgui().mouse_pos();
            ui.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
        })
}
