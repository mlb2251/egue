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
}
struct Foo;

fn foo(x: &mut Foo) {
  bar(&x)
}
fn bar(x: &Foo) {

}


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

fn run_bottom_up(env:&Vec<bottom_up::Val>) {
  // $0 is env[0], $1 is env[1], etc
  use bottom_up::*;

  let prods = {
    use bottom_up::dsl_funcs::*;
    use bottom_up::Type::*;
    vec![
      Prod::new("map",    IntList,    &[IntToInt,IntList],      map),
      Prod::new("add1",   IntToInt,   &[],                      make_add1),
      Prod::new("mul2",   IntToInt,   &[],                      make_mul2),
      Prod::new("$0",     Index(0),   &[],                      idx_0),
      Prod::new("$1",     Index(1),   &[],                      idx_1),
    ]
  };

  let mut search_state = SearchState::new(prods);

  let max_weight = 10;
  // synth loop

}

pub mod bottom_up {

  pub type Result = std::result::Result<Val,Error>;
  pub type DSLFunc = fn(&[Val], &[Val]) -> Result;
  pub type Id = usize;
  #[derive(Debug, Copy, Clone)]
  pub enum Type {Int, IntList, IntToInt, Index(usize)}
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
  #[derive(Default)]
  pub struct SearchState {
    // holds the current state of the search
    pub prods: Vec<Prod>, // production rules
    pub seen: std::collections::HashSet::<Val>, // set of values seen so far
    pub found_vecs: [Vec<Found>; 3],
    dummy_vec: Vec<Found>,
    target_weight: usize,
  }
  pub struct Prod {
    // a production rule
    pub name: String,
    pub ty: Type,
    pub args: [Option<Type>;3],
    pub weight: usize,
    pub func: DSLFunc,
  }
  #[derive(Debug)]
  pub struct Found{
    // a constructed expression we found
    pub prod: Id,
    pub args: [Option<Id>;3],
    pub weight: usize,
    pub val: Val,
  }

  //  struct ProdIter<'a> {
  //    prod: &'a Prod,
  //    search_state: &'a SearchState,
  //    remapped_found_vecs: [&'a Vec<Found>; 3], // remapped so that the first one is for the first type etc
  //    weight: usize,
  //    indices: [usize; 3],
  //    curr_idx: u8, // either 0 1 or 2 corresponding to the 3 possible indices
  //    finished: bool,
  //  }
  
  //  impl<'a> ProdIter<'a> {
  //    pub fn new(prod: &'a Prod, search_state: &'a SearchState) -> ProdIter<'a> {
  //      ProdIter {prod, search_state, weight:search_state.target_weight, indices:[0,0,0]}
  //    }
  //  }
  // impl<'a> Iterator for ProdIter<'a> {
  //   type Item = [Option<&'a Found>;3];
  //   fn next(&mut self) -> Option<Self::Item> {
  //     if self.finished {return None;}
  //     /// ok lets grab the next valid set of 3 items
  //     /// starting by looking at the one we're inspecting now
  //     let [i,j,k] = self.indices;
  //     let [vi,vj,vk] = self.remapped_found_vecs;
  //     loop {
  //       let weight = vi[i].weight + vj[j].weight + vk[k].weight;
  //       if weight > self.weight {
  //         // we exceeded our weight, need to reset an index
  //         // to be precise, we need to move our current index back to zero, step back a single index, and increment it by one
  //         /// 
  //         if curr_idx == 0 {
  //           i = 0
  //         }
  //       }

  //     }


  //     self.remapped_found_vecs[0].get(self.indices[0]);
  //     ()
  //   }
  // }

  impl SearchState {
    pub fn new(prods: Vec<Prod>) -> SearchState {
      SearchState {prods, ..Default::default()}
    }
    fn possible_values(&self, arg: Type) -> &Vec<Found> {
        match arg {
          Type::Int => &self.found_vecs[0],
          Type::IntList => &self.found_vecs[1],
          Type::IntToInt => &self.found_vecs[2],
          _ => panic!("unexpected type ahhh"),
        }
    }

