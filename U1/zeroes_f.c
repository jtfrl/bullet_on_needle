#include "zeroes_f.h"
/* ___ MÉTODOS: ____ */
/* 0 = BISSECAÇÃO
   1 = FALSA POS.
   2 = PONTO FIXO 
   3 = NEWTON-RAPHSON
   4 = SECANTE */


const float EPSI = 1e-10;

float monitor_t(clock_t start, clock_t end){
  return ((double)(end-start))/CLOCKS_PER_SEC;
}

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
  clock_t start = clock();
  printf("k=%d, x=%.5f, f(x)=%g \n\n", k, x, f(x));

  // CASO JA CONVERGIU
  if (fabs(b-a)<EPSI || fabs(f(x)) < EPSI) {
    clock_t end = clock();
    printf("\n\n execução em: %.8f s", monitor_t(start, end));
    return x;
  }

  // CASO EM QUE NÃO É MAIS POSSÍVEL REFINAR
  if(a==x || b==x){
    clock_t end = clock();
    printf("\n\n execução em: %.8f s", monitor_t(start, end));
    return x; 
  }

  // SE O SINAL MUDOU
  if (f(a) * f(x) < 0.0f) {
    return method0(a, x, f, k+1); // b <- x
  } 
  else {
    return method0(x, b, f, k+1); // a <- x
  }

  if(k>MAX_ITER){
    clock_t end = clock();
    printf("\n\n execução em: %.8f s", monitor_t(start, end));
    printf("\n\nERRO! Máximo de iterações atingidas");
    return x;
  }

}


/* ============================================================
 *  FÓRMULA DA FALSA POSIÇÃO
 * ============================================================ */
float sec(float a, float b, float (*f)(float)) {
  return (a * f(b) - b * f(a)) / (f(b) - f(a));
}

float method1(float a, float b, float (*f)(float), int k) {
  float x = sec(a, b, f);
  clock_t start = clock();
  printf("k=%d, x=%.5f, f(x)=%g \n\n", k, x, f(x));

  // CASO JA CONVERGIU
  if (fabs(b-a)<EPSI || fabs(f(x)) < EPSI) {
    clock_t end = clock();
    printf("\n\n execução em: %.8f s", monitor_t(start, end));
    return x;
  }
  // SE O SINAL MUDOU
  if (f(a) * f(x) < 0) {
    return method0(a, x, f, k+1); // b <- x
  } else {
    return method0(x, b, f, k+1); // a <- x
  }

  if(k>MAX_ITER){
    clock_t end = clock();
    printf("\n\n execução em: %.8f s", monitor_t(start, end));
    printf("\n\nERRO! Máximo de iterações atingidas");
    return x;
  }
}

//> métodos 2 e 3 contam com a estratégia "naive" de ir no meio do intervalo

float method2(float a, float b, float(*f_or)(float), float(*phix)(float)){
  float x=(a+b)/2;
  bool in_inter=true;
  float uniform=b-a; //deixamos o intervalo em dist. uniforme
  int k=0;

  /* === histórico dos registros === */
  float hist_x[MAX_ITER+50];
  float hist_fx[MAX_ITER+50];
  hist_x[0]=x;
  hist_fx[0]=f_or(x);

  clock_t start = clock();

  while((fabs(f_or(x))>=EPSI) && k<MAX_ITER){
    if(phix(x)!=0) x=phix(x); //> função de menor grau que f_or 
                              //> que vai ser usada para iterar 
                              //> e obter o valor da raiz

    if(phix(x)<a || phix(x)>b) in_inter=false;
    if(!in_inter){ 
      //x=a+uniform*((float)rand()/(float)RAND_MAX);
      x=a+uniform*((float)rand()/frm);
    }
    k++;
    //adcionando histórico
    hist_x[k]=x;
    hist_fx[k]=f_or(x);

    if(fabs(phix(x)-x)<EPSI){
       printf("EXECUÇÃO PAROU");
       break;    
    }   
  }
  clock_t end = clock();

  for(int i=0;i<=k;i++)  printf("k=%d, x=%.5f, f(x)=%g \n\n", i, hist_x[i], hist_fx[i]);

  printf("\n\n execução em: %.8f s", monitor_t(start, end));

  if(k>=MAX_ITER){
    printf("\n\nERRO! Máximo de iterações atingidas");
  }
  
  return x;
}


float method3(float a, float b, float(*f_or)(float), float(*der_f)(float)){ 
  float x=(a+b)/2; 

  clock_t start = clock();
  int k=0;
  printf("k=%d, x=%.5f, f(x)=%g \n\n", k, x, f_or(x));

  while(fabs(f_or(x))>EPSI){
    if(der_f(x)!=0 && numerical_der(f_or, x)!=0){
      x=x-(f_or(x))/(der_f(x));
    }

    k++;
    printf("k=%d, x=%.5f, f(x)=%g \n\n", k, x, f_or(x));
  }

  clock_t end = clock();
  printf("\n\n execução em: %.8f s", monitor_t(start, end));

  return x;


  if(k>MAX_ITER){
    clock_t end = clock();
    printf("\n\n execução em: %.8f s", monitor_t(start, end));
    printf("\n\n ERRO! Máximo de iterações atingidas");
    return x;
  }
}



float method4(float a, float b, float (*f)(float)) {
    float x_ant = a, x = b;
    float f_ant = f(x_ant), fx = f(x);

    clock_t start = clock();
    printf("k=0, x=%.5f, f(x)=%g\n", x_ant, f_ant);
    printf("k=1, x=%.5f, f(x)=%g\n", x, fx);

    for (int k = 2; k < MAX_ITER; k++) {
        if (fabsf(fx - f_ant) < 1e-15f) {
            printf("Divisao por zero na secante, parando.\n");
            break;
        }
        //float x_new = x - fx * (x - x_ant) / (fx - f_ant);
        float x_new=sec(x_ant, x, f);
        printf("k=%d, x=%.5f, f(x)=%g\n", k, x_new, f(x_new));

        if (fabs(f(x_new)) < EPSI){
            clock_t end = clock();
            printf("\n\n execução em: %.8f s", monitor_t(start, end));
            return x_new;
        }
        
        x_ant = x;  
        f_ant = fx;
        x = x_new;  
        fx = f(x_new);
    }

    clock_t end = clock();
    printf("\n\n execução em: %.8f s", monitor_t(start, end));
    return x;
}

float method5(float a, float b,
                      float (*f)(float), float (*df)(float)) { //híbrido
    int k = 0;
    float x = (a + b) / 2.0;
    double erro = (b - a) / 2.0;

    clock_t start=clock();
    printf("\n=== ESTRATEGIA HIBRIDA: Bisseccao + Newton ===\n");
    /* Fase 1: Bissecção com tolerância 1e-2 (apenas para aquecer) */
    while (erro > 1e-2 && k < 15) {
        if (f(a) * f(x) < 0) b = x;
        else                 a = x;
        x = (a + b) / 2.0;
        erro = (b - a) / 2.0;
        k++;
    }
    printf("Fase 1 (Bisseccao): %d iteracoes, chute x0 = %.10f\n", k, x);

    /* Fase 2: Newton a partir do chute obtido */
    clock_t end = clock();
    printf("\n\n execução em: %.8f s", monitor_t(start, end));
    return method3(x - 0.5, x + 0.5, f, df);

}
