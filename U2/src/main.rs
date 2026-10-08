//  Box<dyn? []>

mod linear_system;
pub mod config;

use num_bigint::BigInt;
use num_rational::BigRational;

use linear_system::LinearSystem;
// const MATRIX_NAME: &'static str = "add32.mtx";
const MATRIX_NAME: &'static str = "dwb512.mtx";

fn main() {
    let mut m = LinearSystem::<BigRational>::from(MATRIX_NAME);
    // let mut m = LinearSystem::<f32>::from(MATRIX_NAME);
    m.gauss_elim();
    println!("{m:?}");
}

