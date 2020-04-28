# Bachelor Report

# Intro - abstract
Speed is first priority

## Motivation

What do you do when you want to quickly search through a big data-set that can't be stored in memory? On a small scale, the easy answer is to just buy a more powerful machine, but this is maybe not always what you want. Should you choose to run a given program on a virtual machine on a cloud-provider like _digital ocean_ then - then rending a machine with many resources quickly becomes expensive. This is where this problem becomes relevant.

If the data cant be in memory it needs to be stored on persistant storage, but 
The data is not a normal key-value storage, since their keys in this case is a range between two values. This makes the choice of storage less trivial. 

## Problem formulation:	
- Analyze the requirements from siteimprove in terms of persistent log and key-value store
- Describe and analyze existing data structures and algorithms for persistent log and key-value stores
- Design/implement/evaluate a persistent log and key-value store in Rust on modern hardware

## Method:	
The method is experimental. I will design, implement and evaluate a prototype.

## Problem explained in detail
Siteimprove needs a web-service that can look up information of a given IP-address. 

The structure around the service (handling http request and so on), are already implemented, so they need the new algorithm for the actual data storage and lookup. Siteimprove's service is implemented in Rust, and therefore want the search algorithms to be implemented in Rust or (languages that can port to Rust)

The primary focus is fast lookup so their customers can get a result as fast as possible. Pre-processing time is not important as long as it doesn't take over a day. Space-wise is not important, since disk storage is relatively cheap, but if it had to be run on a cloud provider, it should be kept under 100 GB persistent storage.

The data-structure needs to be build once a week, and doesn't need to handle any new entry insertions after preprocessing/build of the data-structure. 

Siteimprove's wishes for a lookup time 60 ms on average. That is the highest lookup time they want to offer to their customers. 

Part of the challenge was to be able to run the service on a machine with low resources, to pay less for hardware in the long run (especially if ran on a cloud provider). The max memory usage should be 4 gb memory, but we should strive for as low memory usage as possible. 

Siteimprove had not need for persistent logging. 

### Data

The data is expected to be read from _standard input_ from a file or read from a stream. Each entry consists of two IP addresses and some related data/payload. The first IP determents the lower bound of the range and the second is the upper bound.
The 

The payload can vary in size, but the max payload size is 2^8/256 bytes.It is not possible to access to the real data due to confidentiality, but the average payload size pr. entry. is available. The number of entries are not constant, so the system needs to be able to handle a arbitrary number of entries. We know the usual average entry size, which we will use in this project to test up against. 

Their system needs to hand 150 million IPv4 entries and 35 million IPv6 entries with a payload of 256 bytes. To limit the scope of this project I will focus on the 150 million IPv4 and draw parallels to how the different data-structures would handle IPv6.

So to summarize the conditions / goals

### Si Rules/Priorities
```
- Language:             Rust.
- Dataset:              A set of IP ranges to firms.
                        No overlapping ranges. 
- Pre-processing-time:  Less af day.
                        No new entries after first lookup.
- storage space:        At most 100GB.
- Lookup-time:          First priority.
                        At most 60 milliseconds for average lookuptime. 
- Dataset:              150.mil ipv4
                        Up to 256bytes payload pr entry.
- Memory:               At most 4gb
```

**Assumptions of data**
* The input data contains no overlapping ranges
* No IP range is excluded (No IP range should be ignored because of reserved IP-range-blocks) / in other words... all IP addresses are possible.
* No need to remove or change entries after insertion. 
* The entries should be able to be stream into the program so no way of knowing how many entries will actually go into the system

# Data structures
There are many ways of searching through key-value pairs. The data for this problem consists of ranges, which means that the choice of database type is not obvious, and depends on different factors. It depends on range-size, gap-size (between each range), payload-size pr. entry, how many keys there can exist in total, and the number of entries - and of course how complicated of an implementation you want. 

if the total set of possible keys are finite. 

This
This project focuses on tree structures and simple tables table.

For this section we will refer use these variables:
```
p = payload size in bytes
e = number of entries
```
## Tables
```
delete?

Key-value store is a data storage paradigm

Each key is associated with only one value. This relationship is referred to as a key-value pair.
https://en.wikipedia.org/wiki/Key-value_database
https://www.aerospike.com/what-is-a-key-value-store/

The general understanding is that searching in tables is quicker than most data structures because you can get the data by value directly to a specific index by using the key. Tables can be implemented in many different ways, but the main point is that you can get value associated with a specific key by only doing one lookup. 
```
A simple implementation of a table is to just create a full table for all IP-addresses holding a value for each IP. This obviously results in massive data duplication because a value is stored repeatedly for each key in the associated range. This can easily be improved by actually storing the value in another table and only storing a pointer to it. Now the value is only stored once, but instead, the pointer to it is duplicated for each key. 

<img src="../docs/images/bachelor-04.png" alt="drawing" height="250"/>

One of the downside to this is the full IP range is stored in the database even though you may only have very few entries. A solution is generally to create some kind of hashtable, where keys are hashed and points to some other data-structure (like a linked list), but this is beyond the scope of this project. 

## Binary Trees

To prevent having a lot of duplicated code, another option is to store each entry as a node in a tree instead. A binary tree is a tree where each nodes has on parent and up to 2 children. 
A tree corresponding to the binary search-algorithm achieves the theoretical minimum number of key comparisons that necessary to find a value <ref til Knuth side 426>. Binary search has a search worse time complexity of O(log n), which means that we should aim for tree structure with the same time complexity.

### Binary Search Tree (BST)
BST is a type of binary tree in which the left child of a node has value less than the parent and the right child has value greater than the parent.

On average, binary search trees with n nodes have O(log n) height However, in the worst case, binary search trees can have O(n) height. - wiki. This means 

One of the choices you have to make is to decide if you want to store the payload next to the node itself or the node should store a pointer to payload somewhere else. 

Pros for storing the payload in the node:
- No need to spend time looking up the payload in a different file.
- The payload is probably already cached because of it right next to the node it just accessed.

Pros for storing it a separate file:
- In terms of caching it would be more beneficial to store the payload on a different file because it would mean that the nodes would be closer to each other - meaning they, therefore, make better use of locality while searching down the tree.

Another interesting point is to decide on how you want to store the IP-addresses. The simplest solution is to store the lower bound IP and the upper bound IP - each take up 32 bit - Resulting in 64-bit pr. node.
Another approach could be to only store the lower-bound and then store the delta to the upper-bound. This is useful if you know that the ranges will be small meaning you could get away with storing it on fewer bytes than 4 (32 bit). This is only useful optimizations if you know how the ranges and gaps are distributed, but since we can't do that in this project we have just gone with the simple solution and storing the full IP address for both upper and lower bound. 

### Redblack Tree

An extension of the Binary Search Tree is the redblack tree. A redblack tree is a self-balancing tree structure. This prevents the tree from being imbalanced in exchange for longer build time and bigger nodes.
Studies have shown that the redblack tree is the tree with the least performance variations.
https://en.wikipedia.org/wiki/Binary_search_tree


