#include "aniplotlib.h"


void GraphChannel_delGraphChannel(GraphChannel *t);
void GraphChannel_append_sample(GraphChannel *t, float v);
void GraphChannel_append_sample_minmaxavg(GraphChannel *t, float min, float max, float avg);

void GraphWidget_delGraphWidget(GraphWidget *t);

void assign_to_cpp_string(std::string &cpps, const char* cs);
const char* get_cpp_string_value(const std::string &cpps);
