# Bachelor Report

# Intro - abstract
Speed is first priority

## Motivation

What do you do when you want to quickly search through a big data-set that cant be store in ram?
On a small scale the easy answer is to just buy a more powerful machine, but this is maybe not always what you want. Should you choose to run a given service on a virtual machine on a cloud-provider like _digital ocean_ then - then rending a machine with many resources quickly becomes expensive. This is where this problem becomes relevant.

## Problem explained in detail

Siteimprove needs a service that can look up information of a given ip-address. The primary focus is fast lookup so their customers can get a result as fast as possible.
Pre-processing time is not important as long as it doesn't take over day. 
It has to use under 
Space wise it doesn't matter much either, but again it has to be a realistic/practical amount.

### Data

The data is expected to be read from a file or read as a stream.
Each entry consist of two ip addresses and some related data/payload. The first ip determents the lower bound of the range and the second is the upper bound.
The payload can vary in size, but bla bla bla.

`
It is not possible to access to the real data due to confidentiality, but the average payload size pr. entry. is available. 
The system needs to handle 150 mil ipv4 ranges and 35 mil ipv6 ranges with a payload of 256 bytes.`

### Assumptions
* The input data contains no overlapping ranges
* No ip range is excluded (No ip range should be ignore because of reserved ip-range-blocks) / in other words... all ip addresses are possible.
* No need to remove or change entries after insertion. 
* The entries should should be able to be stream into the program so no way of knowing how many entries will actually go into the system

### the goal
Handle 150 mil entries of Ipv4
Siteimprove's wishes for a lookup time of p99 in 60ms.

The focus of this paper is the 150 mil entry ipv4 - but we will make references towards ipv6.

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

## Why use rust?

"Rust is a multi-paradigm system programming language focused on safety, especially safe concurrency. Rust is syntactically similar to C, but is designed to provide better memory safety while maintaining high performance." - wiki

In a survey done by XXX 51% of the security vulnerabilities in the linux kernel is coursed by concurrency and memory safety issues that are fundamentally impossible to get in rust (unless you use the `unsafe` keyword, which is not recommended)

### Safety

**Memory safety**

On of the  main reasons of using rust is its safety. In general Rust doesn't allow null pointers, dangling pointers, or data races.
This is done by a combination of the concept of ownership (which is basically a restriction of only having one mutable reference) and lifetime (which is a way to eliminate dangling pointers). All this are fixed are enforced at compile time.

```
Each value in Rust has a variable that’s called its owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.
```

This also eliminates C's issue of double free error. 

Rust has a concept of lifetimes. This means that if we have an array of items `[T]` and we create a reference to one of those items `&T` then the that reference needs to leave scope before the array itself. In other words, the array needs to have a longer lifetime than outside pointers to its elements - otherwise the rust compiler wont compile because it can't guarantee that the array isn't de-allocated or changed before accessing `T`. This is both a huge challenge when first starting to work with Rust, but also a really great safety.

//hypotese
This concept usually works great, but it has its challenges when using a memory map, because it can guarantee that the nodes/structs that the pointer points to are still in memory, because the page it is stored on is maybe offloaded, by the kernel/memory map. 
Starting this project is was the plan to let nodes refer to each other by using a `&T` when building a tree. But because of these compiler challenges mentioned above, I chose to instead go for an implementation where each node stored an byte-offset to where its children were stored the memory map. 

**Reading from Memory Map**
Sadly sometimes we can cant use rust's safety, and this is where rust looks more like C.
```rust
pub(crate) unsafe fn bytes_to_type<T>(slice: &[u8]) -> &mut T {
    std::slice::from_raw_parts_mut(slice.as_ptr() as *mut T, std::mem::size_of::<T>())
        .get_mut(0)
        .unwrap()
}
```
This function returns a reference to a mutable object given a reference to a `u8` array. This function is used to get a reference to a node directly on on the memory map. Here we have no guarantee of we are going to get, since it just a pointer and a length that we force to become a reference to type T. I this case we don't have any other way, since Memory Map only know the concept of bytes. 

**Error handling**
C doesn't provide good error handling, because the programmer is expected to prevent errors from occurring in the first place. This means C is much harder and unsafe to code combined with it is very difficult to debug. 

High level languages like java and C# have use mechanisms such as exceptions. Rust doesn’t have exceptions. Instead, rust has two types of error handling `Result<T, E>` and `Option<T>`. Option can be seen as the same as a result but without an error object. 
```rust
pub(crate) fn get_u32_for_ip(v: &str) -> Option<u32> {
    let v: Vec<&str> = v.split('.').collect();
    let len = v.len();
    if len != 4 { return None }
    let mut acc: u32 = 0;
    for i in 0..len {
        match v[i].parse::<u8>() {
            Ok(n) => acc |= (n as u32) << ((len-1-i) * 8) as u32,
            Err(e) => return None
        };
    }
    Some(acc)
}
```
Both concepts are used in this function. Option is used in form of `Some` and `None` and Result is used in `Ok(n)` and `Err(E)`. This function takes a string of 4 numbers separated by a dot `.` - e.g. `192.2.103.11` - and returns unsigned integer wrapped in a option. In this case i use Option as a safe way to use a null-pointer. Being able to handle error with ease is crucial when needing to deliver save code quickly. 


https://en.wikibooks.org/wiki/C_Programming/Error_handling

