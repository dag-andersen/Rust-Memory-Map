echo "SCP - start"
scp -r ./Cargo.toml $1:~/
scp -r ./src $1:~/
scp -r ./run_benchmark_cache.sh $1:~/
scp -r ./run_benchmark.sh $1:~/
echo "SCP - complete"