#[macro_use] extern crate imgui;
extern crate aniplot;

use imgui::*;
use aniplot::TempContainer;
use aniplot::imgui_support::Support;


const CLEAR_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 1.0);


fn main() {
    let mut support = Support::init();
    let mut tmp = unsafe { TempContainer::new() };
    unsafe { tmp.init() };

    loop {
        support.render(CLEAR_COLOR, |ui| hello_world(ui, &mut tmp));
        let active = support.update_events();
        if !active {
            break;
        }
    }
}

fn hello_world<'a>(ui: &Ui<'a>, tmp: &mut TempContainer) {
    ui.window(im_str!("Hello world"))
        .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
            unsafe { tmp.append_samples() };
            unsafe { tmp.do_graph() };
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
