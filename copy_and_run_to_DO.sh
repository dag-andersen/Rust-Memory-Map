perf_cmd='perf stat -o perf.txt --append -e task-clock,cycles,instructions,cache-references,cache-misses'
cargo_pre_cmd='cargo test --release --color=always --package rust_map --bin rust_map'
cargo_post_cmd='-- --exact --nocapture'
bash copy_to_DO.sh $1
ssh root@$1 << EOF
  rm perf.txt

  sync; echo 3 > /proc/sys/vm/drop_caches
  printf "\nBUILDING ----------------------------------------------------------------------------------------------------------------------------\n"
  cargo build --release --color=always
  sleep 2

  sync; echo 3 > /proc/sys/vm/drop_caches
  printf "\ncreate_test_data ----------------------------------------------------------------------------------------------------------------------------\n"
  $perf_cmd $cargo_pre_cmd DO_BenchmarkTests::create_test_data $cargo_post_cmd
  sleep 2

  sync; echo 3 > /proc/sys/vm/drop_caches
  printf "\nbuild_tree ----------------------------------------------------------------------------------------------------------------------------\n"
  $perf_cmd $cargo_pre_cmd DO_BenchmarkTests::build_tree $cargo_post_cmd
  sleep 2

  sync; echo 3 > /proc/sys/vm/drop_caches
  printf "\nbuild_table ----------------------------------------------------------------------------------------------------------------------------\n"
  $perf_cmd $cargo_pre_cmd DO_BenchmarkTests::build_table $cargo_post_cmd
  sleep 2

  sync; echo 3 > /proc/sys/vm/drop_caches
  printf "\nsearch_time_table ----------------------------------------------------------------------------------------------------------------------------\n"
  $perf_cmd $cargo_pre_cmd DO_BenchmarkTests::search_time_table $cargo_post_cmd
  sleep 2

  sync; echo 3 > /proc/sys/vm/drop_caches
  printf "\nsearch_time_tree ----------------------------------------------------------------------------------------------------------------------------\n"
  $perf_cmd $cargo_pre_cmd DO_BenchmarkTests::search_time_tree $cargo_post_cmd
  sleep 2
EOF
scp -r root@$1:~/perf.txt ./


