use super::*;
use super::Val::*;
use super::Error::*;
use std::convert::{TryFrom,TryInto};

pub fn get_prods(num_idxs: usize) -> Vec<Prod> {
  {
    if num_idxs > 3 {panic!("please run with <=3 inputs. This restriction could be lifted i just havent done that")};
    use super::ReturnType::*;
    use super::Type::*;
    vec![
      Prod::new("$0",     Index(0),             &[],                      idx_0),
      Prod::new("$1",     Index(1),             &[],                      idx_1),
      Prod::new("$2",     Index(2),             &[],                      idx_2),

      Prod::new("head",    Int.into(),       &[IntList],      head),
      Prod::new("last",    Int.into(),       &[IntList],      last),
      Prod::new("take",    Int.into(),       &[Int,IntList],      take),
      Prod::new("drop",    Int.into(),       &[Int,IntList],      drop),
      Prod::new("access",    Int.into(),       &[Int,IntList],      access),
      Prod::new("minimum",    Int.into(),       &[IntList],      minimum),
      Prod::new("maximum",    Int.into(),       &[IntList],      maximum),
      Prod::new("reverse",    IntList.into(),       &[IntList],      reverse),
      Prod::new("sort",    IntList.into(),       &[IntList],      sort),
      Prod::new("sum",    Int.into(),       &[IntList],      sum),

      Prod::new("map",    IntList.into(),       &[IntToInt,IntList],      map),
      Prod::new("filter", IntList.into(),       &[IntToBool,IntList],     filter),
      Prod::new("count", Int.into(),       &[IntToBool,IntList],     count),
      Prod::new("zipwith", IntList.into(),       &[IntToIntToInt,IntList,IntList],     zipwith),
      Prod::new("scanl1", IntList.into(),       &[IntToIntToInt,IntList],     scanl1),

      Prod::new("(+1)",   IntToInt.into(),      &[],                      make_add1),
      Prod::new("(-1)",   IntToInt.into(),      &[],                      make_sub1),
      Prod::new("(*2)",   IntToInt.into(),      &[],                      make_mul2),
      Prod::new("(/2)",   IntToInt.into(),      &[],                      make_div2),
      Prod::new("(*-1)",   IntToInt.into(),      &[],                      make_negate),
      Prod::new("(**2)",   IntToInt.into(),      &[],                      make_square),
      Prod::new("(*3)",   IntToInt.into(),      &[],                      make_mul3),
      Prod::new("(/3)",   IntToInt.into(),      &[],                      make_div3),
      Prod::new("(*4)",   IntToInt.into(),      &[],                      make_mul4),
      Prod::new("(/4)",   IntToInt.into(),      &[],                      make_div4),


      Prod::new("(>0)",IntToBool.into(),      &[],                   make_is_gt0),
      Prod::new("(<0)",IntToBool.into(),      &[],                   make_is_lt0),
      Prod::new("is_even",IntToBool.into(),      &[],                   make_is_even),
      Prod::new("is_odd",IntToBool.into(),      &[],                   make_is_odd),

      Prod::new("(+)",IntToIntToInt.into(),      &[],                   make_add),
      Prod::new("(-)",IntToIntToInt.into(),      &[],                   make_sub),
      Prod::new("(*)",IntToIntToInt.into(),      &[],                   make_mul),
      Prod::new("(min)",IntToIntToInt.into(),      &[],                   make_min),
      Prod::new("(max)",IntToIntToInt.into(),      &[],                   make_max),
    ]
  }

}



pub fn idx_0(args: &[&Val], env: &[Val]) -> Result{
  if let [] = args {
    // explicit clone is needed here. In stuff like map() we construct
    // a new value so you get a Val naturally, but here we just lookup
    // a &Val so we need to clone it into a fresh Val
    env.get(0).ok_or(RuntimeError).map(|v|v.clone())
  } else { Err(TypeError) }
}
pub fn idx_1(args: &[&Val], env: &[Val]) -> Result{
  if let [] = args {
    env.get(1).ok_or(RuntimeError).map(|v|v.clone())
  } else { Err(TypeError) }
}
pub fn idx_2(args: &[&Val], env: &[Val]) -> Result{
  if let [] = args {
    env.get(2).ok_or(RuntimeError).map(|v|v.clone())
  } else { Err(TypeError) }
}

