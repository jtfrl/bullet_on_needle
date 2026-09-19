#ifndef ZEROES_F
#define ZEROES_F

#include <float.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define frm (float)RAND_MAX
const float EPSI = 1e-8;

float numerical_der(float (*f)(float), float x);
bool signal_f(float a, float b, float (*f)(float));
float method0(float a, float b, float (*f)(float));
float sec(float a, float b, float (*f)(float)); //> função comum ao metodo ao metodo 1 e ao 4
float method1(float a, float b, float (*f)(float));
float method2(float a, float b, float(*f_or)(float), float(*phix)(float));
float method3(float a, float b, float(*f_or)(float), float(*der_f)(float));
//float method4 (TODO secante)


//TODO em 2 e 3: usar metodo de proximidade ao centro do intervalo

#endif //ZEROES_F