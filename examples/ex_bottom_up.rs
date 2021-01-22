use egue::bottom_up;
use bottom_up::{Val,SearchState,parser::parse_io_complete};
use std::time::Instant;

use clap::{App};

/// NOTE run like "cargo run --release --example ex_bottom_up -- -h" for help info / example!

fn main() {

  let cfg = App::new("bottom up")
    .args_from_usage(
      "
      -w --weight [int]             'max weight to run bottom up till'
      --io <io example>             'an io example like [1,2,3,4],6 -> 4'
      -q --quiet                    'hush hush'
      --no-equiv                    'turns off observational equivalence checks'
      "
    )
    .about("run like cargo run --release --example ex_bottom_up -- -q -w13 --io \"[1,2,3] -> [4,5,6]\" and note that any number of inputs can be provided")
    .get_matches();

  let weight: usize = cfg.value_of("weight").unwrap_or("7").parse().expect("invalid arg type");
  let (env,target) = parse_io_complete(cfg.value_of("io").unwrap()).expect("invalid io example");
  let target = Some(target);
  

  println!("Starting up");

  // let env = vec![Val::Int(3),Val::IntList(vec![1,2,3,4,5])];

  println!("building productions");
  let prods = bottom_up::deepcoder::get_prods();

  println!("building search state");
  let observational_equiv = !cfg.is_present("no-equiv");
  let quiet = cfg.is_present("quiet");
  // let target = Some(Val::IntList(vec![48, 62, 78]));
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