set -e

cargo build --release --color=always

hn="$(hostname)"

printf "\n cache 1000 \n "

./run_benchmark_cache.sh 1000 "cache/$hn/1000_1.txt" >>/dev/null && printf 1
./run_benchmark_cache.sh 1000 "cache/$hn/1000_2.txt" &>/dev/null && printf 2
./run_benchmark_cache.sh 1000 "cache/$hn/1000_3.txt" &>/dev/null && printf 3
./run_benchmark_cache.sh 1000 "cache/$hn/1000_4.txt" &>/dev/null && printf 4
./run_benchmark_cache.sh 1000 "cache/$hn/1000_5.txt" &>/dev/null && printf 5

printf "\n cache 100000 \n "

./run_benchmark_cache.sh 100000 "cache/$hn/100000_1.txt" &>/dev/null && printf 1
./run_benchmark_cache.sh 100000 "cache/$hn/100000_2.txt" &>/dev/null && printf 2
./run_benchmark_cache.sh 100000 "cache/$hn/100000_3.txt" &>/dev/null && printf 3
./run_benchmark_cache.sh 100000 "cache/$hn/100000_4.txt" &>/dev/null && printf 4
./run_benchmark_cache.sh 100000 "cache/$hn/100000_5.txt" &>/dev/null && printf 5

printf "\n cache 10000000 \n "

./run_benchmark_cache.sh 10000000 "cache/$hn/10000000_1.txt" &>/dev/null && printf "1"
./run_benchmark_cache.sh 10000000 "cache/$hn/10000000_2.txt" &>/dev/null && printf "2"
./run_benchmark_cache.sh 10000000 "cache/$hn/10000000_3.txt" &>/dev/null && printf "3"

printf "\n cache 150000000 \n "

./run_benchmark_cache.sh 150000000 "cache/$hn/150000000_1.txt" &>/dev/null && printf "1"
./run_benchmark_cache.sh 150000000 "cache/$hn/150000000_2.txt" &>/dev/null && printf "2"
./run_benchmark_cache.sh 150000000 "cache/$hn/150000000_3.txt" &>/dev/null && printf "3"

