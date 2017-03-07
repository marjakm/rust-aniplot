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


/////////// REAL STUFF ////////////////////////

GraphChannel GraphChannel_GraphChannel();
void GraphChannel_append_sample(GraphChannel *t, float v);
void GraphChannel_append_sample_minmaxavg(GraphChannel *t, float min, float max, float avg);

GraphVisual GraphVisual_GraphVisual();
