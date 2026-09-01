use std::io::{self, BufRead};
use std::collections::VecDeque;
//use micromath::F32Ext; 
//use std::str::FromStr;

// 'derive' dá instrução para o 
// compilador poder fazer comparações se necessário
#[derive(Debug, Clone, PartialEq)] 
pub enum Op{
    Add, Sub, Mul, Div, Pow
}

/* COMPILADOR NÃO RECONHECE ISSO COMO ALGO ÚTIL (?)
impl Op{
    fn precedencia(&self) -> u8{
        match self{
            Op::Add | Op::Sub => 1,
            Op::Mul | Op::Div => 2,
            Op::Pow => 3,
        }
    }

    fn right_op(&self) -> bool{
        matches!(self, Op::Pow) // caso o operador 
                                // não for conforme os de 1 e de 2
                                // manda para 'pow'
    }
}
*/    

#[derive(Debug, Clone, PartialEq)] 
pub enum Token{
    Number(f64),
    Ident(String),
    Plus, Minus, Star, Slash, Chapeu,
    LParen, RParen,
    Comma,
}

#[derive(Debug, Clone)] 
enum Expr{
    Number(f64),
    Var,
    BinOp(Box<Expr>, Op, Box<Expr>),
    Call(String, Box<Expr>)
}



impl std::str::FromStr for Expr{ 
    type Err=String; 
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        let clean = s.replace(' ',"");
        let mut chars=clean.chars().peekable(); //ajuda na iteração da string
        Self::parse_add_sub(&mut chars)
    }
}

impl Expr{
    fn parse_add_sub(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Expr, String>{
        let mut esq=Self::parse_mul_div(chars)?;
        while let Some(&ch) = chars.peek(){ //> enquanto houver caracteres para análise
            if ch=='+' || ch =='-'{
                chars.next();
                let dir=Self::parse_mul_div(chars)?;
                let op = if ch == '+' {Op::Add} else {Op::Sub};
                esq = Expr::BinOp(Box::new(esq), op, Box::new(dir));
            }else{break;}
        }

        Ok(esq)
    }
 

    fn parse_mul_div(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Expr, String>{
        let mut esq=Self::parse_primary(chars)?; //> ajuda a fazer wrap
        while let Some(&ch) = chars.peek(){
            if ch=='*' || ch == '/'{
                chars.next();
                let dir=Self::parse_add_sub(chars)?;
                let op = if ch == '*' {Op::Mul} else {Op::Div};
                esq = Expr::BinOp(Box::new(esq), op, Box::new(dir));
            }
            else{break;}
        }

        Ok(esq)
    }
    
    fn parse_primary(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Expr, String> {
        match chars.peek().copied() {
            Some('(') => {
                chars.next();
                let expr = Self::parse_add_sub(chars)?;
                if chars.next() == Some(')') { Ok(expr) } else { Err("Falta ')'".into()) }
            }
            Some(ch) if ch.is_ascii_digit() => {
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' { num.push(c); chars.next(); } else { break; }
                }
                Ok(Expr::Number(num.parse().unwrap()))
            }
            Some(ch) if ch.is_ascii_alphabetic() => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphabetic() { ident.push(c); chars.next(); } else { break; }
                }
                if ident == "x" || ident == "X" {
                    Ok(Expr::Var)
                } else if chars.peek() == Some(&'(') {
                    chars.next(); // consume '('
                    let arg = Self::parse_add_sub(chars)?;
                    if chars.next() == Some(')') { Ok(Expr::Call(ident, Box::new(arg))) } else { Err("Falta ')'".into()) }
                } else { Err(format!("Identificador desconhecido: {ident}")) }
            }
            _ => Err("ATENÇÃO: token inválido. Por favor, corrija a expressão".into()),
        }
    }

}

/* // vai retornar a expressão a ser aplicada em eval
fn convertMathF(f: String) -> Result: Expr{
    let fun: Expr = f.trim().parse();

} */

// recebe em string a função e ajuda na conversão dos operadores
// em string
// usamos pair aqui para poder contar com os valores 
// de cada entrada e de saida
fn eval(expr: &Expr, x: &f64) -> f64{
    match expr{
        Expr::Number(n)=> *n,
        Expr::Var => *x,
        Expr::BinOp(l, op, r)=>{
            let (l, r) = (eval(l,x), eval(r, x));
            match op {Op::Add=> l+r,
                      Op::Mul=>l*r,
                      Op::Div=>l/r,
                      Op::Sub=>l-r,
                      Op::Pow=>l.powf(r),
                      //_ => unreachable!(),
                      }
        }
        Expr::Call(name, arg)=>{
            let v = eval(arg, x);
            match name.as_str(){
                "sen" => v.sin(), "cos" => v.cos(),
                "tan" | "tg" => v.tan(), "sqrt" => v.sqrt(),
                "ln" => v.ln(),
                _ => panic!("Função desconhecida!"),
            }
        }
    }
}

fn table_function(_expr: Expr, input: &[f64]) -> VecDeque<(f64, f64)>{
    input
        .iter()
        .map(|&x| (x, eval(&Expr::Var, &x))) //> mapeamento de valores da função
        .collect() 
}

fn chase_pos_root(points: &VecDeque<(f64,f64)>){
    //bool posRaiz=false;

    let mut intervalos=Vec::new();

    for i in 0..points.len().saturating_sub(1){
        let(x1, y1) = points[i];
        let(x2, y2) = points[i+1];
        if y1==0.0||(y1.signum()!=y2.signum() && y1!=0.0 && y2!=0.0){
            intervalos.push((x1, x2, y1, y2));
        }
    }

    println!("Quantidade de possibilidades de raízes: {}", intervalos.len());
    for (x1, x2, y1, y2) in intervalos{
        println!(" Intervalo entre x={:.4} (y={:.4}) e x={:.4} (y={:.4})", x1, x2, y1, y2);
    }
}

fn main() -> io::Result<()>{ //> mostrará um resultado a partir da função

    let stdin=io::stdin();
    let mut r = stdin.lock(); //> leitor de dados

    println!("Insira a função matemática\n");
    let mut line=String::new();
    r.read_line(&mut line)?;
    
    if line.trim().is_empty(){
        println!("Vazio. Tente novamente com uma função válida.\n");
        return Ok(()); //> strings vazias serão puladas
    }
    
    let u_expr: Expr=match line.trim().parse(){
        Ok(e)=>e,
        Err(_err)=>{
            println!("Erro de análise da função!");
            return Ok(());
        }
    };


    println!("Agora, insira os 10 valores para entrada (separados por espaço) \n");
    let mut values_for_x=Vec::with_capacity(10);

    while values_for_x.len()<10 {
        let mut buf=String::new();
        if r.read_line(&mut buf)? == 0 {break;}

        for word in buf.split_whitespace(){
            if let Ok(num)=word.parse::<f64>(){
                values_for_x.push(num);
                if values_for_x.len() == 10 {break;}
            }
        }
    }

    if values_for_x.len()<10{
        println!("Menos de 10 valores inseridos para a variável.");
        return Ok(());
    }

    let table = table_function(u_expr, &values_for_x);

    println!("\n\n --- TABELA DE VALORES (x, F(x)) ---");
    for(x,y) in &table{
        // utilizamos formatação de 7 caracteres de saída na tabela
        // e precisão de 3 casas
        println!("f({x:>7.3}) == {y:>10.4}"); 
    }

    println!("\n\n Análise de ráizes:");
    chase_pos_root(&table);

    Ok(())
}