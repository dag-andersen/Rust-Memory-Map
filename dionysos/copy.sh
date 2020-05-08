set -e

vm="daga@dionysos.itu.dk:/mnt/nvme/daga"

echo "SCP - start"
scp -r ./Cargo.toml $vm
scp -r ./src $vm
scp -r ./run_benchmark.sh $vm
scp -r ./run_benchmark_cache_loop.sh $vm
scp -r ./run_benchmark_cache_build.sh $vm
scp -r ./run_benchmark_cache_search.sh $vm
echo "SCP - complete"

