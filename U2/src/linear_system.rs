//  Box<dyn? []>

pub mod read;

use crate::config;

use num_traits::{FromPrimitive, *};
use std::ops::*;
use std::{path::Path, fs};

// TODO: get away from these shitty vectors!!!!
#[derive(Clone)]
struct LinearSystem<T>
where T: Num + Copy {
    pub r#as: Vec<Vec<T>>,
    pub bs  : Vec<T>,
    pub len : usize,
}

impl<T> LinearSystem<T>
where T: Num + Copy {
    fn gauss_elim(&mut self) {
        let len = self.len;
        for p in 0..len {       // Pivots
            for i in (p+1)..len { // Rows
                let m = self.r#as[i][p] / self.r#as[p][p];
                // Li <- Li - mik * Lpivo
                for j in 0..len { // Cols
                    self.r#as[i][j] = self.r#as[i][j] - 
                        m * self.r#as[p][j];
                }
            }
        }
    }

    fn convert(ls: LinearSystem<f64>) -> Option<LinearSystem<T>>
    where
        T: Num + Copy + FromPrimitive,
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
