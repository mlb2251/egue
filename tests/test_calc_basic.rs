use egue::calc;

#[test]
fn test(){
  use calc::*;
  let x = Expr::Binop(Box::new(Expr::Lit(Lit::Float(1.))),BOp::Add,Box::new(Expr::Lit(Lit::Float(2.))));
  let res = x.eval();
  println!("{:?}",res);
  let y: Expr = "(+ 2 2)".parse().unwrap();
  let res = y.eval();
  println!("{:?}",res);
  let y: Expr = "(+ 2 (* 2 2))".parse().unwrap();
  let res = y.eval();
  println!("{:?}",res);
}