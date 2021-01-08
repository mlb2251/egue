use super::Type;

pub enum Val {
  // an actual value of an intermediate or final result
  // in the DSL
  Int(i32),
  IntList(Vec<i32>),
  IntToInt(fn(&i32) -> i32),
  IntToIntToInt(fn(&i32,&i32) -> i32),
  IntToBool(fn(&i32) -> bool)
}

impl Val {
  pub fn ty(&self) -> Type {
    match self {
      Val::Int(_) => Type::Int,
      Val::IntList(_) => Type::IntList,
      Val::IntToInt(_) => Type::IntToInt,
      Val::IntToIntToInt(_) => Type::IntToIntToInt,
      Val::IntToBool(_) => Type::IntToBool,
    }
  }
}

impl std::clone::Clone for Val {
  fn clone(&self) -> Val {
    match self {
      Val::Int(v) => Val::Int(v.clone()),
      Val::IntList(v) => Val::IntList(v.clone()),
      Val::IntToInt(f) => Val::IntToInt(f.clone()),
      Val::IntToIntToInt(f) => Val::IntToIntToInt(f.clone()),
      Val::IntToBool(f) => Val::IntToBool(f.clone()),
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
      Val::IntToIntToInt(f) => {
        let ptr : *const fn(&i32,&i32) -> i32 = f;
        ptr.hash(state)
      }
      Val::IntToBool(f) => {
        let ptr : *const fn(&i32) -> bool = f;
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
      (Val::IntToIntToInt(f),Val::IntToIntToInt(o)) => {
        let ptr : *const fn(&i32,&i32) -> i32 = f;
        let ptr2 : *const fn(&i32,&i32) -> i32 = o;
        ptr == ptr2
      }
      (Val::IntToBool(f),Val::IntToBool(o)) => {
        let ptr : *const fn(&i32) -> bool = f;
        let ptr2 : *const fn(&i32) -> bool = o;
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
      Val::IntList(x)  => write!(f,"IntList({:?})",x),
      Val::IntToInt(_)  => write!(f,"IntToInt(cant_display)"),
      Val::IntToIntToInt(_)  => write!(f,"IntToIntToInt(cant_display)"),
      Val::IntToBool(_)  => write!(f,"IntToBool(cant_display)"),
    }
  }
}