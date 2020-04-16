set -e

output='testdata/out/speed/benchmark.txt'
perf_cmd="perf stat -o $output --append -e task-clock,cycles,instructions,cache-references,cache-misses"
cargo_pre_cmd_release='cargo test --release --color=always --package rust_map --bin rust_map'
cargo_pre_cmd='cargo test --release --color=always --package rust_map --bin rust_map'
cargo_post_cmd='-- --exact --nocapture --ignored'

rm -f $output
hostname >> $output

printf "\nBUILDING RELEASE --------------------------------------------------------------------------------------------------------------------\n"
cargo build --release --color=always
sleep 2

printf "\ncreate_test_data --------------------------------------------------------------------------------------------------------------------\n"
$cargo_pre_cmd_release BenchmarkTests_Separate::create_test_data $cargo_post_cmd
sleep 2

printf "\nshuffling ---------------------------------------------------------------------------------------------------------------------------\n"
shuf DO_Benchmark_test_pre.txt > DO_Benchmark_test.txt

printf "\nbuild_tree --------------------------------------------------------------------------------------------------------------------------\n"
$cargo_pre_cmd_release BenchmarkTests_Separate::create_tree $cargo_post_cmd
sleep 2

printf "\nbuild_redblack ----------------------------------------------------------------------------------------------------------------------\n"
$cargo_pre_cmd_release BenchmarkTests_Separate::create_redblack $cargo_post_cmd
sleep 2

printf "\nbuild_table -------------------------------------------------------------------------------------------------------------------------\n"
$cargo_pre_cmd_release BenchmarkTests_Separate::create_table $cargo_post_cmd
sleep 2

printf "\nBUILDING ----------------------------------------------------------------------------------------------------------------------------\n"
cargo build --release --color=always
sleep 2

printf "\nsearch_time_tree --------------------------------------------------------------------------------------------------------------------\n"
$perf_cmd $cargo_pre_cmd BenchmarkTests_Separate::search_time_tree $cargo_post_cmd >> $output
sleep 2

printf "\nsearch_time_redblack ----------------------------------------------------------------------------------------------------------------\n"
$perf_cmd $cargo_pre_cmd BenchmarkTests_Separate::search_time_redblack $cargo_post_cmd >> $output
sleep 2

printf "\nsearch_time_table -------------------------------------------------------------------------------------------------------------------\n"
$perf_cmd $cargo_pre_cmd BenchmarkTests_Separate::search_time_table $cargo_post_cmd >> $output
sleep 2