set -e

cargo build --release --color=always

printf "\n search_BST \n "

hn="$(hostname)"

./target/release/rust_map -n 1000 --print_info --generate_data --build_BST --search_BST > "speed/$hn/1000_BST_1.txt" && printf 1
./target/release/rust_map -n 1000 --print_info --generate_data --build_BST --search_BST > "speed/$hn/1000_BST_2.txt" && printf 2
./target/release/rust_map -n 1000 --print_info --generate_data --build_BST --search_BST > "speed/$hn/1000_BST_3.txt" && printf 3
./target/release/rust_map -n 1000 --print_info --generate_data --build_BST --search_BST > "speed/$hn/1000_BST_4.txt" && printf 4
./target/release/rust_map -n 1000 --print_info --generate_data --build_BST --search_BST > "speed/$hn/1000_BST_5.txt" && printf 5

printf "\n search_table \n "

./target/release/rust_map -n 1000 --print_info --generate_data --build_table --search_table > "speed/$hn/1000_table_1.txt" && printf 1
./target/release/rust_map -n 1000 --print_info --generate_data --build_table --search_table > "speed/$hn/1000_table_2.txt" && printf 2
./target/release/rust_map -n 1000 --print_info --generate_data --build_table --search_table > "speed/$hn/1000_table_3.txt" && printf 3
./target/release/rust_map -n 1000 --print_info --generate_data --build_table --search_table > "speed/$hn/1000_table_4.txt" && printf 4
./target/release/rust_map -n 1000 --print_info --generate_data --build_table --search_table > "speed/$hn/1000_table_5.txt" && printf 5

printf "\n search_redblack \n "

./target/release/rust_map -n 1000 --print_info --generate_data --build_redblack --search_redblack > "speed/$hn/1000_redblack_1.txt" && printf 1
./target/release/rust_map -n 1000 --print_info --generate_data --build_redblack --search_redblack > "speed/$hn/1000_redblack_2.txt" && printf 2
./target/release/rust_map -n 1000 --print_info --generate_data --build_redblack --search_redblack > "speed/$hn/1000_redblack_3.txt" && printf 3
./target/release/rust_map -n 1000 --print_info --generate_data --build_redblack --search_redblack > "speed/$hn/1000_redblack_4.txt" && printf 4
./target/release/rust_map -n 1000 --print_info --generate_data --build_redblack --search_redblack > "speed/$hn/1000_redblack_5.txt" && printf 5

printf "\n search_BST \n "

./target/release/rust_map -n 100000 --print_info --generate_data --build_BST --search_BST > "speed/$hn/100000_BST_1.txt" && printf 1
./target/release/rust_map -n 100000 --print_info --generate_data --build_BST --search_BST > "speed/$hn/100000_BST_2.txt" && printf 2
./target/release/rust_map -n 100000 --print_info --generate_data --build_BST --search_BST > "speed/$hn/100000_BST_3.txt" && printf 3
./target/release/rust_map -n 100000 --print_info --generate_data --build_BST --search_BST > "speed/$hn/100000_BST_4.txt" && printf 4
./target/release/rust_map -n 100000 --print_info --generate_data --build_BST --search_BST > "speed/$hn/100000_BST_5.txt" && printf 5

printf "\n search_table \n "

./target/release/rust_map -n 100000 --print_info --generate_data --build_table --search_table > "speed/$hn/100000_table_1.txt" && printf "1"
./target/release/rust_map -n 100000 --print_info --generate_data --build_table --search_table > "speed/$hn/100000_table_2.txt" && printf "2"
./target/release/rust_map -n 100000 --print_info --generate_data --build_table --search_table > "speed/$hn/100000_table_3.txt" && printf "3"
./target/release/rust_map -n 100000 --print_info --generate_data --build_table --search_table > "speed/$hn/100000_table_4.txt" && printf "4"
./target/release/rust_map -n 100000 --print_info --generate_data --build_table --search_table > "speed/$hn/100000_table_5.txt" && printf "5"

printf "\n search_redblack \n "

