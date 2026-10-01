/* 
    objetivo:
        criar uma classe que faça os objetos 
        intermediários dos cálcuos
        e controlar precisões (evitar que calc. 
        intermediários) sejam de baixa precisão
*/

#[derive(Debug, Clone, Eq, PartialEq)]

pub struct Lalg{
    pub pivot: [f64],
    pub amatrix: Vec<Vec<f64>>
}

// det: for i in (0..n)
// prod=1
// prod*=pivot[i];
// prod