On important point to make is that it is not always beneficial to use a balanced tree. As Donald Knuth proves in *The art of computer programming, Volume 3, Sorting and searching, second edition, page 430* the search time for the balanced tree is not insanely better than a non-balanced tree on random insertion data. An unbalanced tree has a worse case search time of `O(n)`, but this is very rare and most trees are well balanced. A redblack tree has a `~Log(n)` and a BST has a `~2·log(n)` search time. Which men both data-structures has a time complexity of `O(log(n))`. This means that the increasing price of rebalancing the a redblack tree on large random data inserts, may not always be worth the lower height.
It has been showen, by Chris Okasaki, that insertion in a redblack tree only needs to handle four cases, which makes it easy to implement for project like this. 

<ref Okasaki, Chris (1999-01-01). "Red-black trees in a functional setting". Journal of Functional Programming. 9 (4): 471–477. doi:10.1017/S0956796899003494. ISSN 1469-7653. Archived from the original (PS) on 2007-09-26. Retrieved 2007-05-13.>
<De 4 cases (5) bliver gennemgået her www.geeksforgeeks.org/red-black-tree-set-2-insert/ >

# Implementation

## Why use rust?

Rust is a multi-paradigm system programming language focused on memory safety, especially safe concurrency. - wiki source please

Rust performs similar to C, which makes a good choice for performance. 
https://benchmarksgame-team.pages.debian.net/benchmarksgame/fastest/rust.html


### Safety

**Memory safety**

One of the main reasons of using rust is its safety. In general, Rust doesn't allow null pointers, dangling pointers, or data races.
This is done by a combination of the concept of ownership (which is basically a restriction of only having one mutable reference) and lifetime (which is a way to eliminate dangling pointers). All these are fixed are enforced at compile time.

```
Each value in Rust has a variable that’s called its owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.

This also eliminates C's issue of double free error. 
```

Rust has a concept of lifetimes. This means that if we have an array of items `[T]` and we create a reference to one of those items `&T` then that reference needs to leave scope before the array itself. In other words, the array needs to have a longer lifetime than outside pointers to its elements - otherwise, the rust compiler won't compile because it can't guarantee that the array isn't de-allocated or changed before accessing `T`. This is both a huge challenge when first starting to work with Rust, but also a really great safety. This is great for this project, because we eliminate change of dangling pointers, which can be a pain, when building complecs data-structure which a lot of moving pointers. 

```
delete?
//hypotese
This concept usually works great, but it has its challenges when using a memory map because it can guarantee that the nodes/structs that the pointer points to are still in memory because the page it is stored on is maybe offloaded, by the kernel/memory map. 
```

Starting this project is was the plan to let nodes refer to each other by using a `&T` when building a tree. But because of these compiler challenges mentioned above, I chose to instead go for an implementation where each node stored a byte-offset to where its children were stored the memory map. 

**Reading from Memory Map**
Sadly sometimes we can cant use rust's safety, and this is where rust works more like C.
```rust
pub(crate) unsafe fn bytes_to_type<T>(slice: &[u8]) -> &mut T {
    std::slice::from_raw_parts_mut(slice.as_ptr() as *mut T, std::mem::size_of::<T>())
        .get_mut(0)
        .unwrap()
}
```
This function returns a reference to a mutable object given a reference to a `u8` array. This function is used to get a reference to a node directly on the memory map. Here we have no guarantee of we are going to get, since it just a pointer and a length that we force to become a reference to type T. I this case we don't have any other way since Memory Map only know the concept of bytes.

**Error handling**
C doesn't provide good error handling, because the programmer is expected to prevent errors from occurring in the first place. -wiki source? This means C is much harder and unsafe to code combined with it is very difficult to debug. 

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
Both concepts are used in this function above. Option is used in the form of `Some` and `None` and Result is used in `Ok(n)` and `Err(E)`. This function takes a string of 4 numbers separated by a dot `.` - e.g. `192.2.103.11` - and returns unsigned integer wrapped in an option. In this case, I use Option as a safe way to use a null-pointer. Being able to handle an error with ease is crucial when needing to deliver save code quickly. 

https://en.wikibooks.org/wiki/C_Programming/Error_handling

### Rust combined with C
Rust does not have an official interface/abstraction for using memory maps, but there exist a few open-source libraries created by the community. 
Rust's package management system is called cargo and use the crates as the packages. This uses a crate called `memmap` (version `0.7.0`). This library was chosen based on the fact that it had the most stars on Github. The abstraction provided by the external libraries is not extensive compared to the using the native C, meaning that the setting for the map is not as customizable. 

```
delete?
Rust has the ability to call directly into C files, and you also have the ability to use most of the c standard library inline by using the `libc`- library/crate. This means we can access functions like `mlock` and `mlockall`. `show example`. 
But rusts memory safety can not guarantee the result of these function so it forces us we need to use the "unsafe" keyword. Overall this means that we can use both rust functions and c functions as we please, but we can't guarantee what is going to happen.
```

https://doc.rust-lang.org/nomicon/ffi.html

https://medium.com/@thomascountz/ownership-in-rust-part-1-112036b1126b

https://medium.com/paritytech/why-rust-846fd3320d3f

https://stackoverflow.com/questions/33985018/cannot-borrow-x-as-mutable-because-it-is-also-borrowed-as-immutable
https://stackoverflow.com/questions/47618823/cannot-borrow-as-mutable-because-it-is-also-borrowed-as-immutable


## Design
I this project I have gone for implementing a Binary Search Tree, a redblack tree, and a table. All three implementations have their own module in the source code and have the same interface, so they can be swapped interchangeably. All data-structures are implemented using memory-mapped files. All three implementations use a separate memory-mapped file for storing the payload/values. This memory-mapped file will be referred to as `payload_map`.
In this implementation, I have chosen to store strings as payload, but this could be swapped out with any other datatype.

Before diving deeper into the implementations, we have to look at difference between fixed data-sizes, vs dynamic data-sizes. 

#### Fixed vs. dynamic data length 

Depending on the problem you want to solve you can either choose to use the same fixed amount of space for each entry or have a dynamic size - meaning you only use the necessary amount of space for each entry. 

This choice is important for deciding how to store the payload and how we store the nodes in the tree. 

Fixed sized data could imply using a struct - meaning that the whole file is cut in equal-sized pieces (structs). This means you can refer to the offset of the struct itself, and not to the byte index of the struct. This is important because the byte-index number will be much larger than the struct index, meaning it takes more space to store pointers to byte indexes.

<img src="../docs/images/bachelor-05.png" alt="drawing" width="600"/>