./target/release/rust_map -n 100000 --print_info --generate_data --build_redblack --search_redblack > "speed/$hn/100000_redblack_1.txt" && printf "1"
./target/release/rust_map -n 100000 --print_info --generate_data --build_redblack --search_redblack > "speed/$hn/100000_redblack_2.txt" && printf "2"
./target/release/rust_map -n 100000 --print_info --generate_data --build_redblack --search_redblack > "speed/$hn/100000_redblack_3.txt" && printf "3"
./target/release/rust_map -n 100000 --print_info --generate_data --build_redblack --search_redblack > "speed/$hn/100000_redblack_4.txt" && printf "4"
./target/release/rust_map -n 100000 --print_info --generate_data --build_redblack --search_redblack > "speed/$hn/100000_redblack_5.txt" && printf "5"

printf "\n search_BST \n "

./target/release/rust_map -n 10000000 --print_info --generate_data --build_BST --search_BST > "speed/$hn/10000000_BST_1.txt" && printf "1"
./target/release/rust_map -n 10000000 --print_info --generate_data --build_BST --search_BST > "speed/$hn/10000000_BST_2.txt" && printf "2"
./target/release/rust_map -n 10000000 --print_info --generate_data --build_BST --search_BST > "speed/$hn/10000000_BST_3.txt" && printf "3"
./target/release/rust_map -n 10000000 --print_info --generate_data --build_BST --search_BST > "speed/$hn/10000000_BST_4.txt" && printf "4"
./target/release/rust_map -n 10000000 --print_info --generate_data --build_BST --search_BST > "speed/$hn/10000000_BST_5.txt" && printf "5"

printf "\n search_table \n "

./target/release/rust_map -n 10000000 --print_info --generate_data --build_table --search_table > "speed/$hn/10000000_table_1.txt" && printf "1"
./target/release/rust_map -n 10000000 --print_info --generate_data --build_table --search_table > "speed/$hn/10000000_table_2.txt" && printf "2"
./target/release/rust_map -n 10000000 --print_info --generate_data --build_table --search_table > "speed/$hn/10000000_table_3.txt" && printf "3"
./target/release/rust_map -n 10000000 --print_info --generate_data --build_table --search_table > "speed/$hn/10000000_table_4.txt" && printf "4"
./target/release/rust_map -n 10000000 --print_info --generate_data --build_table --search_table > "speed/$hn/10000000_table_5.txt" && printf "5"

printf "\n search_redblack \n "

./target/release/rust_map -n 10000000 --print_info --generate_data --build_redblack --search_redblack > "speed/$hn/10000000_redblack_1.txt" && printf "1"
./target/release/rust_map -n 10000000 --print_info --generate_data --build_redblack --search_redblack > "speed/$hn/10000000_redblack_2.txt" && printf "2"
./target/release/rust_map -n 10000000 --print_info --generate_data --build_redblack --search_redblack > "speed/$hn/10000000_redblack_3.txt" && printf "3"
./target/release/rust_map -n 10000000 --print_info --generate_data --build_redblack --search_redblack > "speed/$hn/10000000_redblack_4.txt" && printf "4"
./target/release/rust_map -n 10000000 --print_info --generate_data --build_redblack --search_redblack > "speed/$hn/10000000_redblack_5.txt" && printf "5"

printf "\n search_redblack \n "

./target/release/rust_map -n 150000000 --print_info --generate_data --build_redblack --search_redblack > "speed/$hn/150000000_redblack_1.txt" && printf "1"
./target/release/rust_map -n 150000000 --print_info --generate_data --build_redblack --search_redblack > "speed/$hn/150000000_redblack_2.txt" && printf "2"
./target/release/rust_map -n 150000000 --print_info --generate_data --build_redblack --search_redblack > "speed/$hn/150000000_redblack_3.txt" && printf "3"

printf "\n search_BST \n "

./target/release/rust_map -n 150000000 --print_info --generate_data --build_BST --search_BST > "speed/$hn/150000000_BST_1.txt" && printf "1"
./target/release/rust_map -n 150000000 --print_info --generate_data --build_BST --search_BST > "speed/$hn/150000000_BST_2.txt" && printf "2"
./target/release/rust_map -n 150000000 --print_info --generate_data --build_BST --search_BST > "speed/$hn/150000000_BST_3.txt" && printf "3"

printf "\n search_table \n "

./target/release/rust_map -n 150000000 --print_info --generate_data --build_table --search_table > "speed/$hn/150000000_table_1.txt" && printf "1"
./target/release/rust_map -n 150000000 --print_info --generate_data --build_table --search_table > "speed/$hn/150000000_table_2.txt" && printf "2"
./target/release/rust_map -n 150000000 --print_info --generate_data --build_table --search_table > "speed/$hn/150000000_table_3.txt" && printf "3"