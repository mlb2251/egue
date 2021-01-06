use colored::*;
//use std::convert::TryInto;


fn main() {
  // let f: fn(&i32) -> i32 = |x| x+1;
  // let ptr : *const fn(&i32) -> i32 = &f;
  // println!("{:?}",ptr);
  // enum Foo {
  //   A(i32),
  //   B(f32)
  // }
  // use Foo::*;
  // let x = vec![A(10), A(11), B(14.)];
  // fn func(x: &[Foo]) {
  //   for i in (*x).iter() {
  //     // *i = A(10);
  //   }
  //   //let () = A(10);
  // }
  // func(&x);
  // func(&x);
  //test();
  //repl();
  println!("Starting up");
  use bottom_up::Val::*;
  let env = vec![Int(3),IntList(vec![1,2,3,4,5])];
  run_bottom_up(env);
}

#[allow(dead_code)]
fn test(){
  use crate::simple::*;
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


#[allow(dead_code)]
fn repl() {
  use crate::simple::*;
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

// fn map(args: &[Val]) -> Val {
//   assert!(args.len() == 2);
//   let [f,xs] = args;
// }
// fn map(f:fn(&i32)->i32, xs:Vec<i32>) -> bottom_up::Val {
//   bottom_up::Val::IntList(xs.iter().map(|x|f(x)).collect())
// }
// fn map(args:(F, )) -> bottom_up::Val where
//   F: 
// {
//   bottom_up::Val::IntList(xs.iter().map(|x|f(x)).collect())
// }

fn run_bottom_up(env:Vec<bottom_up::Val>) {
  // $0 is env[0], $1 is env[1], etc
  use bottom_up::*;

  println!("building productions");
  let prods = {
    use bottom_up::dsl_funcs::*;
    use bottom_up::ReturnType::*;
    use bottom_up::Type::*;
    vec![
      Prod::new("map",    IntList.into(),       &[IntToInt,IntList],      map),
      Prod::new("add1",   IntToInt.into(),      &[],                      make_add1),
      Prod::new("mul2",   IntToInt.into(),      &[],                      make_mul2),
      Prod::new("$0",     Index(0),             &[],                      idx_0),
      Prod::new("$1",     Index(1),             &[],                      idx_1),
    ]
  };

  println!("building search state");
  let mut search_state = SearchState::new(prods, env);

  println!("running");
  search_state.run(10);

}

pub mod bottom_up {

  pub type Result = std::result::Result<Val,Error>;
  pub type DSLFunc = fn(&[&Val], &[Val]) -> Result;
  pub type Id = usize;
  #[derive(Debug, Copy, Clone)]
  pub enum Type {Int, IntList, IntToInt}
  pub enum ReturnType {Concrete(Type), Index(usize)}
  pub enum Error {
    Runtime,
    TypeError,
    IndexError,
    Other(String),
  }
  pub enum Val {
    // an actual value of an intermediate or final result
    // in the DSL
    Int(i32),
    IntList(Vec<i32>),
    IntToInt(fn(&i32) -> i32)
  }
  pub enum DisplayExpr {
    // just a class for printing out expressions
    Leaf(String),
    Node(String,Vec<DisplayExpr>)
  }
  #[derive(Default)]
  pub struct SearchState {
    // holds the current state of the search
    pub prods: Vec<Prod>, // production rules
    pub seen: std::collections::HashSet::<Val>, // set of values seen so far
    pub found_vecs: [Vec<Found>; 3],
    pub env: Vec<Val>,
    target_weight: usize,
  }
  pub struct Prod {
    // a production rule
    pub name: String,
    pub ty: ReturnType,
    pub args: [Option<Type>;3],
    pub weight: usize,
    pub func: DSLFunc,
    pub id: Option<Id>,
  }
  #[derive(Debug)]
  pub struct Found {
    // a constructed expression we found
    pub prod: Id,
    pub args: [Option<Id>;3],
    pub weight: usize,
    pub val: Val,
    pub id: Option<Id>,
  }

  impl SearchState {
    pub fn new(mut prods: Vec<Prod>, env: Vec<Val>) -> SearchState {
      for (i,prod) in prods.iter_mut().enumerate() {
        prod.id = Some(i);
      }
      SearchState {prods, env, ..Default::default()}
    }
    fn possible_values(&self, arg: Type) -> &Vec<Found> {
        match arg {
          Type::Int => &self.found_vecs[0],
          Type::IntList => &self.found_vecs[1],
          Type::IntToInt => &self.found_vecs[2],
        }
    }
    pub fn expr_of_found(&self, found: &Found) -> DisplayExpr {
      let args:Vec<Id> = found.args.iter()
        .filter_map(|&x|x)
        .collect();
      let prod = &self.prods[found.prod];
      if args.len() == 0 {
        return DisplayExpr::Leaf(prod.name.clone());
      }
      let found_vecs: Vec<&Vec<Found>> = prod.args.iter()
        .filter_map(|&x|x)
        .map(|ty|self.possible_values(ty))
        .collect();
      assert_eq!(found_vecs.len(),args.len());
      let args: Vec<DisplayExpr> = args.iter()
        .zip(found_vecs)
        .map(|(&id,found_vec)| &found_vec[id])
        .map(|found| self.expr_of_found(&found))
        .collect();
      
      return DisplayExpr::Node(prod.name.clone(),args)
    }

    pub fn check_limit(&self, prod:&Prod, args: &[&Found]) -> bool {
      let weight = prod.weight + args.iter().map(|found| found.weight).sum::<usize>();
      return weight <=self.target_weight;
    }
    pub fn try_add(&self, prod:&Prod, args: &[&Found]) -> Option<Found>{
      // check that weight is inbounds
      let weight = prod.weight + args.iter().map(|found| found.weight).sum::<usize>();
      if weight != self.target_weight { return None }
      // map Found to Val so they can be fed in
      let arg_vals : Vec<&Val> = args.iter().map(|found| &found.val).collect();
      // run the DSLFunc
      let res = (prod.func)(&arg_vals[..], &self.env[..]);
      let val = match res {
        Err(Error::TypeError) => panic!("we shouldnt never encouter type errors"),
        Err(_) => panic!("havent handled whatever this is"),
        Ok(val) => val,
      };
      //observational equivalence
      if self.seen.contains(&val) { return None }
      println!("discovered new val: {:?}", val);

      let mut arg_ids = [None;3];
      let id_vec: Vec<Id> = args.iter().map(|found| found.id).map(|x|x.unwrap()).collect();
      for (i,id) in id_vec.into_iter().enumerate() {
        arg_ids[i] = Some(id);
      }

      Some(Found {prod: prod.id.unwrap(), 
             args: arg_ids,
             weight: weight,
             val: val,
             id: None })
    }

    pub fn run(&mut self, weight: usize) {
      println!("run()");
      if weight <= self.target_weight { return }
      println!("passed weight check");
      for _ in self.target_weight..weight {
        self.step();
        println!("{:?}",self);
      }
      println!("done");
    }

    pub fn step(&mut self) {
      self.target_weight += 1;
      println!("Searching for weight={} expressions",self.target_weight);

      let mut to_add = Vec::<Found>::new();

      for prod in self.prods.iter() {
        let args:Vec<_> = prod.args.iter().filter(|x|x.is_some()).map(|&ty|self.possible_values(ty.unwrap())).collect();

        // I know this is terrifying code but let me explain
        // first of all this is just a fun quick project so its fine
        // second i wanted stuff all flat without function calls for speed since we really do know
        // that we only care about these 4 cases. If you try to get into stuff that doesnt
        // do top level branching then you start needing sentinel values and they need to type
        // check correctly so it gets annoying. Also maybe you could do this all with 3 nested
        // maps but idk.
        // TLDR: Just move the hell on and go work on something else now that this works

        // if you wanted you could write some bits as closures (tho also macros would be nice?):
          //let check = |args| {self.check_limit(prod,args)};
          //let insert = |args| {self.try_add(prod,args).map(|x|to_add.push(x));};

        match args.len() {
          0 => {self.try_add(prod,&[]).map(|x|to_add.push(x));},

          1 => {
            for arg0 in args[0].iter() {
              if !self.check_limit(prod,&[arg0]) { break }
              self.try_add(prod,&[arg0]).map(|x|to_add.push(x));
              //insert(&[arg0])
            }
          },

          2 => {
            for arg0 in args[0].iter() {
                if !self.check_limit(prod,&[arg0]) { break }
                for arg1 in args[1].iter() {
                  if !self.check_limit(prod,&[arg0,arg1]) { break }
                  self.try_add(prod,&[arg0,arg1]).map(|x|to_add.push(x));
                }
            }
          },
          3 => {
            for arg0 in args[0].iter() {
                if !self.check_limit(prod,&[arg0]) { break }
                for arg1 in args[1].iter() {
                  if !self.check_limit(prod,&[arg0,arg1]) { break }
                  for arg2 in args[1].iter() {
                    if self.check_limit(prod,&[arg0,arg1,arg2]) { break }
                    self.try_add(prod,&[arg0,arg1,arg2]).map(|x|to_add.push(x));
                  }
                }
            }
          },
          _ => panic!("didnt expect >3 args")
        }
      }


      // update with newly found values
      for mut found in to_add.into_iter(){
        let v = match found.val.ty() {
          Type::Int => &mut self.found_vecs[0],
          Type::IntList => &mut self.found_vecs[1],
          Type::IntToInt => &mut self.found_vecs[2],
        };
        found.id = Some(v.len()); // set the id of Found
        self.seen.insert(found.val.clone()); // mark Found.val as seen
        v.push(found);
      }
    }
  }
  impl std::fmt::Debug for SearchState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("SearchState")
       .field("prods",&self.prods.len())
       .field("seen",&self.seen.len())
       .field("found_ints",&self.found_vecs[0].len())
       .field("found_intlists",&self.found_vecs[1].len())
       .field("found_inttoints",&self.found_vecs[2].len())
       .field("env",&self.env.len())
       .field("target_weight",&self.target_weight)
       .finish()
    }
  }

  pub mod dsl_funcs {
    use super::*;
    use super::Val::*;
    use super::Error::*;
    pub fn map(args: &[&Val], _env: &[Val]) -> Result {
      if let [IntToInt(f),IntList(xs)] = args {
        Ok(IntList(xs.iter().map(|x|f(x)).collect()))
      } else { Err(TypeError) }
    }
    pub fn make_add1(args: &[&Val], _env: &[Val]) -> Result{
      if let [] = args {
        Ok(IntToInt(|x|x+1))
      } else { Err(TypeError) }
    }
    pub fn idx_0(args: &[&Val], env: &[Val]) -> Result{
      if let [] = args {
        // explicit clone is needed here. In stuff like map() we construct
        // a new value so you get a Val naturally, but here we just lookup
        // a &Val so we need to clone it into a fresh Val
        env.get(0).ok_or(IndexError).map(|v|v.clone())
      } else { Err(TypeError) }
    }
    pub fn idx_1(args: &[&Val], env: &[Val]) -> Result{
      if let [] = args {
        env.get(1).ok_or(IndexError).map(|v|v.clone())
      } else { Err(TypeError) }
    }
    // pub fn make_const(v: Val) -> DSLFunc{
    //   let v = v.clone();
    //   let f = |args: &[Val]| -> Result{
    //     if let [] = args {
    //       Ok(v)
    //     } else { Err(TypeError) }
    //   };
    //   f
    // }
    pub fn make_mul2(args: &[&Val], _env:&[Val]) -> Result{
      if let [] = args {
        Ok(IntToInt(|x|x*2))
      } else { Err(TypeError) }
    }
  }


  // pub type Lambda = fn(&[Val]) -> Val;


  impl Val {
    pub fn ty(&self) -> Type {
      match self {
        Val::Int(_) => Type::Int,
        Val::IntList(_) => Type::IntList,
        Val::IntToInt(_) => Type::IntToInt,
      }
    }
  }

  impl std::clone::Clone for Val {
    fn clone(&self) -> Val {
      match self {
        Val::Int(v) => Val::Int(v.clone()),
        Val::IntList(v) => Val::IntList(v.clone()),
        Val::IntToInt(f) => Val::IntToInt(f.clone()),
      }
    }
  }

  // a hash impl for Val that handles the IntToInt case
  // by hashing the underlying function pointer
  impl std::hash::Hash for Val {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
      match self {
        Val::Int(v) => v.hash(state),
        Val::IntList(v) => v.hash(state),
        Val::IntToInt(f) => {
          let ptr : *const fn(&i32) -> i32 = f;
          ptr.hash(state)
        }
      };
    }
  }
  // a hash impl for Val that handles the IntToInt case
  // by hashing the underlying function pointer
  impl std::cmp::PartialEq for Val {
    fn eq(&self, other: &Val) -> bool {
      match (self,other) {
        (Val::Int(v),Val::Int(o)) => v == o,
        (Val::IntList(v),Val::IntList(o)) => v == o,
        (Val::IntToInt(f),Val::IntToInt(o)) => {
          let ptr : *const fn(&i32) -> i32 = f;
          let ptr2 : *const fn(&i32) -> i32 = o;
          ptr == ptr2
        }
        _ => false,
      }
    }
  }
  impl std::cmp::Eq for Val {}

  impl std::fmt::Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Val::Int(x)  => write!(f,"Int({})",x),
        Val::IntList(x)  => write!(f,"Int({:?})",x),
        Val::IntToInt(_)  => write!(f,"IntToInt(cant_display)"),
      }
    }
  }


  impl Prod { 
    pub fn new(name:&str, ty: ReturnType, args_slice: &[Type], func: DSLFunc) -> Prod {
      if args_slice.len() > 3 {panic!("Too many args")};
      let mut args = [None;3];
      for (i,ty) in args_slice.iter().enumerate() {
        args[i] = Some(*ty);
      }
      Prod {name:String::from(name), ty, args, func, weight:1, id:None}
    }
  }

  impl From<Type> for ReturnType {
    fn from(t: Type) -> ReturnType {
      ReturnType::Concrete(t)
    }
  }


}




pub mod simple {
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
}

