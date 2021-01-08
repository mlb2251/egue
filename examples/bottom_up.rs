use egue::bottom_up;
use bottom_up::{Val,SearchState};
use std::time::Instant;


fn main() {

  println!("Starting up");

  let env = vec![Val::Int(3),Val::IntList(vec![1,2,3,4,5])];

  println!("building productions");
  let prods = bottom_up::deepcoder::get_prods();

  println!("building search state");
  let observational_equiv = true;
  let quiet = false;
  let mut search_state = SearchState::new(prods, env, observational_equiv, quiet);

  println!("running");
  let tstart = Instant::now();
  search_state.run(7);
  let elapsed = tstart.elapsed().as_secs_f32();
  println!("{:.2}", elapsed);

}