    pub fn check_limit(&self, prod:&Prod, args: &[&Found]) -> bool {
      let weight = prod.weight + args.iter().map(|found| found.weight).sum::<usize>();
      return weight <=self.target_weight;
    }
    pub fn try_add(&self, prod:&Prod, args: &[&Found]) -> Option<Found>{
      let weight = prod.weight + args.iter().map(|found| found.weight).sum::<usize>();
       if weight != self.target_weight { return None }
        //TODOTODOTODO
        panic!("ACTUALLY GO AND IMPLEMEMNT THE ADDITION HERE!!!");
    }

    pub fn run(&mut self) {
      println!("Searching for weight={} expressions",self.target_weight);

      let mut to_add = Vec::<Found>::new();

      for prod in self.prods.iter() {
        //let [arg1,arg2,arg3] = prod.args;
        // let x:Vec<_> = prod.args
        //   .iter()
        //   .filter(|x|x.is_some())
        //   .map(|x|x.unwrap()) // probably a better way to do this
        //   .map(|ty|self.possible_values(ty))
        //   .collect();
        
        // let it = ProdIter::new(&prod,&self);
        // for args in it {
        //   let [a0,a1,a2] = args;
          
        //   prod.func()
        // }
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




        // for arg0 in args[0].iter() {
        //   for arg1 in args[1].iter() {
        //     if arg0.weight + arg1.weight + prod.weight > self.target_weight {break;}
        //     for arg2 in args[2].iter() {
        //       let weight = arg0.weight + arg1.weight + arg2.weight + prod.weight;
        //       if weight > self.target_weight {break;}
        //       // TODO hmm so is this enough? do we only ever actually increment the innermost one before stepping and all
        //       // TODO skips are really just skips on the innermost iterator?

        //       // TODO watch out for cases with only two args? one arg? I suppose the third arg could be a sentinel value as 
        //       // TODO long as it points to a length-1 vector? Yeah just add a dummy vec that possible_values points to
        //       // This is actually quite a good call. Make it a Found thing with zero weight
        //       // hmm tho also it seems weird to use a sentinel since the value itself doesn't have any meaning
        //       // TODO ACK idk. Stop and think before you do anything, whatever you do
        //     }
        //   }
        // }
        
        
        // if let Some(ty) = prod.args[0] {
        //   let arg0 = self.possible_values(ty);
        //   if let Some(ty) = prod.args[1] {
        //     let arg1 = self.possible_values(ty);
        //     if let Some(ty) = prod.args[2] {
        //       let arg2 = self.possible_values(ty);



        //       let res = prod.func(&[]);
              
        //     }
        //   }
        // }

      }
    }
  }
  // impl Iterator for SearchState {
  //   type Item = ;
  //   fn next(&mut self) -> Option<Self::Item> {

  //   }
  // }

  pub mod dsl_funcs {
    use super::*;
    use super::Val::*;
    use super::Error::*;
    pub fn map(args: &[Val], _env: &[Val]) -> Result {
      if let [IntToInt(f),IntList(xs)] = args {
        Ok(IntList(xs.iter().map(|x|f(x)).collect()))
      } else { Err(TypeError) }
    }
    pub fn make_add1(args: &[Val], _env: &[Val]) -> Result{
      if let [] = args {
        Ok(IntToInt(|x|x+1))
      } else { Err(TypeError) }
    }
    pub fn idx_0(args: &[Val], env: &[Val]) -> Result{
      if let [] = args {
        // explicit clone is needed here. In stuff like map() we construct
        // a new value so you get a Val naturally, but here we just lookup
        // a &Val so we need to clone it into a fresh Val
        env.get(0).ok_or(IndexError).map(|v|v.clone())
      } else { Err(TypeError) }
    }
    pub fn idx_1(args: &[Val], env: &[Val]) -> Result{
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
    pub fn make_mul2(args: &[Val], _env:&[Val]) -> Result{
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


  impl Prod
  { 
    pub fn new(name:&str, ty: Type, args_slice: &[Type], func: DSLFunc) -> Prod {
      if args_slice.len() > 3 {panic!("Too many args")};
      let mut args = [None;3];
      for (i,ty) in args_slice.iter().enumerate() {
        args[i] = Some(*ty);
      }
      Prod {name:String::from(name), ty, args, func, weight:1}
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

