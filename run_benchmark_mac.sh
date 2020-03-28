output='testdata/out/speed/benchmark.txt'
cargo_pre_cmd_release='cargo test --release --color=always --package rust_map --bin rust_map'
cargo_pre_cmd='cargo test  --release --color=always --package rust_map --bin rust_map'
cargo_post_cmd='-- --exact --nocapture --ignored'

rm $output

hostname >> $output

printf "\nBUILDING RELEASE --------------------------------------------------------------------------------------------------------------------\n"
cargo build --release --color=always
sleep 2

printf "\ncreate_test_data --------------------------------------------------------------------------------------------------------------------\n"
$cargo_pre_cmd_release DO_BenchmarkTests::create_test_data $cargo_post_cmd
sleep 2

printf "\nshuffling ---------------------------------------------------------------------------------------------------------------------------\n"
gshuf DO_Benchmark_test_pre.txt > DO_Benchmark_test.txt

printf "\nbuild_table -------------------------------------------------------------------------------------------------------------------------\n"
$cargo_pre_cmd_release DO_BenchmarkTests::build_table $cargo_post_cmd >> $output
sleep 2


printf "\nbuild_tree --------------------------------------------------------------------------------------------------------------------------\n"
$cargo_pre_cmd_release DO_BenchmarkTests::build_tree $cargo_post_cmd >> $output
sleep 2

printf "\nbuild_redblack ----------------------------------------------------------------------------------------------------------------------\n"
$cargo_pre_cmd_release DO_BenchmarkTests::build_redblack $cargo_post_cmd >> $output
sleep 2

printf "\nBUILDING ----------------------------------------------------------------------------------------------------------------------------\n"
cargo build --release --color=always
sleep 2

printf "\nsearch_time_tree --------------------------------------------------------------------------------------------------------------------\n"
$cargo_pre_cmd DO_BenchmarkTests::search_time_tree $cargo_post_cmd >> $output
sleep 2

printf "\nsearch_time_redblack ----------------------------------------------------------------------------------------------------------------\n"
$cargo_pre_cmd DO_BenchmarkTests::search_time_redblack $cargo_post_cmd >> $output
sleep 2

printf "\nsearch_time_table -------------------------------------------------------------------------------------------------------------------\n"
$cargo_pre_cmd DO_BenchmarkTests::search_time_table $cargo_post_cmd >> $output
sleep 2