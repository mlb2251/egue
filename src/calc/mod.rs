use symbolic_expressions::Sexp;

#[derive(Debug)]
pub enum BOp {Add, Sub, Mul, Div}

#[derive(Debug)]
pub enum Lit {
  Float(f32),
  Int(i32),
}

#[derive(Debug,Copy,Clone)]
pub struct SeenVal(Val);

#[derive(Debug,Copy,Clone)]
pub enum Val {Float(f32), Int(i32)}

const HASH_PREC: f32 = 10000.;
fn int_of_float(x:f32) -> i32 {
  (x * HASH_PREC) as i32
}

impl std::hash::Hash for SeenVal {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    // a hash implementation that does int(float * 10000) to handle hashing floats
    match self.0 {
      Val::Int(v) => v.hash(state),
      Val::Float(v) => {
        int_of_float(v).hash(state);
      }
    };
  }
}

impl std::cmp::PartialEq for SeenVal {
  fn eq(&self, other: &SeenVal) -> bool {
    match (self.0,other.0) {
      (Val::Int(v1), Val::Int(v2)) => v1 == v2,
      (Val::Float(v1), Val::Float(v2)) => {
        // in order to uphold the promise of `Eq` later so we 
        // can hash floats, we gotta modify it so that NaN == NaN
        // when it comes to Vals at least
        // (honestly no idea if this matters maybe its a bad idea)
        if v1.is_nan() && v2.is_nan() {return true}
        v1 == v2
      }
      _ => false,
    }
  }
}
impl std::cmp::Eq for SeenVal { }

#[derive(Debug)]
pub enum Expr {
  Binop(Box<Expr>, BOp, Box<Expr>),
  Lit(Lit),
}

impl Expr {
  pub fn eval(&self) -> Result<Val,String>{
      match self {
        Expr::Lit(lit) => match lit {
          Lit::Float(v) => Ok(Val::Float(*v)),
          Lit::Int(v) => Ok(Val::Int(*v))
        },
        Expr::Binop(e1,op,e2) => {
          let v1 = e1.eval()?;
          let v2 = e2.eval()?;
          match (v1,v2) {
            (Val::Float(v1), Val::Float(v2)) => match op {
              BOp::Add => Ok(Val::Float(v1 + v2)),
              BOp::Sub => Ok(Val::Float(v1 - v2)),
              BOp::Mul => Ok(Val::Float(v1 * v2)),
              BOp::Div => Ok(Val::Float(v1 / v2)),
            },
            (Val::Int(v1), Val::Int(v2)) => match op {
              BOp::Add => Ok(Val::Int(v1 + v2)),
              BOp::Sub => Ok(Val::Int(v1 - v2)),
              BOp::Mul => Ok(Val::Int(v1 * v2)),
              BOp::Div => Ok(Val::Int(v1 / v2)),
            },
            _ => Err(format!("unsupported types for {:?}: {:?} and {:?} during eval of {:?}",op,v1,v2,self))
          }
        }
      }
  }
}


impl std::str::FromStr for Expr {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let sexp = symbolic_expressions::parser::parse_str(s.trim()).map_err(|e|e.to_string())?;
    
    fn expr_of_sexp(sexp: &Sexp) -> Result<Expr,String> {
      match sexp {
        Sexp::Empty => Err("empty line".into()),
        Sexp::String(s) => {
          if s.contains(".") { // assume Float
            let val = s.parse::<f32>().map_err(|_| format!("Expected float literal but got {} ",s))?;
            Ok(Expr::Lit(Lit::Float(val)))
          } else { // assume int
            let val = s.parse::<i32>().map_err(|_| format!("Expected int literal but got {} ",s))?;
            Ok(Expr::Lit(Lit::Int(val)))
          }
        }
        Sexp::List(vec) => {
          if vec.len() != 3 {return Err(format!("Expected 3 items in list but got {} for sexp {:?}",vec.len(),vec))}
          let op = vec.first().unwrap(); // guaranteed by vec.len() check
          let op_str = op.string().map_err(|_| format!("expected string for operator but got {:?}",op))?;
          let bop = match op_str.as_str() {
            "+" => BOp::Add,
            "-" => BOp::Sub,
            "*" => BOp::Mul,
            "/" => BOp::Div,
            _ => return Err(format!("unrecognized operator {}",op_str))
          };
          let args: Result<Vec<_>,_> = (&vec[1..]).iter().map(|arg| expr_of_sexp(arg)).collect();
          let mut args = args?; //todo see if theres a way to put this on prev line
          Ok(Expr::Binop(Box::new(args.remove(0)),bop,Box::new(args.remove(0))))
        }
      }
    }
    expr_of_sexp(&sexp)
  }
}

pub fn repl() {
  use colored::*;
  let mut rl = rustyline::Editor::<()>::new();
  loop {
    let line = match rl.readline(">>> ") {
      Ok(line) => line,
      Err(e) => match e {
        rustyline::error::ReadlineError::Interrupted => continue,
        rustyline::error::ReadlineError::Eof => break,
        e => panic!("unexpected error: {:?}",e),
      }
    };
    rl.add_history_entry(line.as_str());
    if line.trim().is_empty() {continue}
    let e:Expr = match line.parse() {
      Ok(e) => e,
      Err(e) => {println!("Error during parse: {}",e.red()); continue},
    };
    let res = match e.eval(){
      Ok(v) => v,
      Err(e) => {println!("Error during eval: {}",e.red()); continue}, 
    };
    println!("{:?}",res);
  }

}