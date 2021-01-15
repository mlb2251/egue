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
  assert_eq!(search_state.found_vecs[1].len(),135);
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
  assert_eq!(search_state.seen.len(),89);
  assert_eq!(search_state.found_vecs[0].len(),23);
  assert_eq!(search_state.found_vecs[1].len(),47);
  assert_eq!(search_state.found_vecs[2].len(),10);
}

#[test]
fn deepcoder_obs_equiv_search() {
  let env = vec![Val::Int(3),Val::IntList(vec![1,2,3,4,5])];
  let prods = bottom_up::deepcoder::get_prods();
  let observational_equiv = true;
  let quiet = true;
  let target = Some(Val::IntList(vec![48, 62, 78]));
  let mut search_state = SearchState::new(prods, env, observational_equiv, quiet, target);
  let res = search_state.run(11);
  assert!(res.is_some());
  //assert!(format!("{:?}",search_state.expr_of_found(&res.unwrap())).trim() == "(zipwith (-) (map (**2) (scanl1 (+) (scanl1 (+) $1))) $1)")
}
