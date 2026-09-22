#include "fcalc.h"
#include "zeroes_f.h"

float* gerar_interval(float a, float b, float step, int *out_size){
    // out_size avisa que deve parar se chegar em zero
    if(step == 0.0f || (a<b && step<0.0f) || (a>b && step>0.0f)){
        *out_size=0;
        return NULL;
    }

    int tam=(int)((b-a)/step)+1;
    if(tam<=0){
        *out_size=0;   
        return NULL;
    }

    float *arr=(float*)malloc(tam * sizeof(float));
    if(arr==NULL){
        *out_size=0;
        return NULL;
    }

    for(int i=0; i<tam; i++) arr[i]=a+(step*i);
    *out_size=tam;

    return arr;
}

float* obter_val_f(float (*f)(float), size_t tam, float* vx){
    float *vy=(float*)malloc(tam *sizeof(float));

    for(int i=0; i<tam; i++){
        vy[i]=f(vx[i]);
    }

    return vy;
}

// será útil para ver em entre que valores estaria uma raiz da função
void show_val_f(float* vy, float* vx, int pos, int tam){
    char formulas[4][100]={"(2*pow(x,4)+4*pow(x,3)+3*pow(x,2)-10*x-15)",
            "(pow(x,5)-2*pow(x,4)-9*pow(x,3)+22*pow(x,2)+4*x-24)",
            "(5*pow(x,3)+pow(x,2)-pow(e, (1-2*x))+cos(x)+20)",
            "(sin(x)*x+4)"};

    printf("\n\n :::: VALORES DE %s ::::", formulas[pos]);
    for(int i=0; i<tam; i++){
        printf("\nf(%.2f) = %.2f", vx[i], vy[i]);
    }

    printf("\n\n");
}

float run(const int op, float a, 
        float b, 
        float (*f)(float), 
        float (*phix)(float),
        float (*derf)(float), int k){

    switch (op) {
        case 0: return method0(a, b, f, k);
        case 1: return method1(a, b, f, k);
        case 2: return method2(a, b, f, phix);
        case 3: return method3(a, b, f, derf);
        case 4: return method4(a, b, f);
        case 5: return method5(a, b, f, derf);
        default: return NAN;
    }
}