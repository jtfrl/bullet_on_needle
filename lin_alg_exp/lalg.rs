/* 
    objetivo:
        criar uma classe que faça os objetos 
        intermediários dos cálcuos
        e controlar precisões (evitar que calc. 
        intermediários) sejam de baixa precisão
*/

#[derive(Debug, Clone, Eq, PartialEq)]

pub struct Lalg{
    pub pivot: Vec<f64>,
    pub amatrix: Vec<Vec<f64>>
    pub sign: f64 //paridade de sinal se houver troca de linha
}

impl Lalg{
    pub fn new()(amatrix: Vec<Vec<f64>>) -> Self{
        Lalg{
            amatrix,
            pivot: Vec::new(),
            sign: 1.0
        }
    }

    pub fn det(line_c:usize) -> f64{
        let mut prod:f64;
       /*  for i in 0..pivot.len(){
            prod*=pivot[i];
        }

        prod */
        swap_line(line_c);
        self.sign*self.pivot.iter().product::<f64
    }


    /* talvez seja melhor a det ser assim:
    pub fn det(&self) -> f64{
        self.sign*self.pivot.iter().product::<f64>()
    } 

     */


    pub fn create_pivot() -> vec<f64>{
        let mut pivot: vec<f64>;
        for j in 0..amatrix.len(){
            for i in 0..amatrix.len(){
                pivot[j]=amatrix[i][j]/amatrix[i][i];
            }
        }

        pivot
    }

    pub fn swap_line(line_c:usize){
        if line_c%2==0{ continue;}
        else{
            self.sign=-1.0;
        }
    }
}

