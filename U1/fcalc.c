#include "fcalc.h"

float* gerar_interval(float a, float b, float step, int *out_size){
    // out_size avisa que deve parar se chegar em zero
    if(step == 0.0f || (a<b && step<0.0f) || (a>b && step<0.0f)){
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
    char formulas[4]={"(2*pow(x,4)+4*pow(x,3)+3*pow(x,2)-10*x-15)",
            "(pow(x,5)-2*pow(x,4)-9*pow(x,3)+22*pow(x,2)+4*x-24)",
            "(5*pow(x,3)+pow(x,2)-pow(e, (1-2*x))+cos(x)+20)",
            "(sin(x)*x+4)"};

    printf("\n\n :::: VALORES DE %s ::::", formulas[pos]);
    for(int i=0; i<tam; i++){
        printf("\nf(%.2f)=%.2f", vx[i], vy[i]);
    }
}