### Rust combined with C
Rust does not have an official interface/abstraction for using memory maps, but there exists a few open source libraries created by the community. 
Rust's package management system is called cargo and use the crates as as the packages. This uses a crate called `memmap` (version `0.7.0`). This library was chosen based on the fact that it had the most stars on Github. The abstraction provided by the external libraries are not extensive compared to the using the native C, meaning that the setting for the map is not as customizable. 

Rust have the ability to call directly into C files, and you also have the ability to use most of the c standard library inline by using the `libc`- library/crate. This means we can access functions like `mlock` and `mlockall`. `show example`. 
But rusts memory safety can not guarantee the result of these function so it forces us we need to use the "unsafe" keyword. Overall this means that we can use both rust functions and c functions as we please, but we cant guarantee what is going to happen. 

https://doc.rust-lang.org/nomicon/ffi.html

https://medium.com/@thomascountz/ownership-in-rust-part-1-112036b1126b

https://medium.com/paritytech/why-rust-846fd3320d3f

https://stackoverflow.com/questions/33985018/cannot-borrow-x-as-mutable-because-it-is-also-borrowed-as-immutable
https://stackoverflow.com/questions/47618823/cannot-borrow-as-mutable-because-it-is-also-borrowed-as-immutable


# Data structures
There is many ways of searching through key-value pairs
The data for this problem consist of ranges, which means that the choice of database type is not obvious, and depends on different factors. It depends on range-size, gap-size (between each range), payload-size pr. entry, how many keys there can exist in total, and number of entries - and of course how complicated of a implementation you want. 

This project focuses on table and tree structures.

For this section we will refer use these variables:
```
p = payload size in bytes
e = number of entries
```

#### Fixed vs. dynamic data length 

Depending on the problem you want to solve you can either choose to use the same fixed amount of space for each entry or have a dynamic size - meaning you only use the necessary amount of space for each entry. 

This choice is important for deciding how to store the payload and how we store the nodes in the tree. 

Fixed sized data could imply using a struct - meaning that the whole file is cut in equal sized pieces (structs). This means you can refer to the offset of the struct itself, and not to the byte index of the struct. This is important because byte index number will be much larger than the struct index, meaning it takes more space to store pointers to byte indexes.

<img src="../docs/images/bachelor-05.png" alt="drawing" width="600"/>

