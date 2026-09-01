use std::io::{self, Read, Write, BufWriter}
use std::collections::VecDeque;
use micromath::F32Ext // checar se há um uso mais apriomorado como f64

pub enum Token{
    Number(f64).
    Ident(String),
    Plus, Minus, Star, Slash, Chapeu
    LParen, RParen,
    Comma,
}

pub enum Op{
    Add, Sub, Mul, Div, Pow
}

pub enum Expr{
    Number(f64),
    Var,
    BinOp(Box<Expr>, Op, Box<Expr>),
    Call(String, Box<Expr>)
}

// vai retornar a expressão a ser aplicada em eval
pub fn convertMathF(f: String){
    let fun: Expr = f.trim().parse();

    impl std::str::FromStr for Expr{ 
        type Err=String; 
        fn from_str(s: &str) -> Result;
    }
}

// recebe em string a função e ajuda na conversão dos operadores
// em string
// usamos pair aqui para poder contar com os valores 
// de cada entrada e de saida
pub fn eval(expr: &Expr, inputs: &[f64]) -> VecDeque<(f64, f64)>{
    match expr{
        Expr::Number(n)=> *n,
        Expr::Var => x,
        Expr::BinOp(l, op, r)=>{
            let (l, r) = (eval(l,x), eval(r, x));
            match op {Op::Add=> l+r,
                      Op::Mul=>l*r,
                      Op::Div=>l/r,
                      Op::Sub=>l-r,
                      _ => unreachable!(),
                      }
        }
        Expr::Call(name, arg)=>{
            let v = eval(arg, x);
            match name.as_str(){
                "sen" => v.sin(), "cos" => v.cos(),
                "tan" | "tg" => v.tan(), "sqrt" => v.sqrt(),
                "ln" => v.ln(),
                _ => panic!("Função desconhecida!");
            }
        }
    }
}

pub fn table_function(expr: Expr, input: &[f64]) -> VecDeque<(f64, f64)>{
    input
        .iter()
        .map(|&x|, (x, eval(expr, x)))
        .collect()
}

pub fn chasePosRoot(points: &[f64]) -> bool{
 bool posRaiz=false;
    points
        .iter()
        //TO-DO condicional que irá checar
        // sinal de positivo ou de negativo aqui
 


}

//TODO?? precisa de pub aq??
fn main(){

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap(); 
    let stdout = io::stdout();
    let mut out = Bufwriter::new(stdout.lock());

    // TODO: valor constante que seria a função convertida


}