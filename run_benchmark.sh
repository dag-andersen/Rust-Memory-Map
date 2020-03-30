output='testdata/out/speed/benchmark.txt'
perf_cmd="perf stat -o $output --append -e task-clock,cycles,instructions,cache-references,cache-misses"
cargo_pre_cmd_release='cargo test --release --color=always --package rust_map --bin rust_map'
cargo_pre_cmd='cargo test  --release --color=always --package rust_map --bin rust_map'
cargo_post_cmd='-- --exact --nocapture --ignored'

rm $output

hostname >> $output

sync; echo 3 > /proc/sys/vm/drop_caches
printf "\nBUILDING RELEASE --------------------------------------------------------------------------------------------------------------------\n"
cargo build --release --color=always
sleep 2

sync; echo 3 > /proc/sys/vm/drop_caches
printf "\ncreate_test_data --------------------------------------------------------------------------------------------------------------------\n"
$perf_cmd $cargo_pre_cmd_release DO_BenchmarkTests::create_test_data $cargo_post_cmd
sleep 2

printf "\nshuffling ---------------------------------------------------------------------------------------------------------------------------\n"
shuf DO_Benchmark_test_pre.txt > DO_Benchmark_test.txt

sync; echo 3 > /proc/sys/vm/drop_caches
printf "\nbuild_table -------------------------------------------------------------------------------------------------------------------------\n"
$perf_cmd $cargo_pre_cmd_release DO_BenchmarkTests::build_table $cargo_post_cmd >> $output
sleep 2

sync; echo 3 > /proc/sys/vm/drop_caches
printf "\nbuild_tree --------------------------------------------------------------------------------------------------------------------------\n"
$perf_cmd $cargo_pre_cmd_release DO_BenchmarkTests::build_tree $cargo_post_cmd >> $output
sleep 2

sync; echo 3 > /proc/sys/vm/drop_caches
printf "\nbuild_redblack ----------------------------------------------------------------------------------------------------------------------\n"
$perf_cmd $cargo_pre_cmd_release DO_BenchmarkTests::build_redblack $cargo_post_cmd >> $output
sleep 2

sync; echo 3 > /proc/sys/vm/drop_caches
printf "\nBUILDING ----------------------------------------------------------------------------------------------------------------------------\n"
cargo build --release --color=always
sleep 2

sync; echo 3 > /proc/sys/vm/drop_caches
swapoff -a && swapon -a
printf "\nsearch_time_tree --------------------------------------------------------------------------------------------------------------------\n"
$perf_cmd $cargo_pre_cmd DO_BenchmarkTests::search_time_tree $cargo_post_cmd >> $output
sleep 2

sync; echo 3 > /proc/sys/vm/drop_caches
swapoff -a && swapon -a
printf "\nsearch_time_redblack ----------------------------------------------------------------------------------------------------------------\n"
$perf_cmd $cargo_pre_cmd DO_BenchmarkTests::search_time_redblack $cargo_post_cmd >> $output
sleep 2

sync; echo 3 > /proc/sys/vm/drop_caches
swapoff -a && swapon -a
printf "\nsearch_time_table -------------------------------------------------------------------------------------------------------------------\n"
$perf_cmd $cargo_pre_cmd DO_BenchmarkTests::search_time_table $cargo_post_cmd >> $output
sleep 2