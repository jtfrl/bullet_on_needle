#include "zeroes_f.h" // métodos de calc. numérico
#include "fcalc.h" // obter valores e aplicar na função

#define elr M_E

/* ==== FÓRMULAS ==== */
static float f1(float x)  { return 2*pow(x,4) + 4*pow(x,3) + 3*pow(x,2) - 10*x - 15; }
static float f2(float x)  { return pow(x,5) - 2*pow(x,4) - 9*pow(x,3) + 22*pow(x,2) + 4*x - 24; }
static float f3(float x)  { return 5*pow(x,3)+pow(x,2)-pow(elr, (1-2*x))+cos(x)+20; }
static float f4(float x)  { return x*sin(x) + 4; }


/* ==== DERIVADAS ==== */
/* (a) f1(x) = 2x^4 + 4x^3 + 3x^2 - 10x - 15   em [0, 3], h = 0.6 */
static float df1(float x) { return 8*pow(x,3) + 12*pow(x,2) + 6*x - 10; }

/* (b) f2(x) = x^5 - 2x^4 - 9x^3 + 22x^2 + 4x - 24  em [0, 5], h = 0.7 */
static float df2(float x) { return 5*pow(x,4) - 8*pow(x,3) - 27*pow(x,2) + 44*x + 4; }

/* (c) f3(x) = 5x^3 + x^2 - e^(1-2x) + cos(x) + 20   em [-5, 5], h = 0.5 */
static float df3(float x) { return 15*pow(x,2)+2*x+2*(pow(elr,1-2*x))-sin(x);}

/* (d) f4(x) = x*sen(x) + 4   em [1, 5], h = 0.5 */
static float df4(float x) { return  (cos(x)*x+sin(x)); }

/* ==== PHI ==== */
static float phix_1(float x) { return (-2*pow(x,4)-4*pow(x,3)-3*pow(x,2)+15)/(-10.0);}
static float phix_2(float x) { return (-pow(x,5)+2*pow(x,4)+9*pow(x,3)-22*pow(x,2)+24)/(-4.0);}
static float phix_3(float x) { 
    if(x<=0) return (-1e7-1)/(-2.0); //optamos trabalhar com um negativo muito "alto", caso x negativo
    return (log(5*pow(x,3)+pow(x,2)+cos(x)+20)-1)/(-2.0);
}
static float phix_4(float x) { return (-4/sin(x));}


typedef struct {
    float (*f)(float);
    float (*df)(float);
    float (*phi)(float);
    float a, b, h;
} MathCalc;

static const MathCalc Questoes[]={
    {f1, df1, phix_1, 0, 3, 0.6f},
    {f2, df2, phix_2, 0, 5, 0.7f},
    {f3, df3, phix_3, -5, 5, 0.5f},
    {f4, df4, phix_4, 1, 5, 0.5f}
};


int main(int argc, char* argv[]){

    if(argc<5){
        perror("Esperado o seguinte uso: ./main <inicio> <fim> <passo> <funcao>");
        return -1;
    }

    float a=atof(argv[1]);
    float b=atof(argv[2]);
    float h=atof(argv[3]);
    int f_id=atoi(argv[4]);

    if(f_id<0 || f_id>3){
        fprintf(stderr, "\n\n ERRO: selecione um número de função entre 0 e 3");
        return EXIT_FAILURE;
    }

    MathCalc calcf=Questoes[f_id];
    calcf.a=a;
    calcf.b=b;
    calcf.h=h;

    int size=0;
    float* u_vx=gerar_interval(calcf.a, calcf.b, calcf.h, &size);
    float* u_vy=obter_val_f(calcf.f, size, u_vx);

    show_val_f(u_vy, u_vx, f_id, size);

    free(u_vx);
    free(u_vy);

    bool rnn=true;
    int _op=0;

    while(rnn){
        printf("\n\n::: DESEJA VERIFICAR ZEROS DE FUNÇÃO? ::: \n ");
        printf("[1] - SIM | [2] - NÃO\n\n");
        
        scanf("%d", &_op);
        
        if(_op<1 || _op>2){ 
            perror("Selecione uma opção válida\n");
            rnn=false;
        }

        switch (_op)
        {
        case 1:{
            printf("Opções de métodos:\n\n");
            printf("\n\n 0 = BISSECAÇÃO \n 1 = FALSA POS \n 2 = PONTO FIXO \n 3 = NEWTON-RAPHSON \n 4 = SECANTE \n 5= HÍBRIDO \n\n");
            int f_op=0;

            scanf("%d", &f_op);

            float x_root=run(f_op, 
                            calcf.a, 
                            calcf.b, 
                            calcf.f, 
                            calcf.phi, 
                            calcf.df, 
                            0);
            printf("\n\nraiz da função: %.5f", x_root);
            break;
        }
        case 2:{
            rnn=false;
            printf("Saindo do programa...\n");
            break;
        }
        default:{
            break;
        }
        }
    }

    return 0;
}