set -e

run_program='./target/release/rust_map'
input_data_shuffled="input_data_shuffled.txt"

cargo build --release --color=always

$run_program --generate_data    --print_info                      --payload_size 1 -n $1 &>/dev/null && printf - && sleep 0.5
$run_program --build_BST        --input_file $input_data_shuffled --payload_size 1 -n $1 &>/dev/null && printf - && sleep 0.5
$run_program --build_redblack   --input_file $input_data_shuffled --payload_size 1 -n $1 &>/dev/null && printf - && sleep 0.5
$run_program --build_table      --input_file $input_data_shuffled --payload_size 1 -n $1 &>/dev/null && printf - && sleep 0.5
