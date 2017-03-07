#[macro_use] extern crate imgui;
extern crate aniplot;

use imgui::*;
use aniplot::imgui_support::Support;
use aniplot::{Channel, Visual, draw_aniplot};


const CLEAR_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 1.0);


fn main() {
    let mut support = Support::init();
    let mut channel1_1 = Channel::new();
    let mut visual1_1 = Visual::new();
    let mut t: f64 = 0.;
    let dt: f64 = 1./60.;


    loop {
        t += dt;
        channel1_1.append_sample( (t.sin() * (t*1.3).sin() + (t*4.3).sin()) as f32 );
        support.render(CLEAR_COLOR, |ui| hello_world(ui, &mut channel1_1, &visual1_1));
        let active = support.update_events();
        if !active {
            break;
        }
    }
}

fn hello_world<'a>(ui: &Ui<'a>, channel1_1: &mut Channel, visual1_1: &Visual) {
    ui.window(im_str!("Hello world"))
        .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
            draw_aniplot(channel1_1, visual1_1, im_str!("robota-1"), Some(ImVec2::new(0., 200.)));
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