<E.g. using a u32 to as a pointer to byte-index result in only being able to refer to max size data size of `2^32 · 8 bytes = 43.4 · 10^9 bytes = 4,3gb`.>
Struct indexes is great if you know the data-object always will have the same size, but if the amount of data needed to be stored vary a lot, then we will wast space on internal padding in the structs, because they are not filled out. This means we instead can make all data-objects have a dynamic size. This would result in us having to store the size of the data-object in the header (because we don't know the size of it) and need to use byte-index to refer to the data. 


```
On the other hand dynamic data size means that --- Dynamic payload means that you for each entry great, since you don't waste space on padding/empty payload, but the downside is that you have to store the size of each block and in the block itself and you have to store the address the addresses payload begins instead of only storing the index to the node/struct of payload you are referring to. --- This is important because this means that the byte index always will be a bigger number than the struct offset. Therefore it is not alway beneficial to use dynamic sized payload if the amount of pointers are huge, since the amount of space needed accumulates. --- This means that an address-pointer of 32b can only point to a max size of ~4.3 byte data ----- For this project i have chosen dynamic payload length, because the payload consist of names, which can vary a lot in length. If fixed length was chosen i would either have to accept a large amount of wasted space, or not allow names to be over a given length meaning i would cut of names.
```

## Binary Trees

### Binary Search Tree (BST)
BST is a type of binary tree in which left child of a node has value less than the parent and right child has value greater than the parent.

One of the choices you have to make is to decide on if you want to store the payload next to the node itself or the node should store a pointer to payload somewhere else. 

Pros for storing the payload on the node:
- No need to spend time on looking up the payload in a different file.
- the payload is probably already cached, because it right next to the node it just accessed.

Pros for storing it a separate file:
- If the payload is a dynamic size, then the node will not have a fixed size, meaning all nodes in the whole tree would have to store bigger pointers, resulting in extra space needed for each node - as explained above.
- In terms of caching it would be more beneficial to store the payload on a different file, because it would mean that the nodes would be closer to each other - meaning they therefore make better use of locality while searching down the tree.

Another interesting point is to decide on how you want to store the ip-addresses. The simplest solution is to store the lower bound ip and the upper bound ip - each take up 32 bit - Resulting in 64 bit pr. node.
Another approach could be to only store the lower-bound and then store the delta to the upper-bound - this is useful if you know that the ranges will me small meaning you could get away with storing it on less bytes than 4 (32 bit). This is only useful optimizations if you know how the ranges and gaps are distributed, but since we cant do that in this project we have just went with the simple solution and storing the full ip address for both upper and lower bound. 

### Redblack Tree

An extension of the Binary Search Tree is the redblack tree. A redblack tree is a self-balancing tree structure. This prevents the tree from being imbalanced in exchange of longer build time and bigger nodes. It was invented in 1972 by Rudolf Bayer.

On important point to make is that it is not always beneficial to use a balanced tree. As Donald Knuth proves in *The art of computer programming, Volume 3, Sorting and searching, second edition, page 430* the search time for balanced tree are not insanely better han non-balanced tree on random insertion data. A unbalanced tree has a worse case search time of `O(n)`, but this is very rare and most trees are well balanced. A redblack tree has a `~Log(n)` and a BST has a `~2·log(n)` search time. Which men both data-structures has a time complexity of `O(log(n))`. 

In 1999, Chris Okasaki showed that insertion in a redblack tree only needs to handle four cases and a because, which makes it easy to implement for project like this. 

<ref Okasaki, Chris (1999-01-01). "Red-black trees in a functional setting". Journal of Functional Programming. 9 (4): 471–477. doi:10.1017/S0956796899003494. ISSN 1469-7653. Archived from the original (PS) on 2007-09-26. Retrieved 2007-05-13.>
<De 4 cases (5) bliver gennemgået her www.geeksforgeeks.org/red-black-tree-set-2-insert/ >

## Tables
```
delete?

Key-value store is a data storage paradigm

A key-value store/key-value database is a simple database that uses an associative array (also known as a dictionary or Map) as the fundamental data model. Each key is associated with only one value. This relationship is referred to as a key-value pair.
https://en.wikipedia.org/wiki/Key-value_database
https://www.aerospike.com/what-is-a-key-value-store/

A table (or dictionary) is 
The general the understanding is that searching in tables are quicker than most data structures, because you can get the data by value directly to a specific index by using the key. 


If you want fast lookup speed tables/dictionaries are a great place to start. Tables can be implemented in many different ways, but the main point is that you can get value associated with a specific key by only doing one lookup. Tables 
```
A simple implementation of table is to just create a full table for all ip-addresses holding a value for each ip. This obviously result in a massive data duplication because a value is stored repeatedly for each key in the associated range. This can easily be improved by actually storing the value in another table and only storing a pointer to it. Now the value is only stored once, but instead the pointer to it is duplicated for each key. 

<img src="../docs/images/bachelor-04.png" alt="drawing" height="250"/>

One of the downside to this is the full ip range is stored in the database even though you may only have very few entries. A solution is generally to create some kind of hashtable, where keys are hashed and points to some other data-structure (like a linked list), but this is beyond the scope of this project. 

# Implementation Design
I this project I have went for implementing a Binary Search Tree, a redblack tree, and a table. All tree implementations have their own module in the source code and has the same interface, so they can be swapped interchangeable. All data-structures are implemented using memory mapped files. All three implementations use a separate memory mapped file for storing the payload/data. This memory mapped file will be refereed to as `payload_table`.
In this implementation i have chosen to store strings as payload, but this could be swapped out with anything other datatype.

Lets declare some variables:
```
p = payload size in bytes
e = number of entries
```

## Payload Map
This memory mapped file contains all entries' value and the the length of the values in bytes. A value is retrieved from the table by giving it the byte index of the header of the value. Each lookup runs in constant time and therefore has a time complexity of *O(1)*. 

Each value has a header of one byte, which is used to store the length of the data. The length is necessary, because we don't know how far we need to read to get the value.
The length of the payload is stored on 1 byte, which means that the payload can be at most be `2^8 = 256` bytes long. This is just a design choice, but could easily be extended by changing all headers would need to be 2 bytes long instead. 

On this picture we can see how `SKAT` would be stored.

<img src="../docs/images/bachelor-03.png" alt="drawing" width="600"/>

**Space**
The space needed for this file can be estimated from the average payload size and the number of entries: `(avg(p) + 1) · e`. The `+1` is the header-size of one byte.
If we have 150.000.000 entries with 255 bytes each, we can calculate the largest possible file to be 38.4 gb

```
what is this - delete?
dynamic payload
(34.5-17.3)/(150.000.000/1000/1000/1000) = 114 bytes is the breakpointet.
```

## BST & Redblack Tree

Both the BST and the redblack tree is implemented very similar. 

Most functions are exactly the same, but with the exception of the insert function (`fn insert_node()` in the source code) in the redblack tree being pretty extensive and the fact that the redblack has functions for changing the root-node. 

The nodes in the two tree are declared as followed:
<table><tr><th>
BST
</th><th>
Redblack
</th></tr><tr><td><pre>
pub struct Node {
    pub min_ip: u32,
    pub max_ip: u32,
    pub left: usize,
    pub right: usize,
    pub name: usize,
}</pre></td><td><pre>
pub struct Node {
    pub red: bool,
    pub min_ip: u32,
    pub max_ip: u32,
    pub left: usize,
    pub right: usize,
    pub parent: usize,
    pub name: usize,
}</pre></td></tr></table>

`min_ip` being the lower-bound, `max_ip` being the higher-bound, `left` being the left child, `right` being the right child, `parent` being the parent node and `name` being a pointer to the `payload_table`, and `red` being the indicator of the node being red og black.

**Insertion**
Each time a entry is added to the tree the a new node will be appended at the end. Because all nodes have the same size, we can point to their index instead of their absolute address. The only difference from the two trees is we store the root-node in the first struct in the redblack tree.

<img src="../docs/images/bachelor-06.png" alt="drawing" width="600"/>

Here we have a simple example of what it would look like if these entries were inserted. 
```
0.0.2.22 0.0.2.55 SKAT
0.0.0.4 0.0.1.20 PWC
0.0.0.0 0.0.0.2 Siteimprove
```
![](../docs/images/bachelor-08.png)
Here we notice that the BST is not balanced and has Node 0 as root and Redblack is balanced and has Node 2 as root. We can also see that 0 valued reference to another node (left,right,parent) is treated as a null-pointer.

**Space**
For each field in the struct ordered by declaration order:
* Add the size of the field.
* Round up the current size to the nearest multiple of the next field's alignment.

Finally, round the size of the struct to the nearest multiple of its alignment.
Following this algorithm the BST nodes have a size of XX bytes while the redblack nodes have a of 48 bytes. Multiplying this with the 150mil entries, give a total file size of X.X gb for BST and 7.2 gb for redblack tree.
A small space-optimization in the redblack tree be to let the boolean be stored as the most significant bit of the name-pointer, reducing the size to to only be 44bytes (assuming we would never get a total payload of `2^64 = 1.8 · 10^19 bytes`). 

https://doc.rust-lang.org/std/mem/fn.size_of.html
https://www.geeksforgeeks.org/is-sizeof-for-a-struct-equal-to-the-sum-of-sizeof-of-each-member/ 

**Implementation overview:**
``` 
BST: 
lookup speed: O(Log(n))
Insert: O(Log(n))
Space: X bytes · n 

redblack:
lookup speed: O(Log(n))
Insert: O(Log(n))
Space: 48 bytes · n 
```

**Handling IpV6**
Tree structures handles IpV6 well. The only change necessary would be to change the `min_ip` and `max_ip` to from `u32` to `u128`. This would increase all nodes' size by `2 · (128 - 32) = 192 bits = 24 bytes`.

## Table

This implementation is based on the simple implementation mentioned in section *Tables*. This file consist of ~4,3mil unsigned integers, `u32`, that functions as a key to lookup the value in the `payload_map`.

An illustration of the data-structure can be seen below:

<img src="../docs/images/bachelor-02.png" alt="drawing" width="600"/>

To symbolize a null-pointer (meaning the ip, does not have any value) we just store 0. This means we need to add 1 to all pointers do differentiate  between null-pointers and real pointers that refer to the first value in payload_map at index 0. This is why we e.g. see ip 200 with value 6 point to byte index 5. 

```
what is this? delete?

This data-structure is the simplest implementation wise of all tree. Overall each lookup goes through these steps:
* In ip_table, get the byte index, `x`, where the value it stored.
* In payload_map, read value, `y`, by reading the the `u8` at x.
* In payload_map, starting from the x+1 read y amount of bytes and return the value.

```

**Space**
The space needed for this table is `(2^32)*64/8/1000/1000/1000 = 34.4 gb`

**Implementation overview:**
``` 
Lookup speed: constant time. 1 lookups.O(1). 
Insertion: constant time. 1 lookups. O(1). 
space: ip_table: 2^32 * 64 = 34.4 gb
```

**Handling IpV6**
In practice this implementation won't work with IpV6.
IpV6 is 128 bit instead of IpV4's 32 bit.
The amount of possible ips is `2^128 = 3,40e38`, and if all have to store a `u32` it result in a file a `3,40e38*32/8/1000/1000 = 1.3e30 gb` file.

# Testing

In the previous sections we have explained the three data-structures, table, BST, and redblack tree. To compare these data-structures the project went through various testing. The tests can be categorized in unit test and benchmarking tests.
* **Unit tests:** Most files and functions are tested using unit tests. All unit test can be found in source-code in the same file as the function they are testing.
* **The benchmarking tests** has been run on a 2 gb ram droplet, a 8 gb ram droplet, and on a 16gb ram macbook pro 2016. Detailed specs can found in appendix X. The automated benchmark test script can be found in appendix X, but the overall structure is explained in this section. 

### Benchmarking
All Benchmark tests go through a setup, build, and lookup phase. Since all three data-structures has the same interface, they can all be tested by using exactly they same functions. 

**Setup - Generating test files**
This first phase generates lines of 2 IP addresses and 1 text string (e.g.: `125.74.3.0 125.74.3.10 Siteimprove`) and writes them to a file. All lines are shuffled by using the unix command `shuf`. 
> Note: First I tried shuffling all entries in memory inside rust using a `Vec<&str>` and writing them to a file afterwards, but this was slower and was more memory intensive, than using the unix command. Both methods require that the all entries can be stored in ram at the same time. This means that the shuffling cant happen on a machine with low resources like the 1 gb memory droplet. <Algortimer eksisterer hvor man kan shuffle uden at bruge ram>

**Build data structure**
The program iterate over each line reading them one by one with regex. Both the tested data-structure and payload_table needs to be build at the same time, meaning each entry is sat into both at the same time, because the data-structure needs to know the byte-offset of the currently inserting entry. This step is deterministic and will alway provide the same output for the same input file. This phase it the most expensive.
> Note: All three data-structures produce the same `payload_table`, so they could in theory share the same payload-file, but implementation wise the all have their own to decouple the data-structures. 

**Lookup**
Testing lookup speed is done by creating some ip-requests and running them on the data-structure. The random requests are collected by iterating over the shuffled list of entries and picking every n'th entry. For each entry a random ip is picked between the upper and lower bound. All the chosen entries are then shuffled again. The actual searching is done by looping over the chosen random ips, and sequentially searching through the data structure and checks that it returns the correct payload. When finished it will print the time it took to do all the lookups. This number is then used to calculated the average lookup time. 

### Cache tests
Special cache miss-tests are performed to track how the cache may impact the performance of data structure. For testing the cache this I used linux command `perf stat -e task-clock,cycles,instructions,cache-references,cache-misses [input]` on the droplets. Between each step they cache is cleared by using the command `sync; echo 3 > /proc/sys/vm/drop_caches`, to make sure we start from an cold cache and each test is not affected by the previous. 

### Test data
The full ip-range is 2^32 = ~4.3mil and the given number of ranges are 150.000.000. This means that there is on average a new range every 28th ip. Since there is no information on how these are distributed, these test we will assume they are relatively evenly distributed over the full range of range of IPv4. Evenly distributing the ranges would suggest the worst case scenario for the table, because the entries would be spread over more pages, than if all entries were small and next to each other.  

Each range's size is a random number between 10-18, and the padding/gap between each range is also a random number between 10-18. The random aspect is added to make it more realistic, instead of having equal size ranges with equal gap between them.
<For testing purposes the payload is always 2 chars. This is mainly duo to generating random strings being a very expensive procedure.>

### Profiling 

A huge part of the performance optimization came from the build-in profiler-tool in _Jetbrain's Clion_ (Jetbrain's low-level-programming IDE). In particular its _Flame Chart_ and _Call Tree_ were very helpful. This was mainly used for seeing how much time the process spend in each scope/stackframe/function to find bottlenecks. The profiler use _sampling_.
"A sampling profiler probes the target program's call stack at regular intervals using operating system interrupts. Sampling profiles are typically less numerically accurate and specific, but allow the target program to run at near full speed."-wiki

