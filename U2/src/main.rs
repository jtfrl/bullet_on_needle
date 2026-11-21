mod linear_system;
pub mod config;

use std::io;
use std::time::Instant;
use num_bigint::BigInt;
use num_rational::BigRational;
use std::env; // ambiente de execução por linha de comando
use std::process::exit;
// use std::io::{self Write};

// matrizes A a serem usadas
// bcsstk22, 23, 24

use linear_system::LinearSystem;
// ::!:: comentado para usar linha de comando
// const MATRIX_NAME: &'static str = "add32.mtx";
/* const MATRIX_NAME: &'static str = "bcsstk22.mtx";
const VECTOR_B: &'static str = "add32.mtx"; */
const MAX_ITER: u32 = 10000;

// acima antes era: dwb512.mtx

fn apply_methods(a_ls: &str, b_ls: &str, op: usize) -> Option<Vec<f64>>{
    let carga_t=Instant::now(); //> tempo de carga
    let mut linearA_b = LinearSystem::<BigRational>::from(a_ls, b_ls);
    let final_t=carga_t.elapsed();

    let mut copy=linearA_b.clone(); //> copy impede problemas nos métodos
    let n=linearA_b.r#as.len(); 
          
    let sol_t=Instant::now(); //> tempo de solução
    match op {
    let mut x_sol=vec![0.0f64; n];
        1 =>{
            copy.gauss_elim();
            println!("\n Usar retrossubstituição (1) ou substituição frontal (2)?");
            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).ok()?; // Se falha => NULL
            let input: usize = choice.trim().parse()::<usize>().expect("Erro ao ler número.");
            x_sol=copy.retro_frontis_subs(input);
            // realizaremos um teste aqui para checar se o método de
            // fato consegue resolver Ax=b com eliminação de Gauss
            
            let rc_1=linearA_b.residual_check(x_sol);
            if !rc_1 {panic!("Solução não aceitável (o resíduo não é menor \
            que o epsilon estabelecido!)");}

        }
        2=>{
            x_sol=copy.fact_LU();
            let rc_2=linearA_b.residual_check(x_sol);
            if !rc_2 {panic!("Solução não aceitável (o resíduo não é menor \
            que o epsilon estabelecido!)");}
        }
        3=>{
            x_sol=copy.gauss_s(MAX_ITER);
        }
        4=>{
            x_sol=copy.gauss_j(MAX_ITER);
        }
        _=>{
            panic!("\nOpção inválida. Use números entre 1 e 4.");
        }

    }
    let sfinal_t=sol_t.elapsed();

    println!("\n\nTEMPO DE CARGA: {:?} | \t TEMPO DE SOLUÇÃO: {:?}", (final_t.as_secs_f64()*1e3), (sfinal_t.as_secs_f64()*1e3));
    println!("\n\n");
    show_res(x_sol,a_ls, b_ls);
    Some(x_sol)
}

fn show_res(x: Vec<f64>, a: &str, b: &str){
    println!("\nA = {}, B = {}", a, b);
    println!("\nVALORES DO VETOR x PARA Ax=b");
    for num in &x{
        println!("\n {}", *num);
    }
}


fn main() {
    let args: Vec<String> = env::args().collect(); //> recebe via linha de comando as informações

    if args.len() != 4{
        eprintln!(
            "uso: {} <matrix.mtx> <vetor_b.mtx> <op> \n\
            op: 1=Gauss, 2=LU, 3=Gauss-Seidel, 4=Gauss-Jacobi", 
            args[0]
        );
        exit(1);
    }

    let m_a=&args[1];
    let m_b=&args[2];
    let op: usize = match args[3].parse(){
        Ok(v)=> v,
        Err(_)=>{
            eprintln!("Número opicional inválido. Escolha um número de 1 a 4. op={}", args[3]);
            exit(1);
        }
    };

    apply_methods(m_a, m_b, op); 

   /*  println("\nDigite alguma opção de método de solução de sistemas lineares (lista a seguir): ");
    println("\n\n1. ELIMINAÇÃO DE GAUSS \n 2. FATORAÇÃO L.U. \n 3. GAUSS-SEIDEL\n 4. GAUSS-JACOBI");
    let mut input_op: usize = input.trim().parse().unwrap();*/
 
}

