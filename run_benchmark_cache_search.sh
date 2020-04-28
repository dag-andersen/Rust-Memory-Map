set -e

perf_cmd="perf stat -o $2 --append -e task-clock,cycles,instructions,cache-references,cache-misses"
run_program='./target/release/rust_map'
input_data_shuffled="input_data_shuffled.txt"

rm -f $2
hostname > $2

sync; echo 3 > /proc/sys/vm/drop_caches && $perf_cmd $run_program --search_BST       --input_file $input_data_shuffled --payload_size 1 -n $1 &>/dev/null && printf _ && sleep 0.5
sync; echo 3 > /proc/sys/vm/drop_caches && $perf_cmd $run_program --search_redblack  --input_file $input_data_shuffled --payload_size 1 -n $1 &>/dev/null && printf _ && sleep 0.5
sync; echo 3 > /proc/sys/vm/drop_caches && $perf_cmd $run_program --search_table     --input_file $input_data_shuffled --payload_size 1 -n $1 &>/dev/null && printf _ && sleep 0.5