https://www.jetbrains.com/help/clion/cpu-profiler.html
https://en.wikipedia.org/wiki/Profiling_(computer_programming)

This was most useful in the beginning both for learning rust and for detecting bottleneck early on. E.g in the a earlier version of the project a new Regex object were initialize every time it read a line for standard input. In the profiler it was a obvious bottleneck - and was therefore changed to  only getting initialized once and just parse a pointer to it round in the system. This is a simple thing but has a huge impact on the performance. On the left image we see how 56% of the time was spent initializing a Regex object, but it only took 14.7% after the change.

<img src="../docs/images/profiler/CallTree1000pre.png" alt="drawing" width="49%"/> <img src="../docs/images/profiler/CallTree1000post.png" alt="drawing" width="49%"/>

The height of the trees are also visible in the profiler though the flame graph. The image below is from a profile run with a on a function that builds both Redblack, table, and BST with 100.000 entries and it with a frequency of 5000 samples pr second.
Since it ran with 100.000 entries we can expect at minimum height of the tree to be `log(100.000)=16`. In the flame graph we can count how many stackframes deep a the redblack's `insert_node` functions goes. The last `insert_leaf_on_node`stackframe is 18 layers deep, which means that the hieght of the tree is 19 (+1 because the inserted leaf also counts). This matches are expectation of a balanced tree.
Furthermore we can see that the BST height is almost double the height of the redblack tree. This also matches are expectations of a unbalanced tree height to have a hight of `2·log(n)`, proved by Knuth mentioned in a previous section.

