set -e

vm="root@128.199.61.84:/mnt/vol1gbv2"

echo "SCP - start"
scp -r ./Cargo.toml $vm
scp -r ./src $vm
scp -r ./run_benchmark.sh $vm
scp -r ./run_benchmark_cache_loop.sh $vm
scp -r ./run_benchmark_cache_build.sh $vm
scp -r ./run_benchmark_cache_search.sh $vm
scp -r ./1gb25gb/run_on_pre_built.sh $vm
echo "SCP - complete"

