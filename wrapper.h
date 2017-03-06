#include "aniplotlib.h"


struct TempContainer {
    GraphWidget graph_widget1;
    GraphChannel graph_channel1_1;
    GraphVisual graph_visual1_1;
    TempContainer();
    void init();
    void append_samples();
    void do_graph();
};

