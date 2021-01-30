use egue::bottom_up::{
    Val,
    SearchState,
    deepcoder,
};
use std::time::Instant;

fn main() {
  let env = vec![Val::Int(3),Val::IntList(vec![1,2,3,4,5])];
  let prods = deepcoder::get_prods(env.len());
  let observational_equiv = true;
  let quiet = true;
  let silent = false;
  let target = None;
  let mut search_state = SearchState::new(prods, env, observational_equiv, quiet, silent, target);
  let tstart = Instant::now();
  search_state.run(14);
  let elapsed = tstart.elapsed().as_secs_f32();
  println!("{:.2}", elapsed);
}