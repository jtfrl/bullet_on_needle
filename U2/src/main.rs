//  Box<dyn? []>

mod linear_system;
pub mod config;

use std::io;
use num_bigint::BigInt;
use num_rational::BigRational;

//bcsstk22, 23, 24

use linear_system::LinearSystem;
// const MATRIX_NAME: &'static str = "add32.mtx";
const MATRIX_NAME: &'static str = "bcsstk22.mtx";
const VECTOR_B: &'static str = "add32.mtx"

// acima antes era: dwb512.mtx


// TODO: verificar se talvez seja preciso trocar 
// o switch por pipes de closure

fn apply_methods(a_ls: &str, b_ls: &str, /*fechamento */, op: usize) -> Option<Vec<f64>>{
    //TODO: instant now, fechamento, elapsed()
    //TODO: criar linearA_b como na main (ajustar)

    let mut linearA_b = LinearSystem::<BigRational>::from(a_ls, b_ls);

    let mut copy=linearA_b.clone(); //> aplicamos copy para 
                                    // evitar problemas nos demais métodos

    let mut x_sol: vec![0.0f64, n];
    let n=linearA_b.r#as.len;
          
    match op {
        1 =>{
            copy.gauss_elim();
            println!("{m:?} \n");
            println!("\n Usar retrossubstituição (1) ou substituição frontal (2)?");
            let mut input: usize = input.trim().parse().unwrap();
            if input<1 || input>2 {panic("Escolha uma opção válida para obter o x");}
            x_sol=linearA_b.retro_frontis_subs(1);
        }
        2=>{
            copy.fact_LU();
        }

    }

    Some(x_sol)
}


fn main() {
    println("Digite alguma opção de método de solução de sistemas lineares (lista a seguir): ");
    //TODO: listar
    let mut input_op: usize = input.trim().parse().unwrap();
    apply_methods(MATRIX_NAME, VECTOR_B, input_op); // FALTA APLICAR FECHAMENTO
 
}

