use egue::bottom_up;
use bottom_up::{Val,SearchState};
use std::time::Instant;


#[test]
fn deepcoder_test() {

  println!("Starting up");

  let env = vec![Val::Int(3),Val::IntList(vec![1,2,3,4,5])];

  println!("building productions");
  let prods = bottom_up::deepcoder::get_prods();

  println!("building search state");
  let observational_equiv = false;
  let quiet = true;
  let mut search_state = SearchState::new(prods, env, observational_equiv, quiet);

  println!("running");
  let tstart = Instant::now();
  search_state.run(4);
  let elapsed = tstart.elapsed().as_secs_f32();
  println!("{:.2}", elapsed);

}