<måske brug ord som "call stack" https://en.wikipedia.org/wiki/Tail_call>

<img src="../docs/images/profiler/100 5 arrows.png" alt="drawing" />

>Note: the BST may even be taller/deeper, since the profiler tasks samples on a given interval, so if a stackframe is added and removed to the call stack in the middle of two samples it would not be displayed. 

Another intersting finding was that rust only optimized to tail-end recursion when running it in release mode (running it with the `--release`-flag).Below we can see that there only exist one `insert_leaf_on_node`-stackframe at the time, meaning that the optimizer created tail-end recursion. 

<img src="../docs/images/profiler/release.png" alt="drawing" width="45%"/> <img src="../docs/images/profiler/nonrelease.png" alt="drawing" width="45%"/>


### Debugging
Debugging the system was mostly done with printlines and by stepping through the code with a debugger. It can be pretty difficult to visualize how exactly each byte is placed in memory maps. The method i used to see it was to print the memory map in bytes. I used this statement `println!("{:?}", &name_table[0..100]);`, which prints out each byte in the range of 0 to 100 of the memory mapped file: `[0,0,0,255,90,0 ... ]`. This way i can print the map before and after each operation and compare them, and check if it works as intended. 

**Verifying tree structure**
It has also been used to check if tree was structured correctly. Both the BST and redblack tree can be printed to standard-out or a file, and a test to verify that the tree is printed correctly. 

Underneath we see the printout to the left and the abstraction to the right: 

<table><tr><td><pre>
------X Huawei
---O Samsung
------X Google
O Siteimprove
------X PwC
---O SKAT
------X Apple

</pre>
</td><td><pre>

<img src="../docs/images/bachelor-07.png" alt="drawing" width="400"/>
</table>

Where `O` is a black node and `X` is a red node. The testdata for generating this can be found in appendix X.

---


# Experiments

## Machines:
Mac 2016
1gb ram 25gb disk - with 100 gb ekstra volume
220 gb ram 

## Search time Experiment
#### Expectation
I would expect table to be the quickest, followed by the redblack tree, followed by the BST.

From a pure theoretical stand point we would assume that the table is the fastes, followed by the redblack tree, followed by the BST.

The table should run in constant time, because it only needs to do 1 lookup.
The both trees should have a O(log(n)) search time, but the BST would be expected to be slower than the redblack tree, because it is not blanched therefore need more key comparers for deeper nodes. 

Even though the table has very few accesses, I would expect it to be relatively worse if the ranges are very spread out, since the entries would be placed further from each other in the file and therefore the table can load mutable ranges in same file.
The trees on the other hand would perform well better on smaller datasizes, because all nodes are stored right next to eachother on the file and will therefore be load on the same page and will be cached.