<E.g. using a u32 to as a pointer to byte-index result in only being able to refer to max size data size of `2^32 · 8 bytes = 43.4 · 10^9 bytes = 4,3gb`.>
Struct indexes are great if you know the data-object always will have the same size, but if the amount of data needed to be stored varies a lot, then we will wast space on internal padding in the structs because they are not filled out. This means we instead can make all data-objects have a dynamic size. This would result in us having to store the size of the data object in the header (because we don't know the size of it) and need to use byte-index to refer to the data. 


```
delete?

On the other hand, dynamic data size means that --- Dynamic payload means that you for each entry great, since you don't waste space on padding/empty payload, but the downside is that you have to store the size of each block and in the block itself and you have to store the address the addresses payload begins instead of only storing the index to the node/struct of payload you are referring to. --- This is important because this means that the byte index always will be a bigger number than the struct offset. Therefore it is not always beneficial to use dynamically sized payload if the number of pointers is huge since the amount of space needed accumulates. --- This means that an address-pointer of 32b can only point to a max size of ~4.3 byte data ----- For this project I have chosen dynamic payload length because the payload consists of names, which can vary a lot in length. If fixed length was chosen I would either have to accept a large amount of wasted space or not allow names to be over a given length meaning I would cut of names.
```


### Payload Map
This memory-mapped file contains all entries' value and the length of the values in bytes. A value is retrieved from the map by giving it the byte index of the header of the value. Each lookup runs in constant time and therefore has a time complexity of *O(1)*. 

Each value has a header of one byte, which is used to store the length of the data. The length is necessary because we don't know how far we need to read to get the value.
The length of the payload is stored on 1 byte, which means that the payload can be at most be `2^8 = 256` bytes long. This is just a design choice, but could easily be extended by changing all headers would need to be 2 bytes long instead. 

On this picture we can see how `SKAT` would be stored.

<img src="../docs/images/bachelor-03.png" alt="drawing" width="600"/>

**Space**
The space needed for this file can be calculated from the max payload size and the number of entries: `(255 + 1) · n`, where `n` is the number of entries. The `+1` is the header-size of one byte. If we have 150.000.000 entries with 255 bytes each, we can calculate the largest possible file to be 38.4 GB. We know that average 

## BST & Redblack Tree

Both the BST and the redblack tree is implemented very similarly. 

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
    pub left: u32,
    pub right: u32,
    pub payload_ptr: u64,
}</pre></td><td><pre>
pub struct Node {
    pub red: bool,
    pub min_ip: u32,
    pub max_ip: u32,
    pub left: u32,
    pub right: u32,
    pub parent: u32,
    pub payload_ptr: u64,
}</pre></td></tr></table>

>`min_ip` being the lower-bound IP, `max_ip` being the higher-bound IP, `left` being the left child, `right` being the right child, `parent` being the parent node and `payload_ptr` being a pointer to the `payload_map`, and `red` is the indicator of the node being red or black.

The `min_ip` and `max_ip` is a `u32`, because IPv4 is 32-bit. Pointers to other nods is `u32`, because we know that there will be at most 2^32 nodes, if the tree only handles IPv4.

**Insertion**
Each time an entry is added to the tree a new node will be appended at the end of the memory mapped file. Because all nodes have the same size, we can point to their node-offset instead of their byte-offset. The only difference between the two trees is we store the root-node in the first struct in the redblack tree.

<img src="../docs/images/bachelor-06.png" alt="drawing" width="600"/>

Here we have a simple example of what it would look like if these entries were inserted. 
```
0.0.2.8 0.0.2.8 SKAT
0.0.0.4 0.0.1.20 PWC
0.0.0.0 0.0.0.2 Siteimprove
```
![](../docs/images/bachelor-08.png)
Here we notice that the BST is not balanced and has Node 0 as root and Redblack is balanced and has Node 2 as root. Reference to another node (left,right,parent) with value 0 is treated as a null-pointer.

**Space**
The space complexity of the trees are O(n). Each node is a struct, which size can be calculated by doing the following: 
For each field in the struct ordered by declaration order:
* Add the size of the field.
* Round up the current size to the nearest multiple of the next field's alignment.

Finally, round the size of the struct to the nearest multiple of its alignment.

Following this algorithm, the BST nodes have a size of 24 bytes while the redblack nodes have of 32 bytes. Multiplying this with the 150mil entries, give a total file size of 3.6GB for BST and 4.8GB for redblack tree.

```
dette er ikke sandt - delete?
A small space-optimization in the redblack tree be to let the boolean be stored as the most significant bit of the payload-pointer, reducing the size to only be 44bytes (assuming we would never get a total payload of `2^64 = 1.8 · 10^19 bytes`). 
```

https://doc.rust-lang.org/std/mem/fn.size_of.html
https://www.geeksforgeeks.org/is-sizeof-for-a-struct-equal-to-the-sum-of-sizeof-of-each-member/ 

**Implementation overview:**
``` 
BST: 
lookup speed: O(Log(n))
Insert: O(Log(n))
Space: 24 bytes · n 

redblack:
lookup speed: O(Log(n))
Insert: O(Log(n))
Space: 32 bytes · n 
```

**Handling IpV6**
Tree structures handles IpV6 well. The only change necessary would be to change the `min_ip` and `max_ip` to from `u32` to `u128` (and declare them at the bottom of the struct instead, because of alignment).

## Table

This implementation is based on the simple implementation mentioned in section *Tables*. This file consist of `2^32`(~4,3 million) unsigned longs, `u64`, that functions as a pointer to lookup the value in the `payload_map`.

An illustration of the data-structure can be seen below
Here we have the datastrucuted constructed of the same entries as in the trees
```
0.0.2.8 0.0.2.8 SKAT
0.0.0.4 0.0.1.20 PWC
0.0.0.0 0.0.0.2 Siteimprove
```

<img src="../docs/images/bachelor-02.png" alt="drawing" width="600"/>

To symbolize a null-pointer (meaning the IP, does not have any value) we just store 0. This means we need to add 1 to all pointers do differentiate between null-pointers and real pointers that refer to the first value in payload_map at index 0. This is why we e.g. see IP 200 with value 6 points to byte index 5. 

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
The amount of possible ips is `2^128 = 3,40e38`, and if all have to store a `u32` it result in a file a `3,40e38*32/8/1000/1000 = 1.3·10^d30 gb` file.

# Testing, Debugging, and Profiling

To ensure the data structures functioned correctly almost all functions in the code has unit-tests. 
An important node is that the tests has to be run with the flag `--test-threads 1`, to make sure they run sequential, because many functions use the same file, and this eliminates risk race-conditions. 

The tests can be categorized into unit tests and integration tests.


### Unit tests

Most files and functions are tested using unit tests. All unit tests can be found in source-code in the same file as the function they are testing. Below I have chosen to highlight some of the more speciel tests. 

**Verifying tree structure**

The unit tests also include test that check that the tree was build was built correctly. Both the BST and redblack tree can be printed to standard-out or a file, and both have a test to verify that the tree is printed correctly. 

Underneath we see the printout to the left and the abstraction to the right of a redblack tree, Where `O` is a black node and `X` is a red node. These tests are on fixed data set (meaning it is not random generated), which means we can check if every single line matches with what we expect. The testdata for generating this can be found in appendix X.

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

The method above works great on small data-sets, but I doesn't scale, so I have added a test, that checks that the redblack tree is build correctly. The function can be seen below.

