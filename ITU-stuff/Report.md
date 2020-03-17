# Bachelor Report

# Intro - abstract
Speed is first priority



## Motivation

when you want to quickly search through a big data-set that cant be store in ram.
On this scale the answer easy answer could be to just buy a more powerful machine, but this is maybe not what you want. Should you choose to run a given service on a virtual machine on a cloud-provider like _digital ocean_ then running a rending a machine with many resources quickly becomes expensive. This is where this problem becomes relevant.

## Problem explained in detail

Siteimprove needs service where they can look up information of a given ip-address. The primary focus is quick lookup, so their customers can access their data as quickly as possible.

Preprocessing time is not important as long as it doesn't take over day. 
Space wise it doesn't matter much either, but again it has to be a realistic/practical amount.

### Data

Each entry consist of two ip addresses and some related data/payload. The first ip determens the lowerbound of the range and the second is the uppoer bound.

I cant get access to the real data due to confindatillity, but i know the number of entries and the amount of payload pr. entry. 
The system needs to handle 150mil ipv4 ranges with a payload of 256 bytes. 
35mil ipv6 ranges. 

### Assumptions
* The input data contains no overlapping ranges
* No range/entry contain the same payload
* No ip range is excluded (No ip range should be ignore because of reserved ip-range-blocks)

### the goal
Handle 150 mil entries
Siteimprove's wishes for a lookup time of p99 in 60ms.

### Si Rules/Priorities
```
- Language:             Rust.
- Dataset:              A set of IP ranges to firms.
                        No overlapping ranges. 
- Pre-processing-time:  Not important.
                        No new entries after first lookup.
- storage space:        Not important.
- Lookup-time:          Important.
```

### Si goals 
I couldn't test on Siteimprove's real data, since it confidential, but could get 
```
- Dataset:              150.mil ipv4
                        35.mil  ipv6
                        256bytes payload pr entry.
- Memory:               4gb
- Lookup-time:          p99 in 60ms.`
```

### Input Cases
- Payload pr. range
- Distance between ranges
- number of ranges
- Range size
- IPv4 or IPv6
- mutable structure vs. dynamic insertion 

### Test Variables:
- ram 
- Space
- lookup time
- build time

## Boundaries
No need to remove or change entries.


## why rust?

### memory safety

no dagling pointers, no data races, no buffer overflows 

Guaranteed by Rust's ownership system - At compile time

An exable of usefulness of this safety can be found a 

In a survey done by XXX 51% of the security vulnerabilities in the linux kernel is coursed by concurrency and memory safety issues that are fundamentally impossible to get in rust (unless you use the `unsafe` keyword, which is not recommended)

### debugging

### ownershitp
https://medium.com/@thomascountz/ownership-in-rust-part-1-112036b1126b

### where rust falls short
```rust
pub(crate) unsafe fn bytes_to_type<T>(slice: &[u8]) -> &mut T {
    std::slice::from_raw_parts_mut(slice.as_ptr() as *mut T, std::mem::size_of::<T>())
        .get_mut(0)
        .unwrap()
}
```
Here we have no garantee of we are going to get, since it just a pointer and a length that we force to become a reference to type T.



https://medium.com/paritytech/why-rust-846fd3320d3f

https://stackoverflow.com/questions/33985018/cannot-borrow-x-as-mutable-because-it-is-also-borrowed-as-immutable
https://stackoverflow.com/questions/47618823/cannot-borrow-as-mutable-because-it-is-also-borrowed-as-immutable


# Data structures
The usually way of searching through persistent data is usually done by saving it in a database usually consisting of key-value entries stored in tables.

The data for this problem consist of ranges, which means that the choice of database type is not obvious, and depends on different factors. 
It depends on range-size, gap-size (between each range), data-size pr entry, how many keys there can exist in total, and number of entries - and ofc how complicated of a implementation you want. 

For this problem i have chosen to look into tree structures and full-table structures. 

Lets declare some variables:
```
p = payload size in bytes
e = number of entries
```

#### Fixed vs. dynamic payload length 
Depending on the problem you want to solve you can either choose to use the same amount of space for each entry or have a dynamic size meaning you only use the necessary amount of space for each entry. 
Dynamic payload is great, since you dont wast space on empty payload, but the downside is that you have to store "absolute addresses" to where the payload begins instead of only storing which number of  fixed-payload-size-block when referring to the payload.
<insert image>
This means that it sometimes are not beneficial to use dynamic, if amount of pointers to the payload accumulates to a given amount. 

For this project i have chosen dynamic payload length, because the payload consist of names, which can vary a lot in length. If fixed length was chosen i would either have to accept a large amount of wasted space, or not allow names to be over a given length meaning i would cut of names.

## Trees

### introduction

There is many differently ways of structuring each node depending on what the goal is.

* Fixed size nodes, vs. dynamic sized nodes. If you go for fixed size nodes, you only need to store the index of the node and not the full address. (as explained above)
* Wanting a separate lookup table or not. This goes hand in hand with the one above, since you cant have 

![Tree disk usage](../docs/images/bachelor-01.png)

Lets split this up into two problems: _range storage_ and _pointer/payload storage_

Depending on if you want to handle ipv6 or not you have to choose 

##### range storage

##### payloadstorage


```

