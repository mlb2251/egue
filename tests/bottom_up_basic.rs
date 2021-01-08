use egue::bottom_up;
use bottom_up::{Val,SearchState};


#[test]
fn deepcoder_test() {
  let env = vec![Val::Int(3),Val::IntList(vec![1,2,3,4,5])];
  let prods = bottom_up::deepcoder::get_prods();
  let observational_equiv = false;
  let quiet = true;
  let target = None;
  let mut search_state = SearchState::new(prods, env, observational_equiv, quiet, target);
  search_state.run(4);
  assert_eq!(search_state.prods.len(),36);
  assert_eq!(search_state.seen.len(),0);
  assert_eq!(search_state.found_vecs[0].len(),154);
  assert_eq!(search_state.found_vecs[1].len(),130);
  assert_eq!(search_state.found_vecs[2].len(),10);
}

#[test]
fn deepcoder_obs_equiv() {
  let env = vec![Val::Int(3),Val::IntList(vec![1,2,3,4,5])];
  let prods = bottom_up::deepcoder::get_prods();
  let observational_equiv = true;
  let quiet = true;
  let target = None;
  let mut search_state = SearchState::new(prods, env, observational_equiv, quiet, target);
  search_state.run(4);
  assert_eq!(search_state.prods.len(),36);
  assert_eq!(search_state.seen.len(),88);
  assert_eq!(search_state.found_vecs[0].len(),23);
  assert_eq!(search_state.found_vecs[1].len(),46);
  assert_eq!(search_state.found_vecs[2].len(),10);
}

