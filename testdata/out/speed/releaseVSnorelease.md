```
root@onegb:~# cargo test --package rust_map --bin rust_map BenchmarkTests::search_time_tree_vs_table -- --exact --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.03s
     Running target/debug/deps/rust_map-e224c576d19f69ac

running 1 test
test BenchmarkTests::search_time_tree_vs_table ... ## search_time_tree_vs_table
highest ip: 5399439
#18181 requests created
table: pushed 0 lines
start searching
--- table score: 23096, #18181 of requests ran
Tree: pushed 0 lines
--- tree score : 75482, #18181 of requests ran
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 21 filtered out

root@onegb:~# cargo test --release --package rust_map --bin rust_map BenchmarkTests::search_time_tree_vs_table -- --exact --nocapture
    Finished release [optimized] target(s) in 0.04s
     Running target/release/deps/rust_map-dc3b335a88728c42

running 1 test
test BenchmarkTests::search_time_tree_vs_table ... ## search_time_tree_vs_table
highest ip: 5400217
#18181 requests created
table: pushed 0 lines
start searching
--- table score: 5800, #18181 of requests ran
Tree: pushed 0 lines
--- tree score : 12593, #18181 of requests ran
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 21 filtered out
```