#### Results
All tests are ran with: `range: 10..18, payload_size: 50, gap: 10`.
15000000 requets, in micro seconds

| Dionysos      | 1k      | 100k      | 10 mil        | 150 mil           |
| ------------- |--------:|----------:|--------------:|------------------:|
| BST           | 2       | 0.7       | x             | 3.515             |
| Redblack      | 2       | 0.9       | x             | 3,462             |
| Table         | 6       | 3.6       | x             | 1.001             |


All tests are ran with: `range: 10..18, payload_size: 50, gap: 10`.
| 1gb25gb       | 1k        | 100k          | 150 mil*           |
| ------------- |----------:|--------------:|------------------:|
| BST           | 25        | 2.5           | 2724              |
| Redblack      | 18        | 4.5           | 8700              |
| Table         | 28        | 1147          | 2730              |

*This was run from a cold cache, because it had to be build on dionysos instead, because the dataset was so big, that the 1gb machine couldnt complete in time.

#### Discussion

* Starting with dionysos, we can see that the table is the fastes on hugedataset, which was what we expected.
* On thing that is intersting is that both tree actually follow along pretty well. We  These small differences can be cause by random change of few datapoints.



Table is faster as expected, because it is only one access.
The tree is actually faster than the redblack tree - this can be explained by it may make better use of locality. When searching down the BST, you search from in the one direction down the file, and the nodes are often close to each other in the file. 
The redblack tree may have a fewer amount of checks, but they are more randomly distributed on in the file, because of rebalancing. 


The smaller the dataset is the smaller more the usikkhered. Because when we do 1000 entries and 10% lookup, then only 100 requests are actually run which is a much lower sample size than 1500000, which we do with. We need to remember this, since the variation ind the raw testdata is more spread out. 


#### Page swapping

Here we can observe that the tree is quicker than the table. This again sounds weird considering the table should run in constant time and the trees should run in log(n).

When doing 150000 search through (every 1000 entry) searching in the table and tree, we see that the tree is actually slower than the table. 

The reason for this could be happen would be that 

"your mmap'ed file in memory is loaded (and offloaded) by pages between all the levels of memory (the caches, RAM and swap)."

The requests are random, and all the ranges are close to evenly distributed over the whole ipv4-range. This means that all entries are equally likely, and there is no pattern in what ips are accessed. The kernel has no way of guessing what to load next.

Since the droplet has a limited amount of memory, memmap abstraction has to load and offload pages between disk and memory continuously. 

If the table needs random access, then the kernel can't guess what to load and offload, so it just has to pick "random" victim page. 
In each node and its children are generally stored on the file close to it (They get further apart the deeper you go in the tree). This means that the kernel can access multiple nodes at the same time, since they are loaded in the same page. This means that the kernel can actually search from "left to right" when searching for a nodes (and never from right to left, when it does go up the tree), which should be relatively quick. 

They could also answer why the balanced tree sometimes are a little slower than tree. Because the balancing effect makes sure that it can back and forth when accessing pages.

The page size on the vm is 4096 bytes, and each node is 48 bytes, meaning that each page store ~85 nodes


In general we can say that truly random access to a few bytes of data on a file should expect awful performance and if you access contiguous chunks sequentially from both files alternatively, you should expect decent performance.


https://stackoverflow.com/questions/43541420/when-and-how-is-mmaped-memory-swapped-in-and-out


On thing that i noticed when running the search test was that the tree search sped up. 

_____


## Build time Experiment

#### Expectation

It is difficult to predict how the build time will perform.

**Table**
The table is most limited by write speed. It should insert each entry in constant time, since it doesn't have to search for the insertion position, but it has to repeatedly store a pointer to the map table, for each ip in the range. For this experiment we have a range ranging between 10 and 18, so the table has to insert 10 to 18 pointers pr entry. I would expect the table to be relativly best on huge dataset, because of the constant insertion time compared to the tress log(n). 

**BST**
The BST only write twice to memory. One for actually placing the node/struct in the map and re-directing its parents pointer. But the slowing part is that the algorithm have to search down the tree every time. Leaving it with a O(Log(n)) insertion time. I would expect the BST to be the fastes on smaller datasets, because all nodes are stored next to each other in the file, and most nodes would probably loaded in ram + rebalancing is only really helpful if the dataset is big.

**Redblack tree**
The redblack tree is more difficult to predict, because it also has to balance the tree. Balancing the tree requires references to multiple nodes above the newly inserted node, and potentially many more if a rotation is needed. In this implementation it does not save a reference to the nodes it encounter down the search, so when balancing it has to re-access/request the node/struct in the memory map, meaning it. This should be an issue when the balanced is only one rotation, but this will be an increasingly problem, when the tree grows and bigger rotations happen. Furthermore we have to remember that the the nodes in the redblack tree are stored more spread out compared to the BST, meaning bigger change that the nodes accessed are not stored on the same page.
I would expect the redblack tree to be the the slowest in all cases, because of the huge amount of node accesses.

##### Dionysos

| Dionysos      | 1k        | 100k      | 150 mil           |
| --------------|----------:|----------:|------------------:|
| BST           | 3874      | 204523    | 686406388         | 
| Redblack      | 62079     | 5907163   | 12199050000       |
| Table         | 23758     | 1381446   | 874202017         |

