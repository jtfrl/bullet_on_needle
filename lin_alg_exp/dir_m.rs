// by: jjoaoll
//  Box<dyn? []>

use num_traits::*;
// TODO: get away from these shitty vectors!!!!
#[derive(Clone)]
struct LinearAlgebra<T>
where T: Num + Copy {
    pub r#as: Vec<Vec<T>>,
    pub bs  : Vec<T>,
    pub len : usize,
}

fn main() {
    let x = String::from("hello");
    let y = x;
    //let mut r0 = &x;
    //*r0 += 1;
    //println!("{x}");
    ////let r1 = &x;
    ////let r2 = &x;
    //
    ////println!("{r1}");
    //println!("{r0}");
}

impl<T> LinearAlgebra<T>
where T: Num + Copy {
    fn gauss_elim(&mut self) {
        let len = self.len;
        for p in 0..len {       // Pivots
            for i in j+1..len { // Rows
                let m = self.r#as[i][p] / self.r#as[p][p] ;
                // Li <- Li - mik * Lpivo
                for j in 0..len { // Cols
                    self.r#as[i][j] -= m * self.r#as[p][j];
                }
            }
        }
    }
}