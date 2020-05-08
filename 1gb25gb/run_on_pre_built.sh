set -e

cargo build --release --color=always

input_data_shuffled="input_data_shuffled.txt"
hn="$(hostname)"


./target/release/rust_map -n 150000000 -p -B -i $input_data_shuffled -g 1000 > "speed/$hn/150000000_TABLE_1.txt" && printf "1"
./target/release/rust_map -n 150000000 --print_info --input_file $input_data_shuffled --search_BST --gap_size 100 > "speed/$hn/150000000_BST_2.txt" && printf "2"
./target/release/rust_map -n 150000000 --print_info --input_file $input_data_shuffled --search_BST --gap_size 100 > "speed/$hn/150000000_BST_3.txt" && printf "3"