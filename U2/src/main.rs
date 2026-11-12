//  Box<dyn? []>

mod linear_system;
pub mod config;

use std::io;
use std::time::Instant;
use num_bigint::BigInt;
use num_rational::BigRational;

//bcsstk22, 23, 24

use linear_system::LinearSystem;
// const MATRIX_NAME: &'static str = "add32.mtx";
const MATRIX_NAME: &'static str = "bcsstk22.mtx";
const VECTOR_B: &'static str = "add32.mtx";
const MAX_ITER: u32 = 10000;

// acima antes era: dwb512.mtx


// TODO: verificar se talvez seja preciso trocar 
// o switch por pipes de closure

fn apply_methods(a_ls: &str, b_ls: &str, op: usize) -> Option<Vec<f64>>{
    //TODO: usar a função show_res aqui
    //TODO: aplicar verificações (sassenfeld OK, dist abs, rel., res)  

    let carga_t=Instant::now(); //> tempo de carga
    let mut linearA_b = LinearSystem::<BigRational>::from(a_ls, b_ls);
    let final_t=carga_t.elapsed();

    let mut copy=linearA_b.clone(); //> aplicamos copy para 
                                    // evitar problemas nos demais métodos

    let mut x_sol: vec![0.0f64, n];
    let n=linearA_b.r#as.len;
    let mut convergence: bool = false;
          
    // TODO: aplicar residual_check especialmente para métodos iterativos
    let sol_t=Instant::now(); //> tempo de solução
    match op {
        1 =>{
            copy.gauss_elim();
            //println!("{m:?} \n");
            println!("\n Usar retrossubstituição (1) ou substituição frontal (2)?");
            let mut input: usize = input.trim().parse().unwrap();
            if input<1 || input>2 {panic("Escolha uma opção válida para obter o x");}
            x_sol=copy.retro_frontis_subs(1);
        }
        2=>{
            x_sol=copy.fact_LU();
        }
        3=>{
            convergence=copy.sassenfeld();
            if convergence {x_sol=copy.gauss_s(MAX_ITER);}
            else {println!("ATENÇÃO: o sistema linear não converge para Gauss-Seidel")}
        }
        4=>{
            x_sol=copy.gauss_j(MAX_ITER);
        }
        _=>{
            panic!("\nOpção inválida. Use números entre 1 e 4.");
        }

    }
    let sfinal_t=sol_t.elapsed();

    println!("TEMPO DE CARGA: {:?} | \t TEMPO DE SOLUÇÃO: {:?}", final_t.as_secs_f64()*1e3, sfinal_t.as_secs_f64()*1e3);

    Some(x_sol)
}

fn show_res(x: vec![f64], a: &str, b: &str){
    println!("\nA = {a}, B = {b}");
    println!("\nVALORES DO VETOR x PARA Ax=b");
    for num in &x{
        println!("\n {num}");
    }
}


fn main() {
    println("\nDigite alguma opção de método de solução de sistemas lineares (lista a seguir): ");
    println("\n\n1. ELIMINAÇÃO DE GAUSS \n 2. FATORAÇÃO L.U. \n 3. GAUSS-SEIDEL\n 4. GAUSS-JACOBI");
    let mut input_op: usize = input.trim().parse().unwrap();
    apply_methods(MATRIX_NAME, VECTOR_B, input_op); // FALTA APLICAR FECHAMENTO
 
}

