/* #include <float.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h> */

#include "zeroes_f.h"
/* ___ MÉTODOS: ____ */
/* 0 = BISSECAÇÃO
   1 = FALSA POS.
   2 = PONTO FIXO 
   3 = NEWTON-RAPHSON*/


const float EPSI = 1e-5;

float numerical_der(float (*f)(float), float x) {
  const float h = x < 0 ? -FLT_MAX : x + (-FLT_MAX);
  return ((f(x + h) - f(x)) / h);
}

// TODO: função para obter maior intervalo f(x)
// para poder comparar com der. de phix(x)


// assumes that the interval already have a signal change
// mudando para signal_f
bool signal_f(float a, float b, float (*f)(float)) {
  if (numerical_der(f, a) * numerical_der(f, b) < 0)
    return false;
  if (numerical_der(f, a) * numerical_der(f, (b + a) / 2) < 0)
    return false;
  return true;
}

float method0(float a, float b, float (*f)(float), int k) {
  float x = (b + a) / 2.0f;
  printf("k=%d, x=%.5f, f(x)=%g \n\n", k, x, f(x));
  // CASO JA CONVERGIU
  if (fabs(b-a)<EPSI || fabs(f(x)) < EPSI) {
    return x;
  }

  // CASO EM QUE NÃO É MAIS POSSÍVEL REFINAR
  if(a==x || b==x) return x; 
  // SE O SINAL MUDOU
  if (f(a) * f(x) < 0.0f) {
    return method0(a, x, f, k+1); // b <- x
  } 
  else {
    return method0(x, b, f, k+1); // a <- x
  }

}

float sec(float a, float b, float (*f)(float)) {
  return ((a * f(b) - b * f(a)) / f(b) - f(a));
}

float method1(float a, float b, float (*f)(float), int k) {
  float x = sec(a, b, f);
  printf("k=%d, x=%.5f, f(x)=%g \n\n", k, x, f(x));
  // CASO JA CONVERGIU
  if (fabs(b-a)<EPSI || fabs(f(x)) < EPSI) {
    return x;
  }
  // SE O SINAL MUDOU
  if (f(a) * f(x) < 0) {
    return method0(a, x, f, k+1); // b <- x
  } else {
    return method0(x, b, f, k+1); // a <- x
  }
}

//> métodos 2 e 3 contam com a estratégia "naive" de ir no meio do intervalo

float method2(float a, float b, float(*f_or)(float), float(*phix)(float)){
  float x=(a+b)/2;
  bool in_inter=true;
  float uniform=b-a; //deixamos o intervalo em dist. uniforme

  int k=0;
  printf("k=%d, x=%.5f, f(x)=%g \n\n", k, x, f_or(x));

  while(fabs(f_or(x)>EPSI)){
    if(phix(x)!=0) x=phix(x); //> função de menor grau que f_or 
                              //> que vai ser usada para iterar 
                              //> e obter o valor da raiz

    if(phix(x)>a || phix(x)<b) in_inter=false;
    if(!in_inter){ 
      //x=a+uniform*((float)rand()/(float)RAND_MAX);
      x=a+(float)rand()/frm/(uniform);
    }
    // TODO aplicar método de verificação de máximo de f(x) vs. der_phix
    
    k++;
    printf("k=%d, x=%.5f, f(x)=%g \n\n", k, x, f_or(x));
  
  }
  return x;
}


float method3(float a, float b, float(*f_or)(float), float(*der_f)(float)){
  // controle com numerical_der
 
  float x=(a+b)/2; 

  int k=0;
  printf("k=%d, x=%.5f, f(x)=%g \n\n", k, x, f_or(x));
  
  while(fabs(f_or(x))>EPSI){
    if(der_f(x)!=0 && numerical_der(f_or, x)!=0){
      x=x-(f_or(x))/(der_f(x));
    }

    k++;
    printf("k=%d, x=%.5f, f(x)=%g \n\n", k, x, f_or(x));
  }

  return x;
}
