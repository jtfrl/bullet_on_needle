use std::io::{self, Read, Write, Bufwriter}
use std::collections::VecDeque;

enum Token{
    Number(f64).
    Ident(String),
    Plus, Minus, Star, Slash, Chapeu
    LParen, RParen,
    Comma,
}
// falta algo pra log também

enum Expr{
    Number(f64),
    Var,
    BinOp(Box<Expr>, Op, Box<Expr>),
    Call(String, Box<Expr>)
}

// vai retornar a expressão a ser aplicada em tableFunction
fn convertMathF(f: String){
    let fun: Expr = f.trim().parse();
}

// recebe em string a função e calcula n valores de saída
// usamos pair aqui para poder contar com os valores 
// de cada entrada e de saida
//fn tableFunction(f: string) -> VecDeque<pair<f64>> {
fn tableFunction(expr: &Expr, x:f64) -> VecDeque<pair<f64>>{
    match expr{
        Expr::Number(n)=> *n,
        Expr::Var => x,
        Expr::BinOp(l, op, r)=>{
            let (l, r) = (tableFunction(l,x), tableFunction(r, x));
            match op {Op::Add=> l+r,
                      Op::Multiply=>l*r
                      /*TO-DO checar como continuar aqui*/}
        }
        Expr::Call(name, arg)=>{
            let v = tableFunction(arg, x);
            match name.as_str(){
                "sen" => v.sin(), "cos" => v.cos(),
                "tan" || "tg" => v.tan(), "sqrt" => v.sqrt(),
                "ln" => v.ln(),
                _ => panic!("Função desconhecida!");
            }
        }
    }
}

fn chasePosRoot(){
 bool posRaiz=false;


}


fn main(){

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap(); 
    let stdout = io::stdout();
    let mut out = Bufwriter::new(stdout.lock());

    // TODO: valor constante que seria a função convertida


}