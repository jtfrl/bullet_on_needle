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

const eps: f64 = 1e-6_f64;

impl<T> LinearSystem<T>
where T: Num + Clone + FromPrimitive {
    pub fn from(matrix_name: &str) -> Self {
        let ls = LinearSystem::<f64>::new(matrix_name);
        Self::convert(ls).unwrap()
    }

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
    
    pub fn retro_subs(&mut self) -> Option<Vec<f64>>
        where T: Send + Sync{
            let n_bs=bs.len();

            assert_eq(r#as.len(), n_bs, "A e b precisam ter o mesmo 
            tamanho (lin. de A = col. de B)!");

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
        Some(x)
    }

    pub fn fact_LU(&mut self) -> Option<Vec<f64>>
        where T: Send + Sync{
            let len = self.len;
            let mut lower: Vec<Vec<f64>> = vec![Vec::new();len];
            let mut upper: Vec<Vec<f64>> = vec![Vec::new();len];
            /*
                for p in 0..len {       // Pivots
                for i in (p+1)..len { // Rows
                    println!("p: {p}, i: {i}");
                    let m = self.r#as[i][p].clone() / self.r#as[p][p].clone(); // multiplier
                    // Li <- Li - mik * Lpivo
                */
            for i in 0..len{
                for j in (i+1)..len{
                    let mult=self.r#as[j][i].clone()/self.r#as[i][i].clone();
                    if i==j {lower[i][i]=1.0;}
                    else if i>j {lower[i][j]=m;}
                    else lower[i][j]=0.0;
                }
            }


            // still to be checked
            for p in 0..len{
                 for _i in 0..len{
                    for _j in (_i+1)..len{
                        let mult=self.r#as[_j][_i].clone()/self.r#as[_i][_i].clone();
                        //if i==j {lower[i][i]=1;}
                        else if _i>_j {upper[_i][_j]=0.0;}
                        self.r#as[_i][_j]-=mult*self.r#as[_i][_j];
                        else upper[_i][_j]=self.r#as[_i][_j];
                    }
                }
            }

            // TODO apply lower to be used in order to get the y
            // from formula Ly=b
            // then use the y to solve Ux=y

        }


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






    // GAUSS-JACOBI
   pub fn gauss_j(&mut self, max_iter: usize) -> Option<Vec<f64>>
        where T: Send+Sync{
            let len=self.len;
            let mut x_new: vec![0.0; len];
            let mut x_prev = vec![0.0; len];
            let mut soma = 0.0;

            if(0..n).any(|i| self.r#as[i][i].abs()<1e-12) {return None;}
            assert_eq(self.len, x_prev.len(), "Os vetores de
            solução x precisam ter o mesmo tamanho (n. de col (A) = n. de linhas (x)!");

            // === PASSO 1 ===
            // de início, consideramos as soluções do vet. x 
            // todas nulas[
            while(max_iter>0){
                for i in 0..len{
                    // let soma=0.0;
                    for _i in 0..len{
                        if _i!=i {soma=soma+self.r#as[i][_i]*x_prev[i];}
                    }

                    // === PASSO 2===
                    // atualizamos as soluções do vetor x
                    // de acordo com o vetor de x anterior
                    x_new[i]=(1.0/self.r#as[i][i])*(self.bs[i]-soma);
                  
                }

                let d=dist_abs(&x_new, &x_prev); // usamos passagem por referencia
                                                // para evitar overwrite
                if d<eps {
                    return Some(x_new);    
                    break;
                }

                std::mem::swap(&mut x_prev, &mut x_new); // troca de vetores

                max_iter=max_iter-1;
            }

            //Some(x_new);
            println!("\n\nATENÇÃO: Não convergiu.");
            None // não converge
        }

   // TODO -  critérios de Seicheifell

   // GAUSS-SEIDEL
   pub fn gauss_s(&mut self, max_iter: usize) -> Option<Vec<f64>>{
            let len=self.len;
            let mut x_new: vec![0.0; len];
            let mut soma = 0.0; 

            if(0..n).any(|i| self.r#as[i][i].abs()<1e-12) {return None;}

            assert_eq(self.len, x_prev.len(), "Os vetores de
            solução x precisam ter o mesmo tamanho (n. de col (A) = n. de linhas (x)!");

            while(max_iter>0){
                let mut max_diff=0.0_f64;
                for i in 0..len{
                    for j in 0..len{
                        if j!=i {soma=soma+self.r#as[i][j]*x_new[i];}
                        //else {soma=soma+self.r#as[i][j]*x_new[i];}
                    }

                    // PASSO DE ATUALIZAÇÃO PARA CADA VALOR DE X_i
                    // seja i+1 o novo vetor de X para encontrar a solução
                    // cada índice 'a' em [i+1][a] é uma posição na linha do vetor
                    // x_new[i+1][2], p. ex., que aproveita x_new[i+1][1] para já
                    // calcular com os valores novos encontrados

                    let new_value=(1.0/self.r#as[i][i])*(self.bs[i]-soma);
                    x_new[i]=new_value;
                    // TODO colocar seicheifell aqui (checar slides)
                }

                let d=dist_abs(&x_new, &x_prev);
                if d<eps{
                    return Some(x_new);
                    break;
                }
                max_iter=max_iter-1;
            }

            println!("\n\nATENÇÃO: Não convergiu.");
            None
        }

   pub fn residual_check(&mut self) -> Bool{
    // TODO: indicará se a diferença entre Ax e B vai a zero
   }
    


   // DISTÂNCIAS RELATIVAS E ABSOLUTAS
   // as saídas dessas funções devem ser
   // comapradas ao valor de eps
   pub fn dist_abs(Vec<f64> x_new, Vec<f64> x) -> f64{
        let tam=x_new.len();
        let diff: Vec<f64>;
        assert_eq(x_new.len(), x_prev.len(), "Os vetores de
         solução x precisam ter o mesmo tamanho!");
        
        for i in 0..tam{
            diff[i]=(x_new[i]-x[i]).abs();
        }

        let mut max = 0.0;
        for sol_x in &diff{
            if max<sol_x { max=sol_x;}
        }

        max;
   }
    
    
   pub fn dist_rel(Vec<f64> x_new, Vec<f64> x) -> f64{
        let diff_a=dist_abs(x_new, x);
        let mut max = 0.0;

        for sol_x in &x_new{
            sol_x=sol_x.abs();
            if max<sol_x {max=sol_x;}
        }

        let diff_r=diff_a/max;

        diff_r;
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

    

