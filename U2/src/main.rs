//  Box<dyn? []>

mod linear_system;
pub mod config;

use num_bigint::BigInt;
use num_rational::BigRational;

//bcsstk22, 23, 24

use linear_system::LinearSystem;
// const MATRIX_NAME: &'static str = "add32.mtx";
const MATRIX_NAME: &'static str = "bcsstk22.mtx";
const VECTOR_B: &'static str = "add32.mtx"

// acima antes era: dwb512.mtx


fn apply_methods(ls: &str, /*fechamento */, op: usize) -> Option<Vec<f64>>{
    //TODO: instant now, fechamento, elapsed()

    switch(op){
        // TODO aplicar jacobi, seidel, LU etc.
    }

    Some(x)
}


fn main() {
    let mut m = LinearSystem::<BigRational>::from(MATRIX_NAME);
    
    // let mut m = LinearSystem::<f32>::from(MATRIX_NAME);

    // TODO ajustar conversão do b


    let mut copy=m.clone();
    //aplicamos copy para evitar problemas nos demais métodos




    copy.gauss_elim();
    println!("{m:?}");
}

