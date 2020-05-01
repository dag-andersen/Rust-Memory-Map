set -e

cargo build --release --color=always

hn="$(hostname)"

mkdir -p "cache/$hn"

printf "\n cache 1000 \n "

./run_benchmark_cache_build.sh 1000 && printf 1
./run_benchmark_cache_search.sh 1000 "cache/$hn/1000_1.txt" && printf 2
./run_benchmark_cache_search.sh 1000 "cache/$hn/1000_2.txt" && printf 3
./run_benchmark_cache_build.sh 1000 && printf 4
./run_benchmark_cache_search.sh 1000 "cache/$hn/1000_3.txt" && printf 5
./run_benchmark_cache_search.sh 1000 "cache/$hn/1000_4.txt" && printf 6

printf "\n cache 100000 \n "

./run_benchmark_cache_build.sh 100000 && printf 1
./run_benchmark_cache_search.sh 100000 "cache/$hn/100000_1.txt" && printf 2
./run_benchmark_cache_search.sh 100000 "cache/$hn/100000_2.txt" && printf 3
./run_benchmark_cache_build.sh 100000 && printf 4
./run_benchmark_cache_search.sh 100000 "cache/$hn/100000_3.txt" && printf 5
./run_benchmark_cache_search.sh 100000 "cache/$hn/100000_4.txt" && printf 6

printf "\n cache 10000000 \n "

./run_benchmark_cache_build.sh 10000000 && printf 1
./run_benchmark_cache_search.sh 10000000 "cache/$hn/10000000_1.txt" && printf 2
./run_benchmark_cache_search.sh 10000000 "cache/$hn/10000000_2.txt" && printf 3
./run_benchmark_cache_search.sh 10000000 "cache/$hn/10000000_3.txt" && printf 4

printf "\n cache 150000000 \n "

./run_benchmark_cache_build.sh 150000000 && printf 1
./run_benchmark_cache_search.sh 150000000 "cache/$hn/150000000_1.txt" && printf 2
./run_benchmark_cache_search.sh 150000000 "cache/$hn/150000000_2.txt" && printf 3
./run_benchmark_cache_search.sh 150000000 "cache/$hn/150000000_3.txt" && printf 4

