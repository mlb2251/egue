use super::{Val,Prod,Found,Type,DisplayExpr,Id,Error};

#[derive(Default)]
pub struct SearchState {
  // holds the current state of the search
  pub prods: Vec<Prod>, // production rules
  pub seen: std::collections::HashSet::<Val>, // set of values seen so far
  pub found_vecs: [Vec<Found>; 5],
  pub env: Vec<Val>,
  target_weight: usize,
  pub observational_equiv: bool,
  pub quiet: bool,
  pub silent: bool,
  pub target: Option<Val>,
}

impl SearchState {
  pub fn new(mut prods: Vec<Prod>, env: Vec<Val>, observational_equiv: bool, quiet: bool, silent: bool, target: Option<Val>) -> SearchState {
    for (i,prod) in prods.iter_mut().enumerate() {
      prod.id = Some(i);
    }
    SearchState {prods, env, observational_equiv, quiet, silent, target, ..Default::default()}
  }
  fn possible_values(&self, arg: Type) -> &Vec<Found> {
      match arg {
        Type::Int => &self.found_vecs[0],
        Type::IntList => &self.found_vecs[1],
        Type::IntToInt => &self.found_vecs[2],
        Type::IntToIntToInt => &self.found_vecs[3],
        Type::IntToBool => &self.found_vecs[4],
      }
  }
  pub fn expr_of_found(&self, found: &Found) -> DisplayExpr {
    let args = &found.args;
    let prod = &self.prods[found.prod];
    if args.len() == 0 {
      return DisplayExpr::Leaf(prod.name.clone());
    }
    let found_vecs: Vec<&Vec<Found>> = prod.args.iter()
      .map(|&ty|self.possible_values(ty))
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
      Err(Error::RuntimeError) => {return None} // program crashes when you try to run it
      Err(Error::TypeError) => panic!("we should never encouter type errors"),
      //Err(_) => panic!("havent handled whatever this is"),
      Ok(val) => val,
    };
    // observational equivalence
    if self.observational_equiv{
      if self.seen.contains(&val) { return None }
    }

    let arg_ids: Vec<Id> = args.iter().map(|found| found.id).map(|x|x.unwrap()).collect();

    Some(Found {prod: prod.id.unwrap(), 
            args: arg_ids,
            weight: weight,
            val: val,
            id: None })
  }

  pub fn run(&mut self, weight: usize) -> Option<Found> {
    if weight <= self.target_weight { return None }
    for _ in self.target_weight..weight {
      let res = self.step();
      if res.is_some() {return res}
    }
    None
  }


  pub fn step(&mut self) -> Option<Found>{
    self.target_weight += 1;
    if !self.silent { println!("===Weight {}===",self.target_weight) };

    let mut to_add = Vec::<Found>::new();
    let mut success = false;

    'outer: for prod in self.prods.iter() {
      let args:Vec<_> = prod.args.iter().map(|&ty|self.possible_values(ty)).collect();

      // I know this is terrifying code but let me explain
      // first of all this is just a fun quick project so its fine
      // second i wanted stuff all flat without function calls for speed since we really do know
      // that we only care about these 4 cases. If you try to get into stuff that doesnt
      // do top level branching then you start needing sentinel values and they need to type
      // check correctly so it gets annoying. Also maybe you could do this all with 3 nested
      // maps but idk.
      // TLDR: Just move the hell on and go work on something else now that this works

      // if you wanted you could write some bits as closures (tho also macros would be nice?):
      let check = |argslice: &[&Found]| {self.check_limit(prod,argslice)};
      let mut insert = |argslice: &[&Found]| -> bool {
        if let Some(v) = self.try_add(prod,argslice) {
          // check if found solution
          if let Some(target_val) = &self.target {
            if *target_val == v.val {
              to_add.push(v);
              success = true;
              return true; //means we found the answer
            }
          }
          to_add.push(v);
        };
        return false;
      };


      match args.len() {
        0 => {if insert(&[]) {break 'outer};},

        1 => {
          for arg0 in args[0].iter() {
            if !check(&[arg0]){break}
            if insert(&[arg0]) {break 'outer};
          }
        },

        2 => {
          for arg0 in args[0].iter() {
              if !check(&[arg0]) {break}
              for arg1 in args[1].iter() {
                if !check(&[arg0,arg1]) {break}
                if insert(&[arg0,arg1]){break 'outer};
              }
          }
        },
        3 => {
          for arg0 in args[0].iter() {
              if !check(&[arg0]) {break}
              for arg1 in args[1].iter() {
                if !check(&[arg0,arg1]) {break}
                for arg2 in args[2].iter() {
                  if !check(&[arg0,arg1,arg2]) {break}
                  if insert(&[arg0,arg1,arg2]){break 'outer};
                }
              }
          }
        },
        _ => panic!("didnt expect >3 args")
      }


      // let aux = |args_remaining:Vec<&Vec<Found>, args_so_far: | {
      //   match args_remaining.len() {
      //     0 =>
      //   }
      //   for arg0 in args[0].iter() {
      //       if !check(&[arg0]) {break}
      //       for arg1 in args[1].iter() {
      //         if !check(&[arg0,arg1]) {break}
      //         for arg2 in args[2].iter() {
      //           if !check(&[arg0,arg1,arg2]) {break}
      //           if insert(&[arg0,arg1,arg2]){break 'outer};
      //         }
      //       }
      //   }
      // },



    };

    let res: Option<Found> = if success {to_add.last().cloned()} else {None};

    // update with newly found values
    for mut found in to_add.into_iter(){
      if self.observational_equiv{
        // this doublecheck is necessary in case the value was added previously on
        // this same to_add run.
        if self.seen.contains(&found.val) { continue }
        self.seen.insert(found.val.clone()); // mark Found.val as seen
      }
      if !self.quiet {
        println!("{:?} -> {:?} ", self.expr_of_found(&found), found.val);
      }
      let v = match found.val.ty() {
        Type::Int => &mut self.found_vecs[0],
        Type::IntList => &mut self.found_vecs[1],
        Type::IntToInt => &mut self.found_vecs[2],
        Type::IntToIntToInt => &mut self.found_vecs[3],
        Type::IntToBool => &mut self.found_vecs[4],
      };
      found.id = Some(v.len()); // set the id of Found
      v.push(found);
    }

    
    if !self.silent { println!("*** {:?}",self) };
    res
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