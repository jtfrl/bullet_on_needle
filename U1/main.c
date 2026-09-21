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
    const char *nome;
    double (*f)(double);
    double (*df)(double);
    double (*phi)(double);
    double a, b, h;
} MathCalc;



int main(int argc, char* argv[]){

    if(argc<4){
        perror("Esperado o seguinte uso: ./main <inicio> <fim> <passo>");
        return -1;
    }

    int size=0;
    float a=atof(argv[1]);
    float b=atof(argv[2]);
    float h=atof(argv[3]);

   /*  float *u_vx1=gerar_interval(a,b,h,&size);
    float *u_vy1=obter_val_f(f1, size, u_vx1); */

    float *u_vx2=gerar_interval(a,b,h,&size);
    float *u_vy2=obter_val_f(f2, size, u_vx2);

    /*
    float *u_vx3=gerar_interval(a,b,h,&size);
    float *u_vy3=obter_val_f(f3, size, u_vx3);

    float *u_vx4=gerar_interval(a,b,h,&size);
    float *u_vy4=obter_val_f(f4, size, u_vx4); */

    show_val_f(u_vy2, u_vx2, 1, size);

    bool rnn=true;
    char op[10];
    int _op=0;

    while(rnn){
        printf("\n\n::: DESEJA VERIFICAR ZEROS DE FUNÇÃO? ::: \n ");
        printf("[1] - SIM | [2] - NÃO\n\n");
        
        fgets(op, sizeof(op), stdin);
        _op=atoi(op);
        
        if(_op<1 || _op>2){ 
            perror("Selecione uma opção válida\n");
            rnn=false;
        }

        switch (_op)
        {
        case 1:{
            printf("Opções de métodos:\n\n");
            printf("\n\n 0 = BISSECAÇÃO \n 1 = FALSA POS \n 2 = PONTO FIXO \n 3 = NEWTON-RAPHSON \n 4 = SECANTE \n\n");
            char op[10];
            int f_op=0;

            fgets(op, sizeof(op), stdin);
            f_op=atoi(op);

            printf("Indique os valores do intervalo: ");
            float u_a=0;
            float u_b=0;
            scanf("%f %f", &u_a, &u_b);
            while (getchar() != '\n'); 
            float x_root=0;
            bool use_of_phi= (f_op==2) ? x_root=run(f_op, a, b, f2, phix_2, NULL, 0): (goto der_use);
            der_use: 
                bool use_of_der= (f_op==3)
            float x_root=run(f_op, a, b, f2, NULL, NULL, 0);
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