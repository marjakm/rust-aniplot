extern crate imgui;
extern crate glium;
extern crate imgui_glium_renderer;

#[allow(dead_code, non_snake_case, non_camel_case_types, improper_ctypes)]
mod generated;
pub mod imgui_support { include!("../imgui-rs/examples/support/mod.rs"); }

use std::borrow::Borrow;


pub struct Channel {
    inner: ::generated::root::GraphChannel,
    pub visual: Option<Visual>,
}

impl Channel {
    pub fn new() -> Self {
        Channel {
            inner: unsafe { ::generated::root::GraphChannel_GraphChannel() },
            visual: None,
        }
    }

    pub fn with_name(name: imgui::ImStr) -> Self {
        let mut s = Self::new();
        s.set_name(name);
        s
    }

    pub fn with_name_and_unit(name: imgui::ImStr, unit: imgui::ImStr) -> Self {
        let mut s = Self::new();
        s.set_name(name);
        s.set_unit(unit);
        s
    }

    pub fn set_name(&mut self, name: imgui::ImStr) -> &mut Self {
        unsafe { ::generated::root::noinline_str_replace(&mut self.inner.name, name.as_ptr()); }
        self
    }

    pub fn set_unit(&mut self, unit: imgui::ImStr) -> &mut Self {
        unsafe { ::generated::root::noinline_str_replace(&mut self.inner.unit, unit.as_ptr()); }
        self
    }

    pub fn set_value_limits(&mut self, min: f32, max: f32) -> &mut Self {
        self.inner.value_min = min;
        self.inner.value_max = max;
        self
    }

    pub fn append_sample(&mut self, value: f32) -> &mut Self {
        unsafe { ::generated::root::GraphChannel_append_sample(&mut self.inner, value); }
        self
    }

    pub fn append_sample_minmaxavg(&mut self, min: f32, max: f32, avg: f32) -> &mut Self {
        unsafe { ::generated::root::GraphChannel_append_sample_minmaxavg(&mut self.inner, min, max, avg); }
        self
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        unsafe { ::generated::root::GraphChannel_delGraphChannel(&mut self.inner) }
    }
}

pub struct Visual {
    pub line_color: imgui::ImVec4,
    pub line_width: f32,
}

impl Visual {
    pub fn new() -> Self {
        Visual {
            line_color: imgui::ImVec4::new(0.8, 0.8, 0.8, 1.0),
            line_width: 1.,
        }
    }

    pub fn with_color(r: f32, g: f32, b: f32, a: f32) -> Self {
        Visual {
            line_color: imgui::ImVec4::new(r,g,b,a),
            line_width: 1.,
        }
    }

    pub fn with_color_and_width(r: f32, g: f32, b: f32, a: f32, width: f32) -> Self {
        Visual {
            line_color: imgui::ImVec4::new(r,g,b,a),
            line_width: width,
        }
    }

    fn patch_graph_visual(&self, channel: &Channel, vis: &mut ::generated::root::GraphVisual) {
        vis.line_color = self.line_color.into();
        vis.line_width = self.line_width;
        vis.graph_channel = unsafe { ::std::mem::transmute::<_, *mut _>(channel)  };
    }
}

pub struct Widget {
    visual: ::generated::root::GraphVisual,
}

impl Widget {
    pub fn new() -> Self {
        Widget {
            visual: unsafe { ::generated::root::GraphVisual_GraphVisual() }
        }
    }

    pub fn draw(&mut self, label: imgui::ImStr, size: Option<imgui::ImVec2>, drawables: &[&Drawable]) {
        let final_size = match size {
            Some(s) => s,
            None => imgui::ImVec2::new(0., 0.)
        };
        let mut tmp_visuals = Vec::with_capacity(drawables.len());
        for d in drawables {
            let mut vis = self.visual.clone();
            d.drawable(&mut vis);
            tmp_visuals.push(vis);
        }
        unsafe {
            let mut graph_widget = ::generated::root::GraphWidget::new();
            for vis in &mut tmp_visuals[..] {
                graph_widget.add_graph(vis);
            }
            graph_widget.DoGraph(label.as_ptr(), final_size.into());
            ::generated::root::GraphWidget_delGraphWidget(&mut graph_widget);
        }
        if let Some(v) = tmp_visuals.into_iter().next() {
            self.visual = v;
        }
    }
}

pub trait Drawable {
    fn drawable(&self, vis: &mut ::generated::root::GraphVisual);
}

impl<C: Borrow<Channel>, V: Borrow<Visual>> Drawable for (C, V) {
    fn drawable(&self, vis: &mut ::generated::root::GraphVisual) {
        self.1.borrow().patch_graph_visual(self.0.borrow(), vis);
    }
}

impl<C: Borrow<Channel>> Drawable for C {
    fn drawable(&self, vis: &mut ::generated::root::GraphVisual) {
        let bs = self.borrow();
        match bs.visual {
            Some(ref v) => v.patch_graph_visual(bs, vis),
            None => Visual::new().patch_graph_visual(bs, vis),
        }
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
