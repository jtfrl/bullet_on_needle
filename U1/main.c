#include "zeroes_f.h" // métodos de calc. numérico
#include "fcalc.h" // obter valores e aplicar na função

#define elr M_E

// funções comentadas para não comprometer compilação

#define F_1(x) (2*pow(x,4)+4*pow(x,3)+3*pow(x,2)-10*x-15)
/* #define F_2(x) (pow(x,5)-2*pow(x,4)-9*pow(x,3)+22*pow(x,2)+4*x-24)
#define F_3(x) (5*pow(x,3)+pow(x,2)-pow(elr, (1-2*x))+cos(x)+20)
#define F_4(x) (sin(x)*x+4) */

/* #define DF_1(x) (8*pow(x,3)+12*pow(x,2)+6*x-10)
#define DF_2(x) (5*pow(x,4)-8*pow(x,3)-27*pow(x,2)+44*x+4)
#define DF_3(x) (15*pow(x,2)+2*x+2*(pow(elr,1-2*x))-sin(x))
#define DF_4(x) (cos(x)*x+sin(x))
 */
// draell, teria como simplificar essa ruma de linha??
static float f1(float x) { return F_1(x);}
/* static float f2(float x) { return F_2(x);}
static float f3(float x) { return F_3(x);}
static float f4(float x) { return F_4(x);} */
/* static float df1(float x) { return DF_1(x);}
static float df2(float x) { return DF_2(x);}
static float df3(float x) { return DF_3(x);}
static float df4(float x) { return DF_4(x);} */

int main(int argc, char* argv[]){

    if(argc<4){
        perror("Esperado o seguinte uso: ./main <inicio> <fim> <passo>");
        return -1;
    }

    int size=0;
    float a=atof(argv[1]);
    float b=atof(argv[2]);
    float h=atof(argv[3]);

    float *u_vx1=gerar_interval(a,b,h,&size);
    float *u_vy1=obter_val_f(f1, size, u_vx1);

    /* float *u_vx2=gerar_interval(a,b,h,&size);
    float *u_vy2=obter_val_f(f2, size, u_vx2);

    float *u_vx3=gerar_interval(a,b,h,&size);
    float *u_vy3=obter_val_f(f3, size, u_vx3);

    float *u_vx4=gerar_interval(a,b,h,&size);
    float *u_vy4=obter_val_f(f4, size, u_vx4); */

    show_val_f(u_vy1, u_vx1, 0, size);

/*

    float f_1=2*pow()
 */
    //obter_val_f
    
    //free aqui
    return 0;
}