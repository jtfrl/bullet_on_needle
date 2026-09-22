#ifndef ZEROES_F
#define ZEROES_F

#include <float.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define frm (float)RAND_MAX
extern const float EPSI; // usamos um extern para evitar redefinições
#define MAX_ITER 100      /* limite de iterações para evitar loop infinito */


float numerical_der(float (*f)(float), float x);
bool signal_f(float a, float b, float (*f)(float));
float method0(float a, float b, float (*f)(float), int k);
float sec(float a, float b, float (*f)(float)); //> função comum ao metodo ao metodo 1 e ao 4
float method1(float a, float b, float (*f)(float), int k);
float method2(float a, float b, float(*f_or)(float), float(*phix)(float));
float method3(float a, float b, float(*f_or)(float), float(*der_f)(float));
float method4(float a, float b, float (*f)(float));          /* Secante         */
float method5(double a, double b,
                      double (*f)(double), double (*df)(double));//híbrido
float monitor_t(clock_t start, clock_t end);


//TODO em 2 e 3: usar metodo de proximidade ao centro do intervalo

#endif //ZEROES_F