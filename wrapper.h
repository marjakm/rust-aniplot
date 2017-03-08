#include "aniplotlib.h"


GraphChannel GraphChannel_GraphChannel();
void GraphChannel_delGraphChannel(GraphChannel *t);
void GraphChannel_append_sample(GraphChannel *t, float v);
void GraphChannel_append_sample_minmaxavg(GraphChannel *t, float min, float max, float avg);

GraphVisual GraphVisual_GraphVisual();
void GraphWidget_delGraphWidget(GraphWidget *t);
void noinline_str_replace(char **p, const char *s);