pub fn head(args: &[&Val], _env: &[Val]) -> Result {
  if let [IntList(xs)] = args {
    xs.first().ok_or(RuntimeError).map(|v|Int(v.clone()))
  } else { Err(TypeError) }
}
pub fn last(args: &[&Val], _env: &[Val]) -> Result {
  if let [IntList(xs)] = args {
    xs.last().ok_or(RuntimeError).map(|v|Int(v.clone()))
  } else { Err(TypeError) }
}
pub fn take(args: &[&Val], _env: &[Val]) -> Result {
  if let [Int(n),IntList(xs)] = args {
    let n = usize::try_from(*n).map_err(|_| RuntimeError)?; // turn n < 0 into runtime error
    // per deepcoder, we are okay with the normal take() behavior
    // where n > len() just returns the full original vector
    Ok(IntList(xs.iter().take(n).copied().collect()))
  } else { Err(TypeError) }
}
pub fn drop(args: &[&Val], _env: &[Val]) -> Result {
  if let [Int(n),IntList(xs)] = args {
    let n = usize::try_from(*n).map_err(|_| RuntimeError)?; // turn n < 0 into runtime error
    // per deepcoder if n > len() we return an empty vector
    Ok(IntList(xs.iter().skip(n).copied().collect()))
  } else { Err(TypeError) }
}
pub fn access(args: &[&Val], _env: &[Val]) -> Result {
  if let [Int(n),IntList(xs)] = args {
    let n = usize::try_from(*n).map_err(|_| RuntimeError)?; // turn n < 0 into runtime error
    xs.get(n).ok_or(RuntimeError).map(|v|Int(v.clone()))
  } else { Err(TypeError) }
}
pub fn minimum(args: &[&Val], _env: &[Val]) -> Result {
  if let [IntList(xs)] = args {
    xs.iter().min().ok_or(RuntimeError).map(|v|Int(v.clone()))
  } else { Err(TypeError) }
}
pub fn maximum(args: &[&Val], _env: &[Val]) -> Result {
  if let [IntList(xs)] = args {
    xs.iter().max().ok_or(RuntimeError).map(|v|Int(v.clone()))
  } else { Err(TypeError) }
}
pub fn reverse(args: &[&Val], _env: &[Val]) -> Result {
  if let [IntList(xs)] = args {
    Ok(IntList(xs.iter().rev().copied().collect()))
  } else { Err(TypeError) }
}
pub fn sort(args: &[&Val], _env: &[Val]) -> Result {
  if let [IntList(xs)] = args {
    let mut res = xs.clone();
    res.sort();
    Ok(IntList(res))
  } else { Err(TypeError) }
}
pub fn sum(args: &[&Val], _env: &[Val]) -> Result {
  if let [IntList(xs)] = args {
    // Iterator::sum() can overflow (panic during debug, silent during release)
    //Ok(Int(xs.iter().sum::<i32>()))
    xs.iter().try_fold(0i32,|acc,next|acc.checked_add(*next).ok_or(RuntimeError)).map(|ys|Int(ys))
  } else { Err(TypeError) }
}

pub fn map(args: &[&Val], _env: &[Val]) -> Result {
  if let [IntToInt(f),IntList(xs)] = args {
    xs.iter().map(|x|f(x)).collect::<LambdaResult<_>>().map(|v|IntList(v))
    //Ok(IntList(xs.iter().map(|x|f(x)).collect()))
  } else { Err(TypeError) }
}
pub fn filter(args: &[&Val], _env: &[Val]) -> Result {
  if let [IntToBool(f),IntList(xs)] = args {
    Ok(IntList(xs.iter().filter(|x|f(x)).copied().collect()))
  } else { Err(TypeError) }
}
pub fn count(args: &[&Val], _env: &[Val]) -> Result {
  if let [IntToBool(f),IntList(xs)] = args {
    // panic if this i32 -> usize conversion fails bc it should never...
    Ok(Int(xs.iter().filter(|x|f(x)).count().try_into().unwrap()))
  } else { Err(TypeError) }
}
pub fn zipwith(args: &[&Val], _env: &[Val]) -> Result {
  if let [IntToIntToInt(f),IntList(xs),IntList(ys)] = args {
    xs.iter().zip(ys.iter()).map(|(x,y)|f(x,y)).collect::<LambdaResult<_>>().map(|v|IntList(v))
    //Ok(IntList(xs.iter().zip(ys.iter()).map(|(x,y)|f(x,y)).collect()))
  } else { Err(TypeError) }
}
pub fn scanl1(args: &[&Val], _env: &[Val]) -> Result {
  if let [IntToIntToInt(f),IntList(xs)] = args {
    // if input is empty list, return empty list
    if xs.len() == 0 {return Ok(IntList(xs.clone()))}
    //let init = Ok(xs.first().unwrap().clone()); //ok to unwrap bc already checked for empty list
    //Ok(IntList(xs.iter().skip(1).scan(init,|x,y|Some(f(x,y))).collect()))
    let init = xs.first().unwrap().clone(); //ok to unwrap bc already checked for empty list

    // xs.iter().skip(1).try_fold(init,|acc,next| f(acc,next))
    let mut ys = vec![init];

    for x in xs.iter().skip(1) {
      ys.push(f(ys.last().unwrap(),x)?);
    }
    Ok(IntList(ys))
    // let scan_fn = |acc:LambdaResult<_>,next| {
    //   acc.and_then(|acc_|f(&acc_,&next))
    // };
    // //xs.iter().skip(1).scan(init,|x,y|Some(f(x,y))).collect()
    // xs.iter().skip(1).scan(init,scan_fn).collect::<LambdaResult<_>>().map(|v|IntList(v))
  } else { Err(TypeError) }
}

