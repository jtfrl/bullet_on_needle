pub mod lalg;
use lalg: *;

pub fn elim_matrix(a: &Vec<Vec<f64,f64>>, b: &[f64]) -> Vec<Vec<f64>>{
    let mut el_pivot: Lalg{
        // todo aplicar a função de create pivot
        amatrix: a,
        pivot: self.pivot.clear(),
        signal: -1
    }
  
   let tam=el_pivot.amatrix.len();


   // === busca de linhas onde há os maiores valores
   for j in 0..tam{ //> usamos j para fazer referência a coluna
        let max_l=j;
        let max_val=el_pivot.amatrix[j][j].abs();
        for k in j+1..tam{
            if el_pivot.amatrix[i][k].abs()>max_val {
                max_val=el_pivot.amatrix[i][k].abs();
                max_l=k;
            }
        }
    //for j in 0..tam{}

    // === troca de casos de pivos
    if (max_l!=j){
        let count_line_c=0; // conta mudanças de linha
        el_pivot.amatrix.swap(j, max_l);
        count_line_c+=1;
    }

    swap_line(count_line_c);

    let pivot_found=el_pivot.amatrix[j][j];
    el_pivot.pivos.push(pivot_found);

    if pivot_found.abs() < 1e-12 {continue;} //pulamos números que não irrelevantes

     /*  for _j in (0..n).rev(){
       // apicar create_factor
       for i in (0..tam).rev(){
            el_pivot.amatrix[i][_j]-=factor*amatrix[j][_j];
       }
    } */


   }

   
  
}

// TODO checar se funciona para outros tipos de matrizes triangulares
pub fn retro_subs(a: &Vec<Vec<f64) -> Option<Vec<f64>>{
    let n = b.len();

    assert_eq!(a.len(), n, "A e b tem que ter tamanhos iguais");

    // por enquanto usando 0.0f64; avaliar precisão
    let mut x=vec![0.0f64; n]; // vetor de pesos que são solução do sistema

    for i in (0..n).rev(){
        let mut soma=0.0f64;
        for j in (i+1)..n{
            soma+=a[i][j]*x[j]; // somatório dos valores para trabalhar com termo independente
        }
        if a[i][i]==0.0{
            return None; 
        }
        // cada valor obitdo em soma
        // é usado para chegar-se às soluções
        x[i]=(b[i]-soma)/a[i][i];
    }

    Some(x)
}


// TODO 
// fatoracao LU