```rust
fn is_tree_corrupt(mmap: &MmapMut, parent_red: bool, node: &Node) -> bool {
    if parent_red == true && node.red == true { return true }
    let right_is_corrupt = node.right != 0 && is_tree_corrupt(mmap, node.red, NodeToMem::get_node(&mmap, node.right as usize));
    let left_is_corrupt  = node.left  != 0 && is_tree_corrupt(mmap, node.red, NodeToMem::get_node(&mmap, node.left  as usize));
    return right_is_corrupt || left_is_corrupt
}
```
This function traveres the redblack tree and checks if a child and its parent is red, which is illegal state, and should have triggered a rebalance. In the source code, a positive and negative test is performed on 50.000 random inserted elements, to ensure that the tree redblack tree was build correctly. This function will return `true` if the tree is corrupted and `false` otherwise. This test doesn't detect if any nodes are disconnected from the tree (meaning they are not reachable from the root), but many other test detect that.

### Integration Tests 
Since all three data-structures has the same interface, they can all be tested by using exactly the same functions. 
The integration tests include:
* Hardcoded input and the requited IPs are also hardcoded
* Hardcoded input but the selected Ip-requests are a random subset from the hardcoded input
* Random generated input data, with random selected IP-request and the selected Ip-requests are a random subset from the input

The Integration tests go through a setup, build, and lookup phase. 

**Setup - Generating test files**
This first phase generates lines of two IP addresses and one text string (e.g.: `125.74.3.0 125.74.3.10 Siteimprove`) and writes them to a file. All lines are shuffled by using the linux command `shuf`. 
> Note: First I tried shuffling all entries in memory inside rust using a `Vec<&str>` and writing them to a file afterward, but this was slower and was more memory intensive than using the linux command. Both methods require that all entries can be stored in ram at the same time. This means that the shuffling cant happens on a machine with low resources like the 1 gb memory droplet, if the dataset is big. <Algortimer eksisterer hvor man kan shuffle uden at bruge ram>

**Build data structure**
The program iterate over each line reading them one by one with regex. Both the tested data-structure and payload_map needs to be built at the same time, meaning each entry is sat into both at the same time because the data-structure needs to know the byte-offset of the currently inserted entry. This step is deterministic and will always provide the same output for the same input file. This phase it the most expensive.
> Note: All three data-structures produce the same `payload_map`, so they could, in theory, share the same payload-file, but implementation wise they all have their own to decouple the data-structures. 

**Lookup**
Testing lookup speed is done by creating some IP-requests and running them on the data-structure. The random requests are collected by iterating over the shuffled list of entries and picking every n'th entry. For each entry, a random IP is picked between the upper and lower bound. All the chosen entries are then shuffled again. The actual searching is done by looping over the chosen random IPs, and sequentially searching through the data structure and checks that it returns the correct payload. When finished it will print the time it took to do all the lookups. This number is then used to calculate the average lookup time. 

### Debugging
Debugging the system was mostly done with printlines and by stepping through the code with a debugger. It can be pretty difficult to visualize how exactly each byte is placed in memory maps. The method I used to see it was to print the memory map in bytes. I used this statement `println!("{:?}", &name_table[0..100]);`, which prints out each byte in the range of 0 to 100 of the memory-mapped file: `[0,0,0,255,90,0 ... ]`. This way I can print the map before and after each operation and compare them, and check if it works as intended. 

When building the redblack tree there is a assert-check while balancing that checks that a child's parent-pointer is the same as it's grandparents' child pointer. This is catch mistakes, should the redblack tree end up being corrupted. This was crucial in the development process. I had a periodic issue where the redblack tree was getting built correctly. In the end I found out that I had made an assumption, which was wrong. The assumption was that the nodes always had a different color when `swapColor(node1,nod2)` was called (`line 119, Redblack/Tree.rs`), based on the discription in XXX. After fixing this, I didn't have any corrupted builds. 

### Profiling 

A huge part of the performance optimization came from the build-in profiler-tool in _Jetbrain's Clion_ (Jetbrain's low-level-programming IDE). In particular its _Flame Chart_ and _Call Tree_ were very helpful. This was mainly used for seeing how much time the process spend in each scope/stackframe/function to find bottlenecks. The profiler use _sampling_.
"A sampling profiler probes the target program's call stack at regular intervals using operating system interrupts. Sampling profiles are typically less numerically accurate and specific, but allow the target program to run at near full speed."-wiki

https://www.jetbrains.com/help/clion/cpu-profiler.html
https://en.wikipedia.org/wiki/Profiling_(computer_programming)

This was most useful at the beginning both for learning rust and for detecting bottleneck early on. E.g in an earlier version of the project a new Regex object was initialized every time it read a line for standard input. In the profiler it was an obvious bottleneck - and was therefore changed to only getting initialized once and just parse a pointer to it round in the system. This is a simple thing but has a huge impact on performance. On the left image, we see how 56% of the time was spent initializing a Regex object, but it only took 14.7% after the change.

<img src="../docs/images/profiler/CallTree1000pre.png" alt="drawing" width="49%"/> <img src="../docs/images/profiler/CallTree1000post.png" alt="drawing" width="49%"/>

The height of the trees is also visible in the profiler in the flame graph. The image below is from a profile run on a function that builds both Redblack, table, and BST with 100.000 entries and it with a frequency of 5000 samples per second.
Since it ran with 100.000 entries we can expect at a minimum height of the tree to be `log(100.000)=16`. In the flame graph, we can count how many stackframes deep a the redblack's `insert_node` functions goes. The last `insert_leaf_on_node`stackframe is 18 layers deep, which means that the height of the tree is 19 (+1 because the inserted leaf also counts). This matches our expectations of a balanced tree.
Furthermore, we can see that the BST height is almost double the height of the redblack tree. This also matches our expectations of unbalanced tree height to have a hight of `2·log(n)`, proved by Knuth mentioned in a previous section.

<måske brug ord som "call stack" https://en.wikipedia.org/wiki/Tail_call>

<img src="../docs/images/profiler/100 5 arrows.png" alt="drawing" />

>Note: The BST may even be taller/deeper, since the profiler tasks samples on a given interval, so if a stackframe is added and removed to the call stack in the middle of two samples it would not be displayed. 

Another interesting finding was that rust only optimized to tail-end recursion when running it in release mode (running it with the `--release`-flag). Below we can see that there only exists one `insert_leaf_on_node`-stackframe at the time, meaning that the optimizer created tail-end recursion. 

<img src="../docs/images/profiler/release.png" alt="drawing" width="45%"/> <img src="../docs/images/profiler/nonrelease.png" alt="drawing" width="45%"/>


# Running the program

The program is ran through the command line. This version of the program can either generate data self or read it from a file. What the program is suppose to do is specified with flags and options. The full list of flags and options can be found in appendix X.

Here we have some examples:
* To generate a data-set of *n* entries, building a table and search through 10% of the data-set (10% is default) :
`./rust_map --generate_data --build_redblack --search_redblack -n 100000`
* Building the redblack tree and searching for a specific ip:
`./rust_map --build_redblack --search_redblack --input_file MyFile.txt --specific_ip "160.5.211.97"`
* Searching in both BST and Redblack for for a specific ip on already built data-structures:
`./rust_map --search_BST --search_redblack --specific_ip "160.5.211.97"`

Some flags and options have a invalid combination. The program will tell you what is wrong and help you provide the right input. Here we have two examples of example of an invalid input:
* `./rust_map --build_redblack` where you don't specifying a input file `--input_file MyFile.txt`and does not use the flag `--generate_data`, because then the program does not have any thing to build off. 
* `./target/release/rust_map --specific_ip "160.5.211.97"`, where you tell it to search for that specific ip, but does not tell it which data structure to search in.