pub fn make_add1(args: &[&Val], _env: &[Val]) -> Result{
  if let [] = args {
    Ok(IntToInt(|x|x.checked_add(1).ok_or(RuntimeError)))
  } else { Err(TypeError) }
}
pub fn make_sub1(args: &[&Val], _env: &[Val]) -> Result{
  if let [] = args {
    Ok(IntToInt(|x|x.checked_sub(1).ok_or(RuntimeError)))
  } else { Err(TypeError) }
}
pub fn make_mul2(args: &[&Val], _env:&[Val]) -> Result{
  if let [] = args {
    Ok(IntToInt(|x|x.checked_mul(2).ok_or(RuntimeError)))
  } else { Err(TypeError) }
}
pub fn make_div2(args: &[&Val], _env:&[Val]) -> Result{
  if let [] = args {
    Ok(IntToInt(|x|x.checked_div(2).ok_or(RuntimeError))) // integer division
  } else { Err(TypeError) }
}
pub fn make_negate(args: &[&Val], _env:&[Val]) -> Result{
  if let [] = args {
    Ok(IntToInt(|x|x.checked_mul(-1).ok_or(RuntimeError)))
  } else { Err(TypeError) }
}
pub fn make_square(args: &[&Val], _env:&[Val]) -> Result{
  if let [] = args {
    Ok(IntToInt(|x|x.checked_mul(*x).ok_or(RuntimeError)))
  } else { Err(TypeError) }
}
pub fn make_mul3(args: &[&Val], _env:&[Val]) -> Result{
  if let [] = args {
    Ok(IntToInt(|x|x.checked_mul(3).ok_or(RuntimeError)))
  } else { Err(TypeError) }
}
pub fn make_div3(args: &[&Val], _env:&[Val]) -> Result{
  if let [] = args {
    Ok(IntToInt(|x|x.checked_div(3).ok_or(RuntimeError)))
  } else { Err(TypeError) }
}
pub fn make_mul4(args: &[&Val], _env:&[Val]) -> Result{
  if let [] = args {
    Ok(IntToInt(|x|x.checked_mul(4).ok_or(RuntimeError)))
  } else { Err(TypeError) }
}
pub fn make_div4(args: &[&Val], _env:&[Val]) -> Result{
  if let [] = args {
    Ok(IntToInt(|x|x.checked_div(4).ok_or(RuntimeError)))
  } else { Err(TypeError) }
}

pub fn make_is_gt0(args: &[&Val], _env: &[Val]) -> Result{
  if let [] = args {
    Ok(IntToBool(|x|*x>0))
  } else { Err(TypeError) }
}
pub fn make_is_lt0(args: &[&Val], _env: &[Val]) -> Result{
  if let [] = args {
    Ok(IntToBool(|x|*x<0))
  } else { Err(TypeError) }
}
pub fn make_is_even(args: &[&Val], _env: &[Val]) -> Result{
  if let [] = args {
    Ok(IntToBool(|x|x%2==0))
  } else { Err(TypeError) }
}
pub fn make_is_odd(args: &[&Val], _env: &[Val]) -> Result{
  if let [] = args {
    Ok(IntToBool(|x|x%2==1))
  } else { Err(TypeError) }
}

pub fn make_add(args: &[&Val], _env: &[Val]) -> Result{
  if let [] = args {
    Ok(IntToIntToInt(|x,y|x.checked_add(*y).ok_or(RuntimeError)))
  } else { Err(TypeError) }
}
pub fn make_sub(args: &[&Val], _env: &[Val]) -> Result{
  if let [] = args {
    Ok(IntToIntToInt(|x,y|x.checked_sub(*y).ok_or(RuntimeError)))
  } else { Err(TypeError) }
}
pub fn make_mul(args: &[&Val], _env: &[Val]) -> Result{
  if let [] = args {
    Ok(IntToIntToInt(|x,y|x.checked_mul(*y).ok_or(RuntimeError)))
  } else { Err(TypeError) }
}
pub fn make_min(args: &[&Val], _env: &[Val]) -> Result{
  if let [] = args {
    Ok(IntToIntToInt(|&x,&y|Ok(std::cmp::min(x,y))))
  } else { Err(TypeError) }
}
pub fn make_max(args: &[&Val], _env: &[Val]) -> Result{
  if let [] = args {
    Ok(IntToIntToInt(|&x,&y|Ok(std::cmp::max(x,y))))
  } else { Err(TypeError) }
}
