#[macro_use] extern crate imgui;
extern crate aniplot;

use imgui::*;
use aniplot::imgui_support::Support;
use aniplot::{Channel, Visual, Widget};


const CLEAR_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 1.0);


fn main() {
    let mut support = Support::init();

    let mut channel1_1 = Channel::new();
    channel1_1.set_name(im_str!("line1"))
              .set_unit(im_str!("m/s2"))
              .set_value_limits(-4., 4.);
    let mut visual1_1 = Visual::new();
    visual1_1.line_color = ImVec4::new(1., 1., 0.8, 1.);
    visual1_1.line_width = 3.;

    let mut channel1_2 = Channel::with_name(im_str!("line2"));
    channel1_2.visual = Some(Visual::with_color(1., 0.5, 0.5, 1.0));

    let mut channel2_1 = Channel::with_name_and_unit(im_str!("line2"), im_str!("m/s"));

    let mut widget1 = Widget::new();
    let mut widget2 = Widget::new();

    let mut t: f64 = 0.;
    let dt: f64 = 1./60.;


    loop {
        t += dt;
        channel1_1.append_sample( (      t.sin() * (t*1.3).sin() +    (t* 4.3).sin()) as f32 );
        channel1_2.append_sample( ((t*12.).sin() * (t*1.1).sin() + 2.*(t*11.4).sin()) as f32 );
        channel2_1.append_sample( ((t*3.1).sin() * (t*7.3).sin() +    (t* 9.4).sin()) as f32 );

        support.render(CLEAR_COLOR, |ui| {
            ui.window(im_str!("Hello world"))
                .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
                .build(|| {
                    widget1.draw(im_str!("robota-1"), Some(ImVec2::new(0., 200.)), &[ &(&channel1_1, &visual1_1), &channel1_2 ]);
                    widget2.draw(im_str!("robota-2"), Some(ImVec2::new(0., 200.)), &[&channel2_1]);
                    ui.separator();
                    let mouse_pos = ui.imgui().mouse_pos();
                    ui.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
                })
        });

        let active = support.update_events();
        if !active {
            break;
        }
    }
}

