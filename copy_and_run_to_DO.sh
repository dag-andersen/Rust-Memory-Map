output='testdata/out/speed/benchmark.txt'
bash copy_to_DO.sh $1
ssh root@$1 << EOF
  ./run_benchmark_combi.sh
EOF
scp -r root@$1:~/$output ./testdata/out/speed/


