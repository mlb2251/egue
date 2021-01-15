use egue::bottom_up;
use bottom_up::{Val,SearchState};
use std::time::Instant;

use clap::{App};


fn main() {

  let cfg = App::new("bottom up")
    .args_from_usage(
      "
      -w --weight [int]             'max weight to run bottom up till'
      -q --quiet                       'hush hush'
      --no-equiv                    'turns off observational equivalence checks'
      "
    ).get_matches();

  let weight: usize = cfg.value_of("weight").unwrap_or("7").parse().expect("invalid arg type");

  println!("Starting up");

  let env = vec![Val::Int(3),Val::IntList(vec![1,2,3,4,5])];

  println!("building productions");
  let prods = bottom_up::deepcoder::get_prods();

  println!("building search state");
  let observational_equiv = !cfg.is_present("no-equiv");
  let quiet = cfg.is_present("quiet");
  let target = Some(Val::IntList(vec![48, 62, 78]));
  // (zipwith (-) (map (**2) (scanl1 (+) (scanl1 (+) $1))) $1) -> IntList([48, 62, 78]) 
  let mut search_state = SearchState::new(prods, env, observational_equiv, quiet, target.clone());

  println!("running");
  let tstart = Instant::now();
  let res = search_state.run(weight);
  let elapsed = tstart.elapsed().as_secs_f32();
  if let Some(found) = res {
    println!("Found solution for target {:?}: {:?}",target, search_state.expr_of_found(&found))
  } else if target.is_some() {
    println!("Was unable to find solution for target: {:?}", target)
  }
  println!("{:.2}", elapsed);

}