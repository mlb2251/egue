use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use egue::bottom_up::{
    Val,
    SearchState,
    deepcoder,
};



// the name of this fn doesnt matter btw
pub fn bench(c: &mut Criterion) {
  let mut group = c.benchmark_group("bottom up benches");
  group.sample_size(100);
//   group.measurement_time(Duration::from_sec(150))();
  group.bench_function("bottom_up(4)", |b| b.iter_batched(
    || { // setup
        let env = vec![Val::Int(3),Val::IntList(vec![1,2,3,4,5])];
        let prods = deepcoder::get_prods(env.len());
        let observational_equiv = true;
        let quiet = true;
        let silent = true;
        let target = None;
        SearchState::new(prods, env, observational_equiv, quiet, silent, target)
    },
    // actual timed bit
    |mut search_state| search_state.run(black_box(10)),
    BatchSize::SmallInput,
  ));
  group.finish();

}

criterion_group!(benches, bench);
criterion_main!(benches);