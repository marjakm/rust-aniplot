#include "aniplotlib.cpp"
#include "wrapper.h"


GraphChannel GraphChannel_GraphChannel() { return GraphChannel(); }
void GraphChannel_delGraphChannel(GraphChannel *t) { t->~GraphChannel(); }
void GraphChannel_append_sample(GraphChannel *t, float v) { t->append_sample(v); }
void GraphChannel_append_sample_minmaxavg(GraphChannel *t, float min, float max, float avg) { t->append_sample_minmaxavg(min, max, avg); }

GraphVisual GraphVisual_GraphVisual() { return GraphVisual((GraphChannel*)1); }
void GraphWidget_delGraphWidget(GraphWidget *t) { t->~GraphWidget(); }
void noinline_str_replace(char **p, const char *s) { str_replace(p, s); }
