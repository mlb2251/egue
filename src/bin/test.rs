use symbolic_expressions::Sexp;
fn main() -> () {
  let x = Expr::Binop(&Expr::Lit(Lit::Float(1.)),BOp::Add,&Expr::Lit(Lit::Float(2.)));
  let res = x.eval();
  println!("{:?}",res);
  let y: Expr = "(+ 2 2)".parse().unwrap();
  let res = y.eval();
  println!("{:?}",res);
}

enum BOp {Add, Sub, Mul, Div}

enum Lit {
  Float(f32),
}

enum Expr<'a> {
  Binop(&'a Expr<'a>, BOp, &'a Expr<'a>),
  Lit(Lit),
}

impl<'a> Expr<'a> {
  fn eval(&self) -> Result<f32,String>{
      match self {
        Expr::Lit(lit) => match lit {
          Lit::Float(v) => Ok(v.clone())
        },
        Expr::Binop(e1,op,e2) => {
          let v1 = e1.eval()?;
          let v2 = e2.eval()?;
            match op {
            BOp::Add => Ok(v1 + v2),
            BOp::Sub => Ok(v1 - v2),
            BOp::Mul => Ok(v1 * v2),
            BOp::Div => Ok(v1 / v2),
          }
        }

      }
  }
}


impl<'a> std::str::FromStr for Expr<'a> {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let sexp = symbolic_expressions::parser::parse_str(s.trim()).map_err(|e|e.to_string())?;
    
    // fn expr_of_sexp(sexp: &Sexp) -> Result<Expr,String> {
    //   match sexp {
    //     Sexp::Empty => Err("empty line".into()),
    //     Sexp::String(s) => {
    //       let val = s.parse::<f32>().map_err(|_| format!("Expected float literal but got {} ",s))?;
    //       Ok(Expr::Lit(Lit::Float(val)))
    //     }
    //     Sexp::List(vec) => {
    //       if vec.len() != 3 {return Err(format!("Expected 3 items in list but got {} for sexp {:?}",vec.len(),vec))}
    //       let op = vec.first().unwrap(); // guaranteed by vec.len() check
    //       let op_str = op.string().map_err(|_| format!("expected string for operator but got {:?}",op))?;
    //       let bop = match op_str.as_str() {
    //         "+" => BOp::Add,
    //         "-" => BOp::Sub,
    //         "*" => BOp::Mul,
    //         "/" => BOp::Div,
    //         _ => return Err(format!("unrecognized operator {}",op_str))
    //       };
    //       let args: Result<Vec<_>,_> = (&vec[1..]).iter().map(|arg| expr_of_sexp(arg)).collect();
    //       let args = args?; //todo see if theres a way to put this on prev line
    //       Ok(Expr::Binop(&args[0],bop,&args[1]))
    //     }
    //   }
    // }
    // expr_of_sexp(&sexp);
    let v = if true {1.} else {2.};
    let x = Expr::Binop(&Expr::Lit(Lit::Float(v)),BOp::Add,&Expr::Lit(Lit::Float(2.)));
    Ok(x)
  }
}


