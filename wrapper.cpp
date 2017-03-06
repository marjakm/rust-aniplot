#include "aniplotlib.cpp"
#include "wrapper.h"

TempContainer::TempContainer() : graph_visual1_1(&graph_channel1_1) {}

void TempContainer::init() {
    graph_widget1.add_graph(&graph_visual1_1);

    graph_visual1_1.line_color = ImColor(1.0f, 1.0f, 0.8f);
    graph_visual1_1.line_width = 3;
    graph_channel1_1.set_value_samplespace_mapping(ImRect(0, 0, 60, 1));
    graph_visual1_1.set_visual_valuespace_mapping(ImRect(0, -5, 10, 5));

    // visual guides that indicate min/max possible values
    graph_channel1_1.value_min = -4;
    graph_channel1_1.value_max = 4;
    str_replace(&graph_channel1_1.name, "line1");
    str_replace(&graph_channel1_1.unit, "m/s2");
}

void TempContainer::append_samples() {
    static double t;
    const float dt = 1.f/60;
    t += dt;
    float y1 = (float)(sin(t) * sin(t*1.3) + sin(t*3.4));
    graph_channel1_1.append_sample(y1);
}

void TempContainer::do_graph() {
    float graph_w = 0; // 0 to fill available space
    float graph_h = 200;
    graph_widget1.DoGraph("robota-1", ImVec2(graph_w, graph_h));
}
