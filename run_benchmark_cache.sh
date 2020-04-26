set -e

perf_cmd="perf stat -o $2 --append -e task-clock,cycles,instructions,cache-references,cache-misses"
run_program='./target/release/rust_map'
input_data_shuffled="input_data_shuffled.txt"

printf "\nBUILDING RELEASE --------------------------------------------------------------------------------------------------------------------\n"
cargo build --release --color=always


rm -f $2
hostname > $2

printf "\nGenerate Data --------------------------------------------------------------------------------------------------------------------------\n"
$run_program --generate_data --print_info --payload_size 1 -n $1 && sync; echo 3 > /proc/sys/vm/drop_caches && sleep 0.5 

printf "\nbuild_tree --------------------------------------------------------------------------------------------------------------------------\n"
$run_program --build_BST --input_file $input_data_shuffled --payload_size 1 -n $1 && sync; echo 3 > /proc/sys/vm/drop_caches && sleep 0.5 

printf "\nbuild_redblack ----------------------------------------------------------------------------------------------------------------------\n"
$run_program --build_redblack --input_file $input_data_shuffled --payload_size 1 -n $1 && sync; echo 3 > /proc/sys/vm/drop_caches && sleep 0.5 

printf "\nbuild_table -------------------------------------------------------------------------------------------------------------------------\n"
$run_program --build_table --input_file $input_data_shuffled --payload_size 1 -n $1 && sync; echo 3 > /proc/sys/vm/drop_caches && sleep 0.5 




printf "\nsearch_time_tree --------------------------------------------------------------------------------------------------------------------\n"
$perf_cmd $run_program --search_BST --input_file $input_data_shuffled --payload_size 1 -n $1 && sync; echo 3 > /proc/sys/vm/drop_caches && sleep 0.5 

printf "\nsearch_time_redblack ----------------------------------------------------------------------------------------------------------------\n"
$perf_cmd $run_program --search_redblack --input_file $input_data_shuffled --payload_size 1 -n $1 && sync; echo 3 > /proc/sys/vm/drop_caches && sleep 0.5 

printf "\nsearch_time_table -------------------------------------------------------------------------------------------------------------------\n"
$perf_cmd $run_program --search_table --input_file $input_data_shuffled --payload_size 1 -n $1 && sync; echo 3 > /proc/sys/vm/drop_caches && sleep 0.5 