# Experiments

To evaluate how the differently models perform, and which live up to the goal/needs, I have done 4 experiments. 

```
In the previous sections, we have explained the three data-structures, table, BST, and redblack tree. To compare these data-structures and see which live up to the goal/needs the project went through various experiments. 
```

All tests has been ran at least 3 time, so limit the amount of random divination. The numbers seen the the tables below are averages of the tests results. If a datapoint had a lot of variation for each run, I ran it a few extra time to get a bigger sample size. If the time-scope was larger for this project, I would have ran more tests to get more accurate results. All tests can be found in the source code.

### Test data

The full IP-range for IPv4 is 2^32 = ~4.3 billion and the number of entries is 150.000.000. This means that there is a new range every 28th IP (`2^32/150.000.000 = 28.7`) on average. Since there is no information on how these are distributed, these test we will assume they are (relatively) evenly distributed over the full range of IPv4. 
This means when we experiment on 150 million entries, we set all ranges to be a random number between 10-18 and the gap between the ranges to be between 10-18. On average this adds up to 28, which means the full ip range will be hit.
The range and the gap is a random number between two values, to add a diversity and make it a bit more realistic. 
Evenly distributing ranges would suggest the worst-case scenario for the table, because the entries would be spread over more pages, than if all entries were small and close to each other.  

To see how the data-structures handle differently sized data-set, the tests also include datasets on e.g. 100k entries. A data-size of 100k entries still has a range of 10-18, but the padding/gap between each entry is `2^32/100.000 - 14+-4 (range) = 42.936+-4`.

The test data is generated as explained in _Integration Tests_ above.

> In the results below (and in appendix) the constant for the tests are displayed in this format `range: 10..18, payload_size: 50, gap: 10`, where `range` is the range of each each entry. In this case each entry can be between 10 and 18 IPs. `payload_size` is the size of the payload for each entry. `gap` is the number of entries it skips when it collects random lookup data. A gap of 100 basically means that 1 out of 100 entries will be requested in a test. 


### Machines

Tha experiments has ben run on different machines. Both high a low memory machines are included. The focus of this project is to search on a low resource machine, so this will be our main focus with the final evaluation - but still use a high resource machine as a benchmark for comparisons. All machines specs can be found in appendix X. The machines will be referred to as following:
* Dionysos: 220 gb memory machine on a XX disk.
* 1gb droplet: 1 gb memory, 1 cpu, 100gb volume disk, hosted on digital ocean.
* 2gb droplet: 2 gb memory, 1 cpu, x disk hosted on digital ocean.
* 8gb droplet: 3 gb memory, 1 cpu, x disk hosted on digital ocean.

I shared Dionysos with another person, so i cant guarantee what else is going on the computer, when i experiment on it... but we tried to coordinate as much as possible so this should have minimal impact. 

## Search time Experiment
#### Expectation

From a purely theoretical standpoint, we would assume that the table is the fastest, followed by the redblack tree, followed by the BST. The table should run in constant time because it only needs to do 2 lookups (one in the table and ond in payload_map). Both trees should have a `O(log(n))` lookup time, but I would expect the BSTd to be slower than the redblack tree because it is not balanced therefore need more key comparers to reach deeper nodes. 

I would expect Dionysos so be much faster than the 1 gb droplet, because it has more ram and should be able to hold more pages in memory before offloading them. Since Dionysos has 120 gb ram it should be able to have all pages loaded in memory for super fast search. 

#### Results

The tests for this experiment has been ran on both Dionysos and the 1gb droplet.
All tests are ran with: `range: 10..18, payload_size: 50, gap: 10`.
The numbers in the table is the average lookup time pr requested IP. All numbers are in microseconds.
These speed tests (besides 150mil on 1gb droplet) are ran right after the build of the model without shutting down the program. This was done to maximize speed by having as many pages already in memory as possible. The shell script for producing the results can be found in appendix X

**Dionysos**
| model         | 1k      | 100k      | 10 mil    | 150 mil    |
| :------------ |--------:|----------:|----------:|-----------:|
| BST           | 3       | 0.7       | 2.1       | 3.74       |
| Redblack      | 3       | 0.9       | 1.7       | 3.15       |
| Table         | 6       | 3.6       | 3.4       | 1.001      |

**1gb Droplet**
| model         | 1k        | 100k    | 10 mil    |
| :------------ |----------:|--------:|----------:|    
| BST           | 1.5       | 0.8     | 8.8       |
| Redblack      | 1.5       | 0.8     | 7.7       |
| Table         | 5.5       | 4.5     | xxx       |

> `*` The 1gb25gb can't build (as explained in the next experiment) meaning that model has to be build on Dionysos and copied to the 1gb droplet. This means that the cache was cold and no pages were loaded into memory, before the speed speed test ran. 

#### Discussion

**Dionysos**

1. Starting with Dionysos, we can see that the table is the fastest on the 150mil-data-set, which was what we expected. 


**both**
One thing that is interesting is that both trees actually follow along pretty well. One could expect the the redblack tree double the speed of the BST, because of the lower height, but this is not the apparently not the case. This can be cause by better usage of locality. When searching down the BST, you search in one direction through the file, and the nodes are often close to each other in the file.
The redblack tree may have a fewer amount of key-comparisons/node-accesses, but nodes are more randomly distributed on in the file because of rebalancing. 
<img src="../docs/images/bachelor-10.png" alt="drawing" width="800"/>
e.g the root is always at node offset 0, and all nodes in the tree will always be right from their parent on the same page or the following pages, while the redblack tree's root could, in theory, be in the middle of the file, and its children could be on any far apart. This means that the redblack tree has a bigger chance of page faults.
Another reason the BST and redblack tree is closer to each other could be that the redblack nodes are bigger and therefore less nodes can be stored on the same page. This means that the redblack tree will also get more pagefault because it can have less nodes loaded in at the same time. This also has impact on the performance, but this should be minimal though.

For smaller data-sizes the table worse than the trees. Even though the table has fewer accesses, I would expect it to be relatively worse if the ranges are spread out (bigger space between them), since the entries would be placed further from each other in the file and therefore the table won't load mutable ranges in the same file. The requests are random, and all the ranges are close to evenly distributed over the whole ipv4-range. This means that all entries are equally likely, and there is no pattern in what ips are accessed. The kernel has no way of guessing what to load next. The trees, on the other hand, would perform better on smaller data sizes, because all nodes are stored right next to each other on the file and will, therefore, be load on the same page and will be cached. This can answer why table is much worse than the tree on smaller more spread out data. 

For both machines we see a drop from 1k to 100k. The reason for the the high lookup average of 1k entries, is probably the overhead of print-lines to standard-out. I implemented (no using external library) a simple progress bar, that prints a dash `-` every time it has processed 1% of the entries (both for searching and building), and flushing immediacy (because i had issue of it not being up to date). This has close to no impact, when there is 150 million entries, but it is a lot when the there is 1k entries, and it has to stop and flush every every 10th entry. This should probably have been a feature you could toggle as a flag.  

