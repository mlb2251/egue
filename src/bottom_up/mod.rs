pub mod deepcoder;
mod val;
mod search_state;

pub use val::Val;
pub use search_state::SearchState;

pub type Result = std::result::Result<Val,Error>;
pub type DSLFunc = fn(&[&Val], &[Val]) -> Result;
pub type Id = usize;
#[derive(Debug, Copy, Clone)]
pub enum Type {Int, IntList, IntToInt, IntToIntToInt, IntToBool}
pub enum ReturnType {Concrete(Type), Index(usize)}
pub enum Error {
  RuntimeError,
  TypeError,
}
pub enum DisplayExpr {
  // just a class for printing out expressions
  Leaf(String),
  Node(String,Vec<DisplayExpr>)
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
#[derive(Debug,Clone)]
pub struct Found {
  // a constructed expression we found
  pub prod: Id,
  pub args: [Option<Id>;3],
  pub weight: usize,
  pub val: Val,
  pub id: Option<Id>,
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

impl std::fmt::Debug for DisplayExpr {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::Leaf(s) => write!(f,"{}",s),
      Self::Node(s,v) => {
        let args: Vec<String>  = v.iter().map(|e|format!("{:?}",e)).collect();
        write!(f,"({} {})", s, args.join(" "))
      }
    }
  }
}