ipv6 - solution E
30.000.000*232 / 8 /1000/1000/1000 = 0.87gb 

2^32/256

#table2
30.000.000 * 256/1000/1000/1000 = ~7.7gb
```


### redblack
https://www.geeksforgeeks.org/red-black-tree-set-2-insert/

to prevent the tree from being unbalanced one could implement a redblack tree.
Cons: Slower build time, more space usage

## Tables

The general understanding is that searching in tables are quicker than most data structures, because you can get the data by going directly to a specific index by using the key. 

1)
The naive implementation of this is to just create a full table for all ip-addresses holding a value for each ip. This obviously result in a massive data duplication because a value is stored repeatedly for each key in the associated range. This can easily be improved by actually storing the payload in another table and only storing the index in the ip_table

holding another key that is used as an address to find the actual value.
<insert image>

Implementation overview:
``` 
Speed: 2 lookups
Storage:
    ip_table: 2^32 * 32
    payload_table: (1 + p) · e
```
This would result in
```
(1+256) · 150000000 = 38.550.000.000 bytes = 308.400.000.000 bits
```
This means that 32 bit is not big enough to store all the  
2)



### space:
best and worst case:
```
#table1
2^32 /8    /1000/1000/1000 = 0.53 GB pr. bit 
bites/bytes/kilo/mega/giga

32 bit pointer -> 17.3 gb
64 bit pointer -> 34.5 gb
```
```
#table2
150.000.000 * 256/1000/1000/1000 = ~38gb

dynamic payload
(34.5-17.3)/(150.000.000/1000/1000/1000) = 114 bytes is the breakpointet.
```


### sql

# Design

# Testing

```
pub fn generate_source_file_with(s:&str, n: u32, range: Range<u32>, padding: Range<u32>, name_size: usize) { ... }
```

## Setup
**Generating test files**
Created a function that generate a text file where each line is 2 ips and 1 name
e.g.: `125.74.3.0 125.74.3.10 Siteimprove` 

## Build
**When running:**
The program iterate over each line reading them one by one with regex. <insert ref here>


## lookup
**choosing random lookups** 
Collecting every x'th node and shuffling them afterwards. 


### Optimizations 

A huge part of the performance came from using a profiler
The profiler used for this project was the build-in profiler-tool in _Jetbrains Clion_, which is Jetbrains low-level-programming IDE. 

<Get better images>

![treeprofiler](../docs/images/treeProfiler.png)
![tableprofiler](../docs/images/tableProfiler.png)


### Debugging
* Stepping through the debugger
* Printing tree
```
--red
-black
black
-black 
```

```

```

## Test Results

# Evaluation

## Code wise?
## design choices?

## Test Data

## Cache 

In theory the cache shouldn't matter if the data-set consists of an infinitely large amount of entires, because the cache would be thrashed anyways.

but on a more realistic scale (like in this project) this can become a factor when i comes to speed.

The immediate thought would be that the tree would benefit from this, since the nodes closer to the root would be read much more often than the rest of the tree, meaning that the data stored in the upper nodes can be retrieved from the cache. 

```
root@ubuntu-s-4vcpu-8gb-fra1-01:~# perf stat -e task-clock,cycles,instructions,cache-references,cache-misses ./target/debug/rust_map

 Performance counter stats for './target/debug/rust_map':

         31.011974      task-clock (msec)         #    0.982 CPUs utilized
          85453068      cycles                    #    2.755 GHz
          90982375      instructions              #    1.06  insn per cycle
            333436      cache-references          #   10.752 M/sec
            171468      cache-misses              #   51.425 % of all cache refs

       0.031567396 seconds time elapsed

```

`perf stat -e task-clock,cycles,instructions,cache-references,cache-misses cargo test --package rust_map --bin rust_map BenchmarkTests::speed_matrix_tree -- --exact`

Tree
```
system out something
System out something
```
Table
```
system out something
System out something
```
### Enchantments / Next Steps 

* Upgrade to redblack tree
* Actually adding a nice api, instead of only running the code through testfuctions/benchmarks.
* No reason to individually place each enty to the ip_table... i could just add them to an array in memory and then place that on disk

# Conclusion  

# References
https://doc.rust-lang.org/1.30.0/book/second-edition/index.html
https://doc.rust-lang.org/1.30.0/book/first-edition/iterators.html
https://doc.rust-lang.org/1.30.0/book/first-edition/the-stack-and-the-heap.html
https://rust-lang-nursery.github.io/rust-cookbook/concurrency/parallel.html
https://rust-lang-nursery.github.io/rust-cookbook/web/scraping.html
https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html

# Appendix


### Siteimprove Data
```
IP v4 entries for isp is 177765
IP v6 entries for isp is 29846
IP v4 entries for mcc is 19547
IP v6 entries for mcc is 563
IP v4 entries for asn is 307279
IP v6 entries for asn is 38018
IP v4 entries for org is 9661299
IP v6 entries for org is 891625
IP v4 entries for domain is 17229245
IP v6 entries for domain is 3164
IP v4 entries for pulse is 124127485
IP v6 entries for pulse is 34027618

124127485+ 17229245+ 9661299+307279+ 19547 + 177765 = 151.000.000
29846 + 563 + 38018 + 891625 + 3164 + 34027618      =  35.000.000
```
