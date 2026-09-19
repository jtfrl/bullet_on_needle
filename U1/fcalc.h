#ifndef FCALC
#define FCALC

#include <float.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

float* gerar_interval(float a, float b, float step, int *out_size);
float* obter_val_f(float (*f)(float), size_t tam, float* vx);
void show_val_f(float* vy, float* vx, int pos, int tam);


#endif //FCALC