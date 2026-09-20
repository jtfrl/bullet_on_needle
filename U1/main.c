#include "zeroes_f.h" // métodos de calc. numérico
#include "fcalc.h" // obter valores e aplicar na função

#define elr M_E

// funções comentadas para não comprometer compilação

//#define F_1(x) (2*pow(x,4)+4*pow(x,3)+3*pow(x,2)-10*x-15)
#define F_2(x) (pow(x,5)-2*pow(x,4)-9*pow(x,3)+22*pow(x,2)+4*x-24)
/* #define F_3(x) (5*pow(x,3)+pow(x,2)-pow(elr, (1-2*x))+cos(x)+20)
#define F_4(x) (sin(x)*x+4) */

//#define DF_1(x) (8*pow(x,3)+12*pow(x,2)+6*x-10)
//#define DF_2(x) (5*pow(x,4)-8*pow(x,3)-27*pow(x,2)+44*x+4)
/* #define DF_3(x) (15*pow(x,2)+2*x+2*(pow(elr,1-2*x))-sin(x))
#define DF_4(x) (cos(x)*x+sin(x)) */

// draell, teria como simplificar essa ruma de linha??
//static float f1(float x) { return F_1(x);}
static float f2(float x) { return F_2(x);}
/* static float f3(float x) { return F_3(x);}
static float f4(float x) { return F_4(x);} */
//static float df1(float x) { return DF_1(x);}
//static float df2(float x) { return DF_2(x);}
/* static float df3(float x) { return DF_3(x);}
static float df4(float x) { return DF_4(x);} 
 */
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
            printf("\n\n 0 = BISSECAÇÃO \n 1 = FALSA POS \n 2 = PONTO FIXO \n 3 = NEWTON-RAPHSON \n\n");
            char op[10];
            int f_op=0;

            fgets(op, sizeof(op), stdin);
            f_op=atoi(op);

            printf("Indique os valores do intervalo: ");
            float u_a=0;
            float u_b=0;
            scanf("%f %f", &u_a, &u_b);


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

    
    //free aqui
    return 0;
}