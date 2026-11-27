#include <stdio.h>
#include <stdlib.h>
#include <math.h>

// by @VictorRhyan

// ============================================================================
// FUNÇÕES AUXILIARES E MÉTODOS DE INTERPOLAÇÃO (2.1)
// ============================================================================

/**
 * Entradas:
 *   - n: número de pontos (grau do polinômio = n - 1)
 *   - x: vetor com as coordenadas x dos pontos conhecidos
 *   - y: vetor com as coordenadas y dos pontos conhecidos
 *   - x_val: valor de x onde se deseja calcular o polinômio interpolador
 * Saída:
 *   - Valor interpolado P(x_val)
 */

// 1. Interpolação via Sistema Linear (Matriz de Vandermonde + Eliminação de Gauss)
double interpolacao_sistema_linear(int n, const double x[], const double y[], double x_val) {
    // Alocação dinâmica de memória para a matriz A e vetores B e coeficientes a
    double **A = (double **)malloc(n * sizeof(double *));
    for (int i = 0; i < n; i++) {
        A[i] = (double *)malloc(n * sizeof(double));
    }
    double *B = (double *)malloc(n * sizeof(double));
    double *a = (double *)malloc(n * sizeof(double));

    // Montagem da Matriz de Vandermonde A[i][j] = x_i^j e vetor coluna B
    for (int i = 0; i < n; i++) {
        B[i] = y[i];
        for (int j = 0; j < n; j++) {
            A[i][j] = pow(x[i], j);
        }
    }

    // Eliminação de Gauss com Pivoteamento Parcial
    for (int i = 0; i < n; i++) {
        int max_row = i;
        for (int k = i + 1; k < n; k++) {
            if (fabs(A[k][i]) > fabs(A[max_row][i])) {
                max_row = k;
            }
        }
        if (max_row != i) {
            double *temp_ptr = A[i];
            A[i] = A[max_row];
            A[max_row] = temp_ptr;
            double temp_val = B[i];
            B[i] = B[max_row];
            B[max_row] = temp_val;
        }

        for (int k = i + 1; k < n; k++) {
            if (fabs(A[i][i]) < 1e-15) continue; // Evita divisão por zero
            double fator = A[k][i] / A[i][i];
            for (int j = i; j < n; j++) {
                A[k][j] -= fator * A[i][j];
            }
            B[k] -= fator * B[i];
        }
    }

    // Retrosubstituição
    for (int i = n - 1; i >= 0; i--) {
        a[i] = B[i];
        for (int j = i + 1; j < n; j++) {
            a[i] -= A[i][j] * a[j];
        }
        a[i] /= A[i][i];
    }

    // Avaliação do polinômio P(x_val) = a0 + a1*x + a2*x^2 + ...
    double resultado = 0.0;
    for (int j = 0; j < n; j++) {
        resultado += a[j] * pow(x_val, j);
    }

    // Liberação de memória
    for (int i = 0; i < n; i++) free(A[i]);
    free(A);
    free(B);
    free(a);

    return resultado;
}

// 2. Interpolação via Método de Lagrange
double interpolacao_lagrange(int n, const double x[], const double y[], double x_val) {
    double p_x = 0.0;
    for (int i = 0; i < n; i++) {
        double L_i = 1.0;
        for (int j = 0; j < n; j++) {
            if (j != i) {
                L_i *= (x_val - x[j]) / (x[i] - x[j]);
            }
        }
        p_x += y[i] * L_i;
    }
    return p_x;
}

// 3. Interpolação via Diferenças Divididas de Newton
double interpolacao_diferencas_divididas(int n, const double x[], const double y[], double x_val) {
    double **F = (double **)malloc(n * sizeof(double *));
    for (int i = 0; i < n; i++) {
        F[i] = (double *)malloc(n * sizeof(double));
    }

    for (int i = 0; i < n; i++) {
        F[i][0] = y[i];
    }

    for (int j = 1; j < n; j++) {
        for (int i = j; i < n; i++) {
            F[i][j] = (F[i][j-1] - F[i-1][j-1]) / (x[i] - x[i-j]);
        }
    }

    double p_x = F[0][0];
    double termo = 1.0;
    for (int i = 1; i < n; i++) {
        termo *= (x_val - x[i-1]);
        p_x += F[i][i] * termo;
    }

    for (int i = 0; i < n; i++) free(F[i]);
    free(F);

    return p_x;
}

// ============================================================================
// TESTES E EXECUTÁVEIS DAS QUESTÕES
// ============================================================================

