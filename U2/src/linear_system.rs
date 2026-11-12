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
where T: Num + Clone + FromPrimitive + ToPrimitive {
    pub fn from(matrix_name: &str, b_matrix: &str) -> Self {
        let mut ls = LinearSystem::<f64>::new(matrix_name, b_matrix);
        for b in ls.bs.iter_mut(){
            *entry-=50.0; // adaptamos cada vetor b
                         //  para ficar no intervalo 
                         // entre -50 e 50
        }
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
                    self.r#as[i][j] = self.r#as[i][j].clone() - m.clone() * self.r#as[p][j].clone();
                }

                // bi <- bi - mip*bp
                self.bs[i] = self.bs[i].clone() - m*self.bs[p].clone();
            }
        }
    }
    
    // agora, usamos 1 para retrossubstituição, e 2 para substituição frontal
    pub fn retro_frontis_subs(&mut self, op: usize) -> Option<Vec<f64>>
        where T: Send + Sync{
            let n_bs=bs.len();

            assert_eq(r#as.len(), n_bs, "A e b precisam ter o mesmo 
            tamanho (lin. de A = col. de B)!");

            let mut x=vec![0.0f64;n];

            if(op==1){
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
            }else if(op==2){
                  for i in (0..n){
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
            else{
                panic!("ERRO! Escolha uma opção para retrossubstituição (1) ou substituição frontal (2).");
            }
        Some(x)
    }

    pub fn fact_LU(&mut self) -> Option<Vec<f64>>
        where T: NumAssign + ToPrimitive{
            let len = self.len;
            let mut lower: Vec<Vec<f64>> = vec![vec![0.0f64; len];len];
            let mut upper: Vec<Vec<f64>> = vec![vec![0.0f64; len];len];

            // MATRIZ INFERIOR E SUPERIOR
            for i in 0..len{
                lower[i][i]=1.0;
                if self.r#as[i][i].is_zero() { 
                    panic!("Pivô nulo!");
                    return None; 
                } // Fatora em LU, mas se zero está no pivô vai a None
                for j in (i+1)..len{
                    let mult=self.r#as[j][i].clone()/self.r#as[i][i].clone();
                    lower[j][i]=mult.to_f64()?;
                    for k in i..len{
                        let advance=self.r#as[i][k].clone();
                        self.r#as[j][k]-=mult*advance;
                    }
                }
            }

            for g in 0..len{
                for h in g..len{
                    upper[g][h]=self.r#as[g][h].to_f64()?;
                }
            }
        

            let mut y = vec![0.0f64; len];
            for i in 0..len{
                let mut s=0.0;
                for j in 0..i{
                    s+=lower[i][j]*y[j];
                }
                y[i]=(self.bs[i].to_f64()?-s);
            }

            let mut x=vec![0.0f64; len];
            for i in (0..len).rev(){
                let mut sigma=0.0;
                for j in (i+1)..len{
                    sigma+=upper[i][j]*x[j];
                }
                if upper[i][i].abs()<1e-12{
                    return None; // Caso onde o pivô é nulo
                }
                x[i]=(y[i]-sigma)/upper[i][i];
            }

            Some(x)
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
            let mut x_new: vec![0.0f64; len];
            let mut x_prev = vec![0.0f64; len];
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
            let mut x_new: vec![0.0f64; len];
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

   pub fn residual_check(&mut self, vec<f64> x_sol) -> Bool{
    // TODO: indicará se a diferença entre Ax e B vai a zero
        let len=self.len;
        let mut perto: bool;
        let mut line_Ax: vec![0.0f64; len];
        for i in 0..len{
            for j in 0..len{
                line_Ax[i]=self.r#as[i][j].x_sol[i];
            }
        }

        let mut max=0.0f64;
        let mut diff=0.0;
        let mut u=0;
        for val in &line_Ax{
            for u in 0..len{diff=(*val-self.bs[u]).abs();}
            if(max<diff) {max=diff;}
        }

        if max<=eps{
            perto=true;
        }
        perto=false;

        if perto {println!("Solução para o sistema é aceitável");}

        perto;
   }

   // convergência para gauss seidel
   // itera pelo vetor beta
   // a procura de algum valor que seja 
   pub fn sassenfeld(&mut self) -> bool{
     let n=self.len;
     let mut beta: vec![0.0f64;n];
     for i in 0..n{
        let mut s=0.0;
        for j in 0..n{
            if(j!=i){
                if j<i {s=self.r#as[i][j].abs()*beta[j];}
                else {s=self.r#as[i][j].abs();}
            }
            beta[i]=s/self.r#as[i][i].abs();
        }   
        // iteramos em b para ir comparando com o 0
        // e verificar se obtemos algum valor que de fato
        // seja menor que 1
        beta.iter().cloned().fold(0.0, f64::max) < 1.0;
     }
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
    //(*)  alterada para usar endereço dedicado ao b
    pub fn new(matrix_name: &str, b_matrix: &str) -> Self {
        let mut as_path = Path::new(config::AS_PATH).join(matrix_name);
        as_path.set_extension("mtx");
        let bs_path = Path::new(config::BS_PATH).join(b_matrix); // * 

        let as_content = fs::read_to_string(&as_path).unwrap_or_else(|err| {
            panic!("Erro ao ler o arquivo da matriz A ({:?}): {err}", as_path);
        });

        let bs_content = fs::read_to_string(&b_matrix).unwrap_or_else(|err|{
            panic!("Erro ao ler o arquivo do vetor b ({:?}): {err}", bs_path);
        });

        let r#as = read::extract_sas(&as_content);
        let n = r#as.len();

        // let opt_bs_content = fs::read_to_string(&bs_path);
        let bs = read::handle_bs(bs_path, n);
        Self { r#as, bs, len: n }
    }
}

    