The first thing we notice is that the redblack tree is the worst performant data structure, as we expected.

This experiment was also started on the 1gb machine, and i started with the 150mil, but stopped the test after 10 hours after seeing that is has only inserted 10% of the testdata, meaning it would not be able to finished in time, building at that rate. It was never a requrement to be able to build the datastrucutre on a low resource machine, but it could be interstring to do an experiment on how long it would actually take on different datasizes. 

Based on these numbers we can conclude that all of the data-structures are viable option for solving this issue, if ran on machine with specs like dionysos. This build time will increase the slower the machine. In the future a it could be interesting to experiment on how low the specs can be and still be able to build in 24 hours. 

As an very small experiment i tried to place a full array of pointers for each entry in the table instead of placeing each pointer indivually. The implementation was to create a array with the length of the range, and then loop over the array placing all the pointers in the array, and then copying the whole array into the memory map. This didnt make a difference, which also would be expected, since 

The longest build time is redblack tree with a average of 3.4 hours, which is way below the one day limit. 

From this experiment we can conclude that the redblack tree is the least scaleable solution, with this specific implementation. 


**Redblack tree on low resources.**
One of the biggest problems I encountered was i couldn't build the redblack tree from the whole dataset, but only on smaller datasets. 

In C memmap, mlock and all in the same family of functions and you can use them together. In rust there is no such thing. 
There is a type called `Pin<>`, where can pin memory in ram and 

I haven't managed to find a single place online where pinning and MmupMap is used/mentioned together, making me believe they were either not mean to be used together or no one have every tried. 

i made a test where i ran the redblack tree build, and locked all nodes with mlock (the unsafe c function), and the program died after 1000-1100 nodes. I was quickly to see if this was the whole program in itself that had a limit or it was just the MmupMap . So to test this i also added the mlock to the binary tree, and builded them sequentially without unlocking anything and that ran didn't crash. This means that the limit is not on the process, but on the memmap. 

This was maybe 

What you would expect from a memmap would be to load in page after page in only 


When 


```rust 
fn get_node_raw<'a>(mmap: &'a MmapMut, offset: usize) -> &'a mut Node {
    let byte_map = &mmap[offset..(offset+NODE_SIZE)];
    let number = unsafe { libc::mlock(byte_map.as_ptr() as *const c_void, byte_map.len()) };
    assert_eq!(number, 0);
    node_from_bytes(&byte_map)
}
```



## Caching Experiment

##### Expectation
In theory the cache shouldn't matter if the data-set consists of an infinitely large amount of entries, because the cache would be thrashed anyway - But on a more realistic scale (like in this project) this can become a factor when i comes to speed.

The immediate thought would be that the tree would benefit from this, since the nodes closer to the root would be read much more often than the rest of the tree, meaning that the data stored in the upper nodes can be retrieved from the cache. 

The BST would have better caching because the all top nodes in the tree are all placed right next to each other in the file (and there fore are stored in the same pages), and that is not necessarily the case for the redblack tree, because the root is moved around while balancing the tree. We will expect the gap between the trees to increase the bigger the data set.

For this implementation it is difficult to isolate the cache-miss counting only to the searching. This is coursed by the fact that the last step (mentioned in testing XXX) includes _generating search input_, _the actual lookups_, and _looking up the payload/value in the `payload_map`_. 
Both the _genration serach input_ and looking up the payload_ is the exactly the same step for all three datamodels meaning they can be seen as a constant factor in these cache tests. This means that generating and looking in payloads_table should be stable for all 3 tests.For the test performed the payload size was set to `1 byte`, bot make sure almost all accesses in the payload_map were cached. 

##### Results

All tests run with script found on Appendix B.

_den her er ikke spread_
| 1gb mem       | 1k            | 100k      | 10 mil    |
| ------------- |--------------:| ---------:|----------:|
| BST           | 42.594 %      | 51.339 %  | 46.182 %  |
| Redblack      | 48.678 %      | 53.470 %  | 50.712 %  |
| Table         | 45.949 %      | 52.400 %  | 10.399 %  |

This is just as expected. Stable cache misses. Higher count for 1 gb machine. Tree having the lowest cache miss-rate.

##### Dionysos - done 

| Dionysos      | 1k            | 100k      | 10 mil    | 150 mil   |
| ------------- |--------------:| ---------:|----------:|----------:|
| BST           | 36.693 %      | 42.852 %  | 77.475 %  | 67.873 %  |
| Redblack      | 36.267 %      | 41.271 %  | 79.102 %  | 69.431 %  |
| Table         | 35.864 %      | 41.901 %  | 87.384 %  | 79.413 %  |


As expected the table has the worst cache and the tree is best.

##### afterthoughts

As expected the table has the worst cache and the tree is best.

______

## Memory Usage Experiment
#### Expectation
I would expect to see that program would keep loading pages into memory as long as there is memory left. And only start offloading pages, when the ram is full.

#### Method
Make a search for every single entry in the table on the full data set on a droplet of digital ocean

#### Results:

**Using `free` command**

| Dionysos      | Idle      | Running       | difference        |
| ------------- |----------:|--------------:|------------------:|
| 8             | 140mb     | 270mb         | 130 mb            |
| 2gb           | 80mb      | 200mb         | 120 mb            |
| dionysos      |           |               |                   |

Another interesting finding was that the memory capped out after only running 10% af the table build. 

