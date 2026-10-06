// talvez criar uma classe?
pub mod lalg;
use lalg: *;

pub fn elim_matrix(a: &Vec<Vec<f64,f64>>, b: &[f64]) -> Vec<Vec<f64>>{
    let mut el_pivot: Lalg{
        // todo aplicar a função de create pivot
        amatrix: a,
    }
    for j in (0..n).rev(){
        //TODO controle 
        //if el.pivot[j] == 0 panic!
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