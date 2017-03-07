extern crate imgui;
extern crate glium;
extern crate imgui_glium_renderer;

#[allow(dead_code, non_snake_case, non_camel_case_types, improper_ctypes)]
mod generated;
pub mod imgui_support { include!("../imgui-rs/examples/support/mod.rs"); }


pub struct Channel {
    inner: ::generated::root::GraphChannel
}

impl Channel {
    pub fn new() -> Self {
        Channel {
            inner: unsafe { ::generated::root::GraphChannel_GraphChannel() }
        }
    }

    pub fn append_sample(&mut self, value: f32) {
        unsafe { ::generated::root::GraphChannel_append_sample(&mut self.inner as _, value) }
    }

    pub fn append_sample_minmaxavg(&mut self, min: f32, max: f32, avg: f32) {
        unsafe { ::generated::root::GraphChannel_append_sample_minmaxavg(&mut self.inner as _, min, max, avg) }
    }

    pub fn set_value_limits(&mut self, min: f32, max: f32) {
        self.inner.value_min = min;
        self.inner.value_max = max;
    }
}


pub struct Visual {
    pub line_color: imgui::ImVec4,
}

impl Visual {
    pub fn new() -> Self {
        Visual {
            line_color: imgui::ImVec4::new(0.8, 0.8, 0.8, 1.0),
        }
    }

    fn drawable(&self, channel: &Channel) -> ::generated::root::GraphVisual {
        let mut vis = unsafe { ::generated::root::GraphVisual_GraphVisual() };
        vis.line_color = self.line_color.into();
        vis.graph_channel = unsafe { ::std::mem::transmute::<_, *mut _>(channel)  };
        vis
    }
}

pub fn draw_aniplot(channel: &Channel, visual: &Visual, label: imgui::ImStr, size: Option<imgui::ImVec2>) {
    let final_size = match size {
        Some(s) => s,
        None => imgui::ImVec2::new(0., 0.)
    };
    let mut final_visual = visual.drawable(channel);
    unsafe {
        let mut graph_widget = ::generated::root::GraphWidget::new();
        graph_widget.add_graph(&mut final_visual as _);
        graph_widget.DoGraph(label.as_ptr() as _, final_size.into());
    }
}


impl From<::generated::root::ImVec4> for imgui::ImVec4 {
    fn from(i: ::generated::root::ImVec4) -> Self {
        imgui::ImVec4::new(i.x, i.y, i.z, i.w)
    }
}

impl From<imgui::ImVec4> for ::generated::root::ImVec4 {
    fn from(i: imgui::ImVec4) -> Self {
        ::generated::root::ImVec4 {x:i.x, y:i.y, z:i.z, w:i.w}
    }
}

impl From<::generated::root::ImVec2> for imgui::ImVec2 {
    fn from(i: ::generated::root::ImVec2) -> Self {
        imgui::ImVec2::new(i.x, i.y)
    }
}

impl From<imgui::ImVec2> for ::generated::root::ImVec2 {
    fn from(i: imgui::ImVec2) -> Self {
        ::generated::root::ImVec2 {x:i.x, y:i.y}
    }
}
