cargo build --release --example ex_bench
sudo dtrace -c 'target/release/examples/ex_bench' -o out.stacks -n 'profile-997 /execname == "ex_bench"/  { @[ustack(100)] = count(); }'
~/clones/FlameGraph/stackcollapse.pl out.stacks | ~/clones/FlameGraph/flamegraph.pl > flame.svg
