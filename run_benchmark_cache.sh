set -e

output='testdata/out/speed/benchmark.txt'
perf_cmd="perf stat -o $output --append -e task-clock,cycles,instructions,cache-references,cache-misses"
$run_program='./target/release/rust_map'
cargo_post_cmd='-- --exact --nocapture --ignored'
input_data_shuffled="input_data_shuffled.txt"

printf "\nBUILDING RELEASE --------------------------------------------------------------------------------------------------------------------\n"
cargo build --release --color=always
sleep 2

rm -f $output
hostname >> $output

printf "\nGenerate Data --------------------------------------------------------------------------------------------------------------------------\n"
$run_program --generate_data --print_info -n $1
sleep 2

printf "\nbuild_tree --------------------------------------------------------------------------------------------------------------------------\n"
$run_program --build_BST --input_file $input_data_shuffled -n $1
sleep 2

printf "\nbuild_redblack ----------------------------------------------------------------------------------------------------------------------\n"
$run_program --build_redblack --input_file $input_data_shuffled -n $1
sleep 2

printf "\nbuild_table -------------------------------------------------------------------------------------------------------------------------\n"
$run_program --build_table --input_file $input_data_shuffled -n $1
sleep 2




printf "\nsearch_time_tree --------------------------------------------------------------------------------------------------------------------\n"
$perf_cmd $run_program --search_BST --input_file $input_data_shuffled >> $output
sleep 2

printf "\nsearch_time_redblack ----------------------------------------------------------------------------------------------------------------\n"
$perf_cmd $run_program --search_redblack --input_file $input_data_shuffled >> $output
sleep 2

printf "\nsearch_time_table -------------------------------------------------------------------------------------------------------------------\n"
$perf_cmd $run_program --search_table --input_file $input_data_shuffled >> $output
sleep 2