void executar_questao_2_2() {
    printf("\n=======================================================\n");
    printf("f(x) = e^x em [1, 3]\n");
    printf("=======================================================\n");

    int valores_n[] = {3, 4, 5, 10}; // Graus do polinômio (n pontos = n_grau + 1)
    double x_teste = 2.0; // Ponto médio para avaliação
    double valor_exato = exp(x_teste);

    printf("Valor Exato de f(%.2f) = e^(%.2f) = %.8f\n\n", x_teste, x_teste, valor_exato);

    for (int k = 0; k < 4; k++) {
        int n_grau = valores_n[k];
        int n_pontos = n_grau + 1; // n_pontos para grau n

        double *x = (double *)malloc(n_pontos * sizeof(double));
        double *y = (double *)malloc(n_pontos * sizeof(double));

        double h = (3.0 - 1.0) / n_grau;
        for (int i = 0; i < n_pontos; i++) {
            x[i] = 1.0 + i * h;
            y[i] = exp(x[i]);
        }

        double val_sis = interpolacao_sistema_linear(n_pontos, x, y, x_teste);
        double val_lag = interpolacao_lagrange(n_pontos, x, y, x_teste);
        double val_new = interpolacao_diferencas_divididas(n_pontos, x, y, x_teste);

        printf("--- Grau n = %d (%d pontos) ---\n", n_grau, n_pontos);
        printf("Sistema Linear:       %.8f | Erro: %.2e\n", val_sis, fabs(val_sis - valor_exato));
        printf("Lagrange:             %.8f | Erro: %.2e\n", val_lag, fabs(val_lag - valor_exato));
        printf("Diferencas Divididas: %.8f | Erro: %.2e\n\n", val_new, fabs(val_new - valor_exato));

        free(x);
        free(y);
    }
}

void executar_questao_2_3() {
    printf("\n=======================================================\n");
    printf("f(x) = 1 / (1 + 25*x^2) (Fenomeno de Runge)\n");
    printf("=======================================================\n");

    int valores_n[] = {4, 10, 15};
    double x_teste = 0.5; // Ponto de teste
    double valor_exato = 1.0 / (1.0 + 25.0 * x_teste * x_teste);

    printf("Valor Exato de f(%.2f) = %.8f\n\n", x_teste, valor_exato);

    for (int k = 0; k < 3; k++) {
        int n_grau = valores_n[k];
        int n_pontos = n_grau + 1;

        double *x = (double *)malloc(n_pontos * sizeof(double));
        double *y = (double *)malloc(n_pontos * sizeof(double));

        double h = (4.0 - 0.0) / n_grau;
        for (int i = 0; i < n_pontos; i++) {
            x[i] = 0.0 + i * h;
            y[i] = 1.0 / (1.0 + 25.0 * x[i] * x[i]);
        }

        double val_sis = interpolacao_sistema_linear(n_pontos, x, y, x_teste);

        printf("--- Grau n = %d (%d pontos) ---\n", n_grau, n_pontos);
        printf("Resultado via Sistema Linear em x=%.2f: %.8f\n", x_teste, val_sis);
        printf("Erro Absoluto: %.8f\n\n", fabs(val_sis - valor_exato));

        free(x);
        free(y);
    }
}

void executar_questao_2_4() {
    printf("\n=======================================================\n");
    printf("Concentracao de CO2 (NOAA)\n");
    printf("=======================================================\n");

    // Dados fornecidos
    double anos[] = {1960, 1970, 1980, 1990, 2000, 2010, 2020};
    double co2[]  = {315,  327,  337,  335,  370,  388,  411};

    // Item (a): Interpolação de 3º grau para o ano de 2005 (requer 4 pontos)
    // Pontos escolhidos: 1990, 2000, 2010, 2020 (centralizados em torno de 2005)
    double x_a[] = {1990, 2000, 2010, 2020};
    double y_a[] = {335,  370,  388,  411};
    double res_2005 = interpolacao_lagrange(4, x_a, y_a, 2005);
    printf("(a) Estimativa para o ano 2005 (Grau 3, pontos 1990-2020): %.2f ppm\n\n", res_2005);

    // Item (b): Estimativa para 2008 (Grau 2 e Grau 3)
    // Grau 2 (3 pontos): 1990, 2000, 2010
    double x_b2[] = {1990, 2000, 2010};
    double y_b2[] = {335,  370,  388};
    double res_2008_g2 = interpolacao_lagrange(3, x_b2, y_b2, 2008);

    // Grau 3 (4 pontos): 1990, 2000, 2010, 2020
    double res_2008_g3 = interpolacao_lagrange(4, x_a, y_a, 2008);

    printf("(b) Estimativas para o ano 2008 (Observado = 381.00 ppm):\n");
    printf("    - Polinomio Grau 2: %.2f ppm | Erro: %.2f ppm\n", res_2008_g2, fabs(res_2008_g2 - 381.0));
    printf("    - Polinomio Grau 3: %.2f ppm | Erro: %.2f ppm\n\n", res_2008_g3, fabs(res_2008_g3 - 381.0));

    // Item (c): Determinar o ano em que CO2 atinge 350 ppm usando polinômio de 2º grau
    // Escolhe-se o intervalo [1980, 1990, 2000] onde 350 ppm está contido
    // Método: Interpolação Inversa (X como função de Y)
    double x_c[] = {1980, 1990, 2000};
    double y_c[] = {337,  335,  370};
    // Interpola-se Ano(CO2) para CO2 = 350 ppm
    double ano_350 = interpolacao_lagrange(3, y_c, x_c, 350.0);

    printf("(c) Ano estimado para concentracao de 350 ppm (Grau 2): %.2f (Aproximadamente ano %.0f)\n", 
           ano_350, round(ano_350));
}

int main() {
    executar_questao_2_2();
    executar_questao_2_3();
    executar_questao_2_4();
    return 0;
}