It is also important to note that when the dataset is smaller the usikkerhed is higher. When we do 1000 entries and 10% lookup, then only 100 requests are actually run which is a much lower sample size than 1500000, which we do with. We need to remember this since the variation in the raw test data is more spread out. We just have to keep this in mind when reading the results.



```
"your mmap'ed file in memory is loaded (and offloaded) by pages between all the levels of memory (the caches, RAM and swap)."
The requests are random, and all the ranges are close to evenly distributed over the whole ipv4-range. This means that all entries are equally likely, and there is no pattern in what ips are accessed. The kernel has no way of guessing what to load next.

Since the droplet has a limited amount of memory, memmap abstraction has to load and offload pages between disk and memory continuously. 


They could also answer why the balanced tree sometimes is a little slower than the BST. Because the balancing effect makes sure that it can back and forth when accessing pages.


The page size on the vm is 4096 bytes, and each node is 48 bytes, meaning that each page store ~85 nodes


In general, we can say that truly random access to a few bytes of data on a file should expect awful performance and if you access contiguous chunks sequentially from both files alternatively, you should expect decent performance.
```


https://stackoverflow.com/questions/43541420/when-and-how-is-mmaped-memory-swapped-in-and-out

https://www.webopedia.com/TERM/P/paging.html
https://www.webopedia.com/TERM/S/swap.html


_____


## Build time

#### Expectation

For this project, there were no system requirements for the machine that should build the data-structure. The only requirement was that it had to be built in less than a day. This experiment has been build on Dionysos.

**Table**
The table should be most limited by write speed. It should insert each entry in linear time `O(r)`, where r is range of entry, since it doesn't runtime doesnt grow with the number of entries, but it has to repeatedly store a pointer to the map table, for each ip in the range. In these experiment we have a range ranging between 10 and 18, so the table has to insert 10 to 18 pointers per entry. This means the insertion complexity can bee seen as constant `O(1)`. I would expect the table to be best on a large data-set, because of the constant insertion time compared to the trees' log(n) insertion time. 

**BST**
The BST only writes twice to memory. One for actually placing the node/struct in the map and re-directing its parents' pointer. But the slowing part is that the algorithm has to search down the tree every time. Leaving it with a `O(Log(n))` insertion time. I would expect the BST to be the fastest on smaller datasets because all nodes are stored next to each other in the file (great use of locality) and most nodes would probably be loaded in memory.

**Redblack tree**
The redblack tree is more difficult to predict because it also has to balance the tree. Balancing the tree requires references to multiple nodes above the newly inserted node, and potentially many more if a rotation is needed. In this implementation, it does not save a reference to the nodes it encounters down the search, so when balancing it has to re-access/request the node in the memory map. This shouldn't be an issue when the balanced is only one rotation, but this will be an increasing problem when the tree grows and bigger rotations happen.
Furthermore, as described in the discussion oin the previous experiment, we have to remember that the nodes in the redblack tree are stored more spread out compared to the BST, less use of locality. Overall I would expect the redblack tree to be the slowest in all cases, because of the huge amount of node accesses.

##### Results
This tests are done by running the shell script found in appendix X on dionysos.
All numbers are in microseconds. 

**Build time per data-structure**
| Model         | 1k        | 100k      | 10  mil      | 150 mil           |
| --------------|----------:|----------:|-------------:|------------------:|
| BST           | 3874      | 204523    | 29716768     | 857455857         |  
| Redblack      | 62079     | 6156475   | 655914608    | 11897741304       |
| Table         | 23758     | 1381446   | 144225979    | 930878211         |

**Average insertion time pr entry**
| Model         | 1k        | 100k      | 10  mil      | 150 mil            |
| --------------|----------:|----------:|-------------:|-------------------:|
| BST           | 3.8      | 2.0        | 3.0          | 4.5                |  
| Redblack      | 62.1     | 59.1       | 65.6         | 81.3               |
| Table         | 23.8     | 13.8       | 14.4         | 5.8                |



The first thing we notice is that the redblack tree is the worst performing data structure, as we expected.

As mentioned in the previous experiment, building the data structures also makes use of the progress bar made out of flushed printlines. Again this has a meaningful overhead on small data sizes, which i probably why we see a decrease from 1k to 100k in average insertion time for all data structures. More on this can be found in the previous experiment.

One interesting point is that the percentage difference between the BST and table decreases the bigger the data size (besides the jump from 1k to 100k, as described just above). The difference between them goes from 16% -> 14% -> 20% -> 70%, which indicates that the table will catch up to the BST the bigger the dataset. This is kind of expected considering constant time scales better than logarithmic. For bigger datasets the table may have been a better option, but not for this data size.

This experiment was also tried on the 1gb machine, but with less success. When building the redblack tree with 150 mil datasize, I stopped the 1gb machine after 10 hours after seeing that has only inserted ~15% of the test data, meaning it would not be able to finish in time, building at that rate. It was never a requirement to be able to build the data-structure on a low resource machine, but it could be interesting to do an experiment on how long it would actually take on different data-sizes. 

> Note: As a very small experiment, I tried to place a full array of pointers for each entry in the table instead of placing each pointer individually. The implementation was to create an array with the length of the range and then loop over the array placing all the pointers in the array and then copying the whole array into the memory map. This didn't make a difference, which also would be expected, since writing to an array in memory would be the same as writing to the map in memory. 

The longest build time is the redblack tree with an average of 3.4 hours, which is way below the one day limit. Based on these numbers we can conclude that all of the data-structures are a viable option for solving this issue if ran on a machine with specs like Dionysos. This build time will increase the slower the machine. In the future, it could be interesting to experiment on how low the specs can be and still be able to build in 24 hours. 

The best performing data structure seems to be the BST - This is probably a combination of only 2 writes to memory and good use of locally.

From this experiment, we can conclude that the redblack tree is the least scalable solution, with this specific implementation. 

## Memory Usage Experiment
#### Expectation

Memory is an important factor when working with memory mapped files. The kernel should keep loading in pages as long as there is free memory left - and only start offloading pages, when the memory is close to full.
A page are 4kb, which means that if we in a memory mapped file access data at least once every 4kb, we will load all pages in the whole file. So if we build the full table with a padding/gap of 10-18 bytes, we would have to load in all 2^32*64 bit (34.4 GB). This is of course not possible on a 1 gb memory machine, so the kernel has to start offloading pages again. The victim page is chosen by some algorithm in the kernel. If it keeps loading and offloading the same page it is know as thrashing, since it keeps asking for something and throwing it away even though it needs it again in the near future. This is a challenge when IP-requests are random, because it cant predict what to keep in memory, if there is no pattern.

Overall the expectation is that machine will keep loading in pages as long as there is space left on its disk.

#### Method
To test this I built a table on four different machines, with 150 entries, and check memory usage, before and under the building the table. Here we would expect it to use all ram available and after that loading and offloading pages (paging). Too track the memory usage I used the Linux command `free`, that prints the current memory usage before and while the program is running. 

#### Results:

**Calling `free` command**

