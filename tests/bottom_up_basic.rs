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
  assert_eq!(search_state.seen.len(),90);
  assert_eq!(search_state.found_vecs[0].len(),24);
  assert_eq!(search_state.found_vecs[1].len(),47);
  assert_eq!(search_state.found_vecs[2].len(),10);
}

#[test]
fn deepcoder_obs_equiv_search() {
  let env = vec![Val::Int(3),Val::IntList(vec![1,2,3,4,5])];
  let prods = bottom_up::deepcoder::get_prods();
  let observational_equiv = true;
  let quiet = true;
  //use example: (scanl1 (+) (reverse (scanl1 (*) $1))) -> IntList([120, 144, 150, 152, 153])
  // yes i checked manually and that's the right answer
  let target = Some(Val::IntList(vec![120, 144, 150, 152, 153]));
  let mut search_state = SearchState::new(prods, env, observational_equiv, quiet, target);
  let res = search_state.run(6);
  assert!(res.is_some());
  assert!(format!("{:?}",search_state.expr_of_found(&res.unwrap())).trim() == "(scanl1 (+) (reverse (scanl1 (*) $1)))")
}
