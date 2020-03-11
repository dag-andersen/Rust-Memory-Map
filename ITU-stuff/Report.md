# Bachelor Report

# Intro
Speed is first priority
## Motivation

when you want to quickly search through a big data-set that cant be store in ram.
On this scale the answer easy answer could be to just buy a more powerful machine, but this is maybe not what you want. Should you choose to run a given service on a virtual machine on a cloud-provider like _digital ocean_ then running a rending a machine with many resources quickly becomes expensive. This is where this problem becomes relevant.

## problem explained in detail

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
https://stackoverflow.com/questions/33985018/cannot-borrow-x-as-mutable-because-it-is-also-borrowed-as-immutable
https://stackoverflow.com/questions/47618823/cannot-borrow-as-mutable-because-it-is-also-borrowed-as-immutable


# Data structures
The usually way of searching through persistent data is usually done by saving it in a database usually consisting of key-value entries stored in tables.

The data for this problem consist of ranges, which means that the choice of database type is not obvious, and depends on different factors. 
It depends on range-size, gap-size (between each range), data-size pr entry, how many keys there can exist in total, and number of entries - and ofc how complicated of a implementation you want. 

For this problem i have chosen to look into tree structures and full-table structures. 

## Trees

### introduction


![Tree disk usage](../docs/images/bachelor-01.png)
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
The naive implementation of this is to just create a full table for all ip-addresses holding a value for each ip. This obviously result in a massive data duplication because a value is stored repeatedly for each key in the associated range. This can easily be improved by actually storing the payload in another table and only storing the 

 holding another key that is used as an address to find the actual value.

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

### optimizations 
profilers
## Setup
## Test Results

# Evaluation

## Cache 

In theory the cache shouldn't matter if the data-set consists of an infinitely large amount of entires, because the cache would be thrashed anyways.

but on a more realistic scale (like in this project) this can become a factor when i comes to speed.

The immediate thought would be that the tree would benefit from this, since the nodes closer to the root would be read much more often than the rest of the tree, meaning that the data stored in the upper nodes can be retrieved from the cache. 

# Conclusion  

# References
https://doc.rust-lang.org/1.30.0/book/second-edition/index.html
https://doc.rust-lang.org/1.30.0/book/first-edition/iterators.html
https://doc.rust-lang.org/1.30.0/book/first-edition/the-stack-and-the-heap.html
https://rust-lang-nursery.github.io/rust-cookbook/concurrency/parallel.html
https://rust-lang-nursery.github.io/rust-cookbook/web/scraping.html
https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html

# Appendix
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
``
