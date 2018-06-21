#include "aniplotlib.cpp"
#include "wrapper.h"


void GraphChannel_delGraphChannel(GraphChannel *t) { t->~GraphChannel(); }

void GraphChannel_append_sample(GraphChannel *t, float v) { t->append_sample(v); }
void GraphChannel_append_sample_minmaxavg(GraphChannel *t, float min, float max, float avg) { t->append_sample_minmaxavg(min, max, avg); }

void GraphWidget_delGraphWidget(GraphWidget *t) { t->~GraphWidget(); }

void assign_to_cpp_string(std::string &cpps, const char* cs) { cpps.assign(cs); }
const char* get_cpp_string_value(const std::string &cpps) { return cpps.c_str(); }
