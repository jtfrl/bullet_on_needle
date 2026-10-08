//  Box<dyn? []>

pub mod read;

use crate::config;

use num_traits::{FromPrimitive, *};
use std::ops::*;
use std::{path::Path, fs};

// TODO: get away from these shitty vectors!!!!
// this is inteded to create a matrix (some helper attributes)
#[derive(Clone, Debug)]
pub struct LinearSystem<T>
where T: Num + Clone + FromPrimitive {
    pub r#as: Vec<Vec<T>>,
    pub bs  : Vec<T>,
    pub len : usize,
}

impl<T> LinearSystem<T>
where T: Num + Clone + FromPrimitive {
    pub fn from(matrix_name: &str) -> Self {
        let ls = LinearSystem::<f64>::new(matrix_name);
        Self::convert(ls).unwrap()
    }

    //TODO control of changing lines (maybe another function)
    pub fn gauss_elim(&mut self)
    where T: Send + Sync {
        let len = self.len;
        for p in 0..len {       // Pivots
            for i in (p+1)..len { // Rows
                println!("p: {p}, i: {i}");
                let m = self.r#as[i][p].clone() / self.r#as[p][p].clone(); // multiplier
                // Li <- Li - mik * Lpivo
                self.r#as[i][p] = T::zero();
                for j in p+1..len { // Cols
                    self.r#as[i][j] = self.r#as[i][j].clone()  
                        m.clone() * self.r#as[p][j].clone();
                }
                // bi <- bi - mip*bp
                self.bs[i] = self.bs[i].clone() - m*self.bs[p].clone();
            }
        }
    }
    
    pub fn retro_subs(&mut self) -> Option<Vec<f64>>{
        where T: Send + Sync{
            let n_bs=bs.len();

            assert_eq(r#as.len(), n_bs, "A e b precisam ter o mesmo tamanho (lin. de A = col. de B)!");

            let mut x=vec![0.0f64;n];

            for i in (0..n).rev(){
                let mut soma=0.0f64;
                for j in (i+1)..n{
                    soma+=self.r#as[i][j]*x[j];
                }
                if r#as[i][i]==0.0{
                    return None;
                }

                x[i]=(bs[i]-soma)/r#as[i][i];
            }
        }
        Some(x)
    }

    // NEXT METHODS TO INCLUDE
    // f*cking LU factoration
    // apply LU to solve LinearSys
    // gauss-jacobi iterative here too


    fn convert(ls: LinearSystem<f64>) -> Option<LinearSystem<T>>
    where
        T: Num + Clone + FromPrimitive,
    {
        Some(Self{
            r#as: ls.r#as
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(T::from_f64)
                        .collect::<Option<Vec<T>>>()
                })
                .collect::<Option<Vec<Vec<T>>>>()?,

            bs: ls.bs
                .into_iter()
                .map(T::from_f64)
                .collect::<Option<Vec<T>>>()?,

            len: ls.len,
        })
    }

}

impl LinearSystem<f64> {
    pub fn new(matrix_name: &str) -> Self {
        let mut as_path = Path::new(config::AS_PATH).join(matrix_name);
        as_path.set_extension("mtx");
        let bs_path = Path::new(config::BS_PATH).join(matrix_name);

        let as_content = fs::read_to_string(&as_path).unwrap_or_else(|err| {
            panic!("Erro ao ler o arquivo A ({:?}): {err}", as_path);
        });

        let r#as = read::extract_sas(&as_content);
        let n = r#as.len();

        // let opt_bs_content = fs::read_to_string(&bs_path);
        let bs = read::handle_bs(bs_path, n);
        Self { r#as, bs, len: n }
    }
    
    
}
