echo "SCP - copy to DO"
scp -r ./Cargo.toml root@$1:~/
scp -r ./src root@$1:~/
scp -r ./run_benchmark.sh root@$1:~/
echo "SCP - complete"