| Machine       | Idle      | Running       | Difference    |
| ------------- |----------:|--------------:|--------------:|
| 1gb           | 97mb      | 160mb         | 63 mb         |
| 2gb           | 85mb      | 203mb         | 122 mb        |
| 8gb           | 142mb     | 271mb         | 129 mb        |
| Dionysos      | 9681mb    | 9612mb        | 69 mb         |

The free command was ran many times during the build of the data-structure, and already after building 10% of the input data, the memory usage stopped increasing and had a stable memory usage the rest of the building-process.

#### Discussion

The fact that the memory usage didn't keep growing means that there is a limit somewhere in the system, since it starts paging before all memory is used. This can also bee seen by looking at the digital oceans monitoring tool. Looking at the 1gb machine over the time period of one week, where all tests in this project were performed, it never got over 33% memory usage. 
<img src="../docs/images/ram_usage_1gb2.png" alt="drawing" width="400"/>

In the schema above we see a pattern for the differences being a multiple of ~64mb. To figure out why it limits itself further testing is needed. 
This also means that the lookup and build speeds could potentially be much faster, if the memory map could use its full memory, which was the goal from the beginning. 


As mentioned building the table with 150 million entries would use 2^32*64 bits (34.4 GB), and therefore a machine with more memory would be faster than er low memory resource machine. But this was not the case when running a search with a gap_size of 100 on a 2 gb droplet and a 8 gb droplet.
```
2 gb droplet
## search_time_table
Search time --- #640696794 micro seconds, #1500000 of requests ran, #0 skipped

8 gb droplet
## search_time_table
Search time --- #711256037 micro seconds, #1500000 of requests ran, #0 skipped
```
The perform almost the same, which should be the case - but makes sense considering that they both capped out at 120 mb, as shown in the schema above. 

As an experiment i tried lock pages in memory, but that ended up crashing instead. 



```
mmap'ed files in memory is loaded (and offloaded) by pages between all the levels of memory (the caches, RAM and swap).
```

```
Skal dette med? 

**Building Redblack tree on low resources.**

One of the biggest problems I encountered was I couldn't build the redblack tree from the whole dataset, but only on smaller datasets.
When I ran my many unit-test, the tree worked fine, but if I ran tests with many entries, the tree was constructed wrong. When building the redblack trees 
I check if the grandparent's child-pointer equals the nodes parent-pointer, these have to be equal otherwise there is something wrong with the family relation, and some nodes may have no reference to it, which makes the tree unreliable an pretty much worthless. 

My hypostasis is that since the redblack tree is the tree with the most reference, some of the references point to pages that may be offloaded by the memory map.
This can never happen in vanilla rust, because of its memory safety, but since accessing an element on a memory map requires the `unsafe keyword`- it cant guarantee what is going to happen with that piece of memory. 


In C memmap, mlock, mlockall are all in the same family of functions and you can use them together. In Rust, there is no such thing. 

I made a test where I ran the redblack tree build, and locked all nodes with mlock (the unsafe c function), and the program died after 1000-1100 nodes.

There is a type called `Pin<>`, where can pin memory in ram, I haven't managed to find a single place online where `Pin<>` and MmupMap is used/mentioned together, making me believe they were either not mean to be used together or no one has ever tried. An interesting experiment in the future would be to pin the nodes, using `pin<>`, while searching done the tree.


fn get_node_raw<'a>(mmap: &'a MmapMut, offset: usize) -> &'a mut Node {
    let byte_map = &mmap[offset..(offset+NODE_SIZE)];
    let number = unsafe { libc::mlock(byte_map.as_ptr() as *const c_void, byte_map.len()) };
    assert_eq!(number, 0);
    node_from_bytes(&byte_map)
}
```

____


## Caching Experiment

### Cache tests
Special cache miss-tests are performed to track how the cache may impact the performance of data structure. For testing the cache this I used Linux command `perf stat -e task-clock, cycles, instructions, cache-references, cache-misses [input]` on the droplets. Between each step they cache is cleared by using the command `sync; echo 3 > /proc/sys/vm/drop_caches`, to make sure we start from a cold cache and each test is not affected by the previous. The script for running the experiment can be found in appendix X

##### Expectation
In theory, the cache shouldn't matter if the data-set consists of an infinitely large amount of entries because the cache would be thrashed anyway - But on a more realistic scale (like in this project) this can become a factor when it comes to speed.

The immediate thought would be that the tree would benefit from this, since the nodes closer to the root would be read much more often than the rest of the tree, meaning that the data stored in the upper nodes can be retrieved from the cache. 

As mentioned a few times in the previous experiments, the BST makes better use of locality. This means that the BST should have fewer cache misses than the redblack table, because more nodes are stored on the same pages and children are closer to their parents. We will expect the percentage difference between the trees to increase the bigger the dataset, because the redblack trees nodes will be more an more spread out. 

For this implementation, it is difficult to isolate the cache-miss counting only to the searching. This is coursed by the fact that the searching-step (mentioned in testing XXX) includes _generating search input_, _the actual lookups_, and _looking up the payload in the `payload_map`_. 
Both the _generation search input_ and looking up the payload_ is exactly the same step for all three data models meaning they can be seen as a constant factor in these cache tests. This means that generating and looking in `payloads_table` should be stable for all 3 tests. For the test performed the payload size was set to `1 byte`, both make sure almost all access in the `payload_map` were cached. 

##### Results

All tests run with script found on Appendix X.

##### Dionysos

| Dionysos      | 1k            | 100k      | 10 mil    | 150 mil   |
| ------------- |--------------:| ---------:|----------:|----------:|
| BST           | 36.693 %      | 56.340 %  | 77.475 %  | 67.873 %  |
| Redblack      | 36.267 %      | 51.276 %  | 79.102 %  | 69.431 %  |
| Table         | 35.864 %      | 79.751 %  | 87.384 %  | 79.413 %  |


| 1gb mem       | 1k            | 100k      | 10 mil    | 150 mil           |
| ------------- |--------------:| ---------:|----------:|------------------:|
| BST           | 30.248 %      | 51.339 %  | 46.111 %  | 27.669 %          |
| Redblack      | 39.376 %      | 53.470 %  | 26.038 %  | 43.318 %          |
| Table         | 45.952 %      | 52.400 %  | 9.198 %   | 39.264 %          |

##### Discussion

Starting with Dionysos, as expected the cache the data is as expected. The table has the highest 

This is just as expected. Stable cache misses. Higher count for 1 gb machine. Tree having the lowest cache miss-rate.


As expected the table has the worst cache and the tree is best.


______


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

>Note: Compiler optimizations can be extremely hard to predict and understand, so I wont jump to any big conclusions based on this. But a interesting test to do. This test was done before the redblack tree implementation. 




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
* No reason to individually place each enty to the ip_table... I could just add them to an array in memory and then place that on disk
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


## Appendix A - Siteimprove Data
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


## Appendix B - Cache Script