**1gb25gb**
<img src="../docs/images/ram_usage_1gb.png" alt="drawing" width="400"/>

11.50% - 14.20% = 2.5%
2.5% of 1 gb is 25 mb

done with `perf`

Spinning up a 8 gb machine on digital ocean - 

- 8 gb maskine caper ud ved 270mb - idle på 140mb -> 130 mb diff
- 2 gb maskiner capper ud ved 200mb - idle på 80mb -> 120 mb diff
- dionysos capper ud ved 25477976kb - idle 25243404kb -> 254 mb diff


#### Discussion
As we can see the memory usage never exceeds 17%. This shouldn't be the case since we just ran over 150 million entries equally spread out over 35 gb meaning we would expect to see much larger memory usage.

To figure out why it limits itself to further testing is needed. 

...

This is also noticable when doing 1500000 searches (every 100) in the table on the 2 gb mem droplet and the 8 db droplet. Here we could expect the 8 bg droplet to perform much better, since it didnt have the offload as much as the 2 gb droplet. As seen below this was not the case. They perform almost the same

```
100 gap 2 gb
## search_time_table
----------------------------------------------------------------------------------------------------
Search time --- #640696794 micro seconds, #1485148 of requests ran, #0 skipped
Search time --- #624812422 micro seconds, #1485148 of requests ran, #0 skipped

100 gap 8bg
## search_time_table
Search time --- #711256037 micro seconds, #1485148 of requests ran, #0 skipped
```

When doing the searches i tracked the memory usage on the droplet.
Both droplets on with both tree and table searches had a peak memory usage of 120 mb.
This limit was reacted after only searching though 2 procent of the request. and they it stayed pretty stable.
While testing it 

____


## Extra intersting findings.

## Compiler optimizations
200.000 entries - 10 gap
Done on a 2gb machine
-without release
--- table score: 23096, #18181 of requests ran
--- tree score : 75482, #18181 of requests ran
-with release
--- table score: 5800, #18181 of requests ran
--- tree score : 12593, #18181 of requests ran

here we can see that the factor table decreases by a factor of 3, and the tree decreases with a factor of 6. This goes hand in hand with thit the change we saw previously with the stackframes and the tail-end recursion. 

>Note: Compiler optimizations can be extremely hard to predict and understand, so i wont jump to any big conclusions based on this. But a interesting test to do. This test was done before the redblack tree implementation. 




# Final thoughs

It would be intersting to look into other tress types. Theoreticly fewest amount of key compariosons, but the bottlench is more on the amount of pagefaults. 



# Evaluation

Both table and the tree was under goal given from Siteimprove. 

The table is clearly not practical for handling ipV6
The table has duplicated data


## Code wise?


## design choices?


### Enchantments / Next Steps 

* Upgrade to redblack tree
* Actually adding a nice api, instead of only running the code through testfuctions/benchmarks.
* No reason to individually place each enty to the ip_table... i could just add them to an array in memory and then place that on disk
* The red-tag could be part of the names first bit - because otherwise it takes 64 bit

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

124.127.485 + 17.229.245 + 9.661.299 + 307.279 + 19.547 + 177.765    = 151.000.000
29.846      + 563        + 38.018    + 891.625 + 3.164  + 34.027.618 =  35.000.000
```


Appendix B

```bash
set -e

output='testdata/out/speed/benchmark.txt'
perf_cmd="perf stat -o $output --append -e task-clock,cycles,instructions,cache-references,cache-misses"
cargo_pre_cmd='cargo test --release --color=always --package rust_map --bin rust_map'
cargo_post_cmd='-- --exact --nocapture --ignored'

rm -f $output
hostname >> $output

printf "\nBUILDING RELEASE --------------------------------------------------------------------------------------------------------------------\n"
cargo build --release --color=always
sleep 2

printf "\ncreate_test_data --------------------------------------------------------------------------------------------------------------------\n"
$cargo_pre_cmd BenchmarkTests_Separate::create_test_data $cargo_post_cmd
sleep 2

printf "\nshuffling ---------------------------------------------------------------------------------------------------------------------------\n"
shuf input_data.txt > input_data_shuffled.txt



printf "\nbuild_tree --------------------------------------------------------------------------------------------------------------------------\n"
$cargo_pre_cmd BenchmarkTests_Separate::create_tree $cargo_post_cmd
sleep 2

printf "\nbuild_redblack ----------------------------------------------------------------------------------------------------------------------\n"
$cargo_pre_cmd BenchmarkTests_Separate::create_redblack $cargo_post_cmd
sleep 2

printf "\nbuild_table -------------------------------------------------------------------------------------------------------------------------\n"
$cargo_pre_cmd BenchmarkTests_Separate::create_table $cargo_post_cmd
sleep 2





printf "\nsearch_time_tree --------------------------------------------------------------------------------------------------------------------\n"
$perf_cmd $cargo_pre_cmd BenchmarkTests_Separate::search_time_tree $cargo_post_cmd >> $output
sleep 2

printf "\nsearch_time_redblack ----------------------------------------------------------------------------------------------------------------\n"
$perf_cmd $cargo_pre_cmd BenchmarkTests_Separate::search_time_redblack $cargo_post_cmd >> $output
sleep 2

printf "\nsearch_time_table -------------------------------------------------------------------------------------------------------------------\n"
$perf_cmd $cargo_pre_cmd BenchmarkTests_Separate::search_time_table $cargo_post_cmd >> $output
sleep 2
```