```bash
set -e

cargo build --release --color=always

hn="$(hostname)"

printf "\n cache 1000 \n "

./run_benchmark_cache_build.sh 1000 && printf 1
./run_benchmark_cache_search.sh 1000 "cache/$hn/1000_1.txt" && printf 2
./run_benchmark_cache_search.sh 1000 "cache/$hn/1000_2.txt" && printf 3
./run_benchmark_cache_build.sh 1000 && printf 4
./run_benchmark_cache_search.sh 1000 "cache/$hn/1000_3.txt" && printf 5
./run_benchmark_cache_search.sh 1000 "cache/$hn/1000_4.txt" && printf 6

printf "\n cache 100000 \n "

./run_benchmark_cache_build.sh 100000 && printf 1
./run_benchmark_cache_search.sh 100000 "cache/$hn/100000_1.txt" && printf 2
./run_benchmark_cache_search.sh 100000 "cache/$hn/100000_2.txt" && printf 3
./run_benchmark_cache_build.sh 100000 && printf 4
./run_benchmark_cache_search.sh 100000 "cache/$hn/100000_3.txt" && printf 5
./run_benchmark_cache_search.sh 100000 "cache/$hn/100000_4.txt" && printf 6

printf "\n cache 10000000 \n "

./run_benchmark_cache_build.sh 10000000 && printf 1
./run_benchmark_cache_search.sh 10000000 "cache/$hn/10000000_1.txt" && printf 2
./run_benchmark_cache_search.sh 10000000 "cache/$hn/10000000_2.txt" && printf 3
./run_benchmark_cache_search.sh 10000000 "cache/$hn/10000000_3.txt" && printf 4

printf "\n cache 150000000 \n "

./run_benchmark_cache_build.sh 150000000 && printf 1
./run_benchmark_cache_search.sh 150000000 "cache/$hn/150000000_1.txt" && printf 2
./run_benchmark_cache_search.sh 150000000 "cache/$hn/150000000_2.txt" && printf 3
./run_benchmark_cache_search.sh 150000000 "cache/$hn/150000000_3.txt" && printf 4
```

```bash
set -e

run_program='./target/release/rust_map'
input_data_shuffled="input_data_shuffled.txt"

cargo build --release --color=always

$run_program --generate_data    --print_info                      --payload_size 1 -n $1 
&>/dev/null && printf - && sleep 0.5
$run_program --build_BST        --input_file $input_data_shuffled --payload_size 1 -n $1 
&>/dev/null && printf - && sleep 0.5
$run_program --build_redblack   --input_file $input_data_shuffled --payload_size 1 -n $1 
&>/dev/null && printf - && sleep 0.5
$run_program --build_table      --input_file $input_data_shuffled --payload_size 1 -n $1 
&>/dev/null && printf - && sleep 0.5
```

```bash
set -e

perf_cmd="perf stat -o $2 --append -e task-clock,cycles,instructions,cache-references,cache-misses"
run_program='./target/release/rust_map'
input_data_shuffled="input_data_shuffled.txt"

rm -f $2
hostname > $2

sync; echo 3 > /proc/sys/vm/drop_caches && 
$perf_cmd $run_program --search_BST       --input_file $input_data_shuffled --payload_size 1 -n $1 
&>/dev/null && printf _ && sleep 0.5
sync; echo 3 > /proc/sys/vm/drop_caches && 
$perf_cmd $run_program --search_redblack  --input_file $input_data_shuffled --payload_size 1 -n $1 
&>/dev/null && printf _ && sleep 0.5
sync; echo 3 > /proc/sys/vm/drop_caches && 
$perf_cmd $run_program --search_table     --input_file $input_data_shuffled --payload_size 1 -n $1 
&>/dev/null && printf _ && sleep 0.5
```

## Appendix C - Droplet

```
1gb25gb
    description: Computer
    product: Droplet
    vendor: DigitalOcean
    version: 20171212
    serial: 188818393
    width: 64 bits
    capabilities: smbios-2.4 dmi-2.4 vsyscall32
    configuration: boot=normal family=DigitalOcean_Droplet uuid=EC9DDD1B-6217-40D6-8DA6-B7532A4CDA4F
  *-core
       description: Motherboard
       physical id: 0
     *-cpu
          description: CPU
          product: Intel(R) Xeon(R) Gold 6140 CPU @ 2.30GHz
          vendor: Intel Corp.
          physical id: 401
          bus info: cpu@0
          slot: CPU 1
          size: 2GHz
          capacity: 2GHz
          width: 64 bits
          capabilities: fpu fpu_exception wp vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ss syscall nx pdpe1gb rdtscp x86-64 constant_tsc arch_perfmon rep_good nopl cpuid tsc_known_freq pni pclmulqdq vmx ssse3 fma cx16 pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand hypervisor lahf_lm abm 3dnowprefetch cpuid_fault invpcid_single pti ssbd ibrs ibpb tpr_shadow vnmi flexpriority ept vpid fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid mpx avx512f avx512dq rdseed adx smap clflushopt clwb avx512cd avx512bw avx512vl xsaveopt xsavec xgetbv1 pku ospke md_clear
     *-memory
          description: System Memory
          physical id: 1000
          size: 1GiB
          capacity: 1GiB
          capabilities: ecc
          configuration: errordetection=multi-bit-ecc
        *-bank
             description: DIMM RAM
             physical id: 0
             slot: DIMM 0
             size: 1GiB
             width: 64 bits
     *-scsi
          physical id: 1
          logical name: scsi2
        *-disk
             description: EXT4 volume
             product: Volume
             vendor: Linux
             physical id: 0.0.1
             bus info: scsi@2:0.0.1
             logical name: /dev/sda
             logical name: /mnt/volume_nyc1_01
             version: 1.0
             serial: 0947718c-25fb-4f54-a8dd-9f8ad600cf4f
             size: 100GiB
             capabilities: 5400rpm journaled extended_attributes large_files huge_files dir_nlink recover 64bit extents ext4 ext2 initialized
             configuration: ansiversion=5 created=2020-04-16 15:34:15 filesystem=ext4 lastmountpoint=/mnt/volume_nyc1_01 logicalsectorsize=512 modified=2020-04-17 19:12:00 mount.fstype=ext4 mount.options=rw,noatime,discard,data=ordered mounted=2020-04-17 19:12:00 sectorsize=512 state=mounted
            

```


## Apendix D - Program Help
```
Rust Memory Map 0.1.0
Dag Andersen <dagbjerreandersen@gmail.com>
Searching in memory mapped files

USAGE:
    rust_map [FLAGS] [OPTIONS]

FLAGS:
        --build_BST          Builds a BST for given input
        --build_redblack     Builds a Redblack Tree for given input
        --build_table        Builds a Table for given input
        --generate_data      Generates random entries instead of getting the input from a file
    -h, --help               Prints help information
        --print_info         Prints the setup for this run
        --search_BST         Searches down the BST with <<number_of_entries> / <gap_size>> number of entries
        --search_redblack    Searches down the Redblack Tree with <<number_of_entries> / <gap_size>> number of entries
        --search_table       Searches the Table with <<number_of_entries> / <gap_size>> number of entries
    -V, --version            Prints version information

OPTIONS:
    -g, --gap_size <gap_size>
            The number of entries it skips while selecting/collecting entries to search for

    -f, --input_file <input_file>                  The file for building the data-structure
    -n, --number_of_entries <number_of_entries>    The number of entries
    -p, --payload_size <payload_size>              The amount of bytes for each entry
```