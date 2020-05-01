## Search time Experiment
#### Expectation

From a purely theoretical point of view, we would assume that the table is the fastest, followed by the redblack tree, followed by the BST. The table should run in constant time because it only needs to do two lookups (one in the table and one in payload_map). Both trees should have a `O(log(n))` lookup time complexity, but I would expect the BST to be slower than the redblack tree because it is not balanced and therefore needs more key comparers to reach deeper nodes. 

I would expect Dionysos to be much faster than the 1 gb droplet, because it has more memory and should be able to hold more pages in memory before offloading them. Since Dionysos has 120 gb memory it should be able to have all pages loaded in memory for fast search. 

#### Results

The tests for this experiment have been run on both Dionysos and the 1gb droplet.
All tests are run with: `range: 10..18, payload_size: 50, gap: 10`.
The numbers in the table are the average lookup time pr. requested IP. All numbers are in microseconds.
These speed tests were run right after the building of the model without shutting down the program. This was done to maximize speed by having as many pages already in memory as possible. The shell script for producing the results can be found in appendix XX.

**Dionysos**
| model         | 1k      | 100k      | 10 mil    | 150 mil    |
| :------------ |--------:|----------:|----------:|-----------:|
| BST           | 3       | 0.7       | 2.1       | 3.74       |
| Redblack      | 3       | 0.9       | 1.7       | 3.15       |
| Table         | 6       | 3.6       | 3.4       | 1.001      |

**1gb Droplet**
| model         | 1k        | 100k    | 10 mil*    | 150 mil*  |
| :------------ |----------:|--------:|----------:|----------:| 
| BST           | 1.5       | 0.8     | 8.8       | 1402      |
| Redblack      | 1.5       | 0.8     | 7.7       | xxx       |
| Table         | 5.5       | 4.5     | 2103      | xxx       |

> `*` The 1gb25gb can't build these data sizes (as explained in the next experiment) meaning that model has to be built on Dionysos and copied to the 1gb droplet. This means that the cache was cold, and no pages were loaded into memory before the speed test was run. 

#### Discussion

**Dionysos**

Starting with Dionysos, we can see that the table is the fastest on the 150mil-data-set, which was what we expected. 

One thing that is interesting is that for both machines for both trees follow along well. One could expect that the redblack tree would have double the speed of the BST, because of the lower height, but this is not the case. This can be due to better usage of locality by the BST. When searching down the BST, you search in one direction through the file, and the nodes are often close to each other in the file.
The redblack tree may have fewer key-comparisons/node-accesses, but nodes are more randomly distributed in the file because of rebalancing. 
<img src="../docs/images/bachelor-10.png" alt="drawing" width="800"/>
E.g in the BST the root is always at node offset 0, and all nodes in the tree will always be to the right from their parent on the same page or the following pages, while the redblack tree's root could, in theory, be in the middle of the file, and its children could be far apart. This means that the redblack tree has a bigger chance of page faults.
Another reason the BST and redblack tree are perform close to each other, could be that the redblack nodes are bigger and therefore fewer nodes can be stored on the same page. This means that the redblack tree will also get more pagefaults because it can have less nodes loaded in at the same time. This also has impact on the performance, but this should be minimal, though.

For smaller data sizes the table is worse than the trees. Even though the table has fewer accesses, I would expect it to be relatively worse if the ranges are spread out (bigger space between them), since the entries would be placed further apart from each other in the file and therefore the table won't load multiple ranges from the file. The requests are random, and all the ranges are close to evenly distributed over the whole ipv4-range. This means, that all entries are equally likely, and that there is no pattern in which ips are accessed. The system has no way of guessing what to load next. The trees, on the other hand, would perform better on smaller data sizes, because all nodes are stored right next to each other on the file and will, therefore, be loaded on the same page and will be cached. This can answer why table is much worse than the trees on smaller more-spread-out data. 

For both machines we see a drop from 1k to 100k. The reason for the high lookup average of 1k entries, is probably the overhead of print-lines to standard-out. I implemented (not using external library) a simple progress bar, that prints a dash `-` every time it has processed 1% of the entries (both for searching and building), and flushing immediacy (because I had an issue of it not being up to date). This has close to no impact, when there are 150 million entries, but it is a lot when there are 1k entries, and it has to stop and flush every 10th entry. This should probably have been a feature you could toggle as a flag.  

It is also important to note that when the data set is smaller, the statistical power is lower. When we do 1000 entries and 10% lookup, only 100 requests are actually run which is a much lower sample size than 1.500.000, which we have with full 150 mil entries . We need to remember this since the variation in the raw test data is more spread out. We just have to keep this in mind when reading the results.


```
Note til philippe: Jeg ved ikke rigtig hvorfor
1gb-droplet bliver vildt meget langsommere på
dataset der er 150million, eftersom vi kan se i
experimentet med memory usage, at begge maskiner har et limit på 64mb memory. 
```
_____


## Build time

#### Expectation

For this project, there were no system requirements for the machine that should build the data structure. The only requirement was that it had to be built in less than a day. This experiment has been built on Dionysos.

**Table**
The table should primarily be limited by write speed. It should insert each entry in linear time `O(r)`, where r is the range of entry, since the runtime doesn’t grow with the number of entries, but it has to repeatedly store a pointer to the map table, for each ip in the range. In these experiments, we have a range ranging between 10 and 18, so the table must insert 10 to 18 pointers per entry. This means the insertion complexity can be seen as constant `O(1)`. I would expect the table to be best on a large data set, because of the constant insertion time compared to the trees' log(n) insertion time. 

**BST**
The BST only writes twice to memory. One for placing the node/struct in the map and one re-directing its parent’s pointer. But the slowing part is that the algorithm must search down the tree every time. Leaving it with a `O(Log(n))` insertion time. I would expect the BST to be the fastest on smaller data sets because all nodes are stored next to each other in the file (great use of locality) and most nodes would probably be loaded in memory.

**Redblack tree**
The redblack tree is more difficult to predict because it also has to balance the tree. Balancing the tree requires references to multiple nodes above the newly inserted node, and potentially many more if a rotation is needed. In this implementation, it does not save a reference to the nodes it encounters down the search, so when balancing it has to re-access/request the node in the memory map. This should not be an issue when the balancing is only one rotation, but this will be an increasing problem when the tree grows, and bigger rotations happen.
Furthermore, as described in the discussion on the previous experiment, we must remember that the nodes in the redblack tree are stored more spread out compared to the BST, with less use of locality. Overall, I would expect the redblack tree to be the slowest in all cases, because of the huge amount of node accesses.

##### Results
This experiment is done by running the shell script found in appendix XXX on dionysos.
All numbers are in microseconds. 

**Build time per data structure**
| Model         | 1k        | 100k      | 10 mil      | 150 mil           |
| --------------|----------:|----------:|-------------:|------------------:|
| BST           | 3874      | 204523    | 29716768     | 857455857         |  
| Redblack      | 62079     | 6156475   | 655914608    | 11897741304       |
| Table         | 23758     | 1381446   | 144225979    | 930878211         |

**Average insertion time pr. entry**
| Model         | 1k        | 100k      | 10 mil      | 150 mil            |
| --------------|----------:|----------:|-------------:|-------------------:|
| BST           | 3.8       | 2.0        | 3.0          | 4.5                |  
| Redblack      | 62.1      | 59.1       | 65.6         | 81.3               |
| Table         | 23.8      | 13.8       | 14.4         | 5.8                |

##### Discussion

The first thing we notice is that the redblack tree is the worst performing data structure, just as we had expected.

As mentioned in the previous experiment, building the data structures also makes use of the progress bar made from flushed printlines. Again, this has a meaningful overhead on small data sizes, which is probably why we see a decrease from 1k to 100k in average insertion time for all data structures. More on this can be found in the previous experiment.

One interesting point is that the percentage difference between the BST and the table decreases the bigger the data size (besides the jump from 1k to 100k, as described just above). The percentage difference between them goes from 16% (3.8/23.8) -> 14% (2.0/13.8) -> 20%(3.0/14.4) -> 70%(4.5/5.8), which indicates that the table will catch up to the BST the bigger the data set. This is kind of expected considering constant time scales better than logarithmic. For bigger data sets the table may have been a better option, but not for this data size.

This experiment was also tried on the 1gb machine, but with less success. When building the redblack tree with 150 mil data size, I stopped the 1gb machine after 10 hours after seeing that it had only inserted ~15% of the test data, meaning it would not be able to finish in time, building at that rate. It was never a requirement to be able to build the data structure on a low resource machine, but it could be interesting to do an experiment on how long it would take on different data sizes. 

> Note: As a very small experiment, I tried to place a full array of pointers for each entry in the table instead of placing each pointer individually. The implementation was to create an array with the length of the range and then loop over the array placing all the pointers in the array and then copying the whole array into the memory map. This did not make a difference, which also would be expected, since writing to an array in memory would be the same as writing to the map in memory. 

The longest build time is the redblack tree with an average of 3.4 hours, which is way below the one-day limit. Based on these numbers we can conclude that all the data structures are a viable option for solving this issue if run on a machine with specs like Dionysos. This build time will increase, the slower the machine is. In the future, it could be interesting to experiment on how low the specs can be and still be able to build in 24 hours. 

The best performing data structure seems to be the BST - This is probably a combination of only 2 writes to memory and good use of locally.

From this experiment, we can conclude that the redblack tree is the least scalable solution, with this specific implementation. 

## Memory Usage Experiment
#### Expectation

Memory is an important factor when working with memory mapped files. The kernel should keep loading in pages as long as there is free memory left - and only start offloading pages when the memory is close to full.
A page is 4kb, which means that if we in a memory mapped file access data at least once every 4kb, we will load all pages in the whole file. So, if we build the full table with a padding/gap of 10-18 bytes, we will have to load in all 2^32*64 bit (34.4 GB). This is of course not possible on a 1 GB memory machine, so the system must start offloading pages again. The victim page is chosen by some algorithm in the cache system. If it keeps loading and offloading the same page it is known as thrashing, since it keeps asking for something and throwing it away even though it needs it again in the near future. This is a challenge when IP-requests are random because it cannot predict what to keep in memory if there is no pattern.

Overall, the expectation is that the machine will keep loading in pages as long as there is space left on its disk.

#### Method
To test this, I built a table on four different machines, with 150 entries, and check memory usage, before and during the building of the table. Here we would expect it to use all memory available and after that loading and offloading pages (paging). To track the memory usage I used the Linux command `free`, that prints the current memory usage before and while the program is running. 

#### Results

**Calling `free` command**

| Machine       | Idle      | Running       | Difference    |
| ------------- |----------:|--------------:|--------------:|
| 1gb           | 97mb      | 160mb         | 63 mb         |
| 2gb           | 85mb      | 203mb         | 122 mb        |
| 8gb           | 142mb     | 271mb         | 129 mb        |
| Dionysos      | 9681mb    | 9612mb        | 69 mb         |

The free command was run many times during the build of the data structure, and already after building 10% of the input data, the memory usage stopped increasing and had a stable memory usage the rest of the building process.

#### Discussion

The fact that the memory usage did not keep growing means that there is a limit somewhere in the system, since it starts paging before all memory is used. This can also be seen by looking at the digital oceans monitoring tool. Looking at the 1gb machine over the time period of one week, where all tests in this project were performed, it never got over 33% memory usage. 
<img src="../docs/images/ram_usage_1gb2.png" alt="drawing" width="400"/>

In the schema above the graph shows a pattern for the differences being a multiple of ~64mb. To figure out why it limits itself, further testing is needed. 
This also means that the lookup and build speeds could potentially be much faster, if the memory map could use its full memory, which was the goal from the very beginning. 


As mentioned, building the table with 150 million entries would use 2^32*64 bits (34.4 GB), and therefore a machine with more memory would be faster than a low memory resource machine. But this was not the case when running a search with a gap_size of 100 on a 2 gb droplet and an 8 gb droplet.
```
2 gb droplet
## search_time_table
Search time --- #640696794 micro seconds, #1500000 of requests ran, #0 skipped

8 gb droplet
## search_time_table
Search time --- #711256037 micro seconds, #1500000 of requests ran, #0 skipped
```
They perform almost the same, which should not be the case - but makes sense considering that they both capped out at 120 mb, as shown in the schema above. 

The cap at a multiple of ~64 is maybe caused by the memory map not loading pages into memory but instead in the disk buffer/cache buffer, which is an embedded hard disk which functions as a buffer between the disk and the rest of the computer. These buffers are often between 8 and 256 MiB. It is possible that this is the primary bottleneck for performance. In other words, the memory mapped file load and offload pages in the buffer instead of main memory. 

> As an experiment I tried to lock pages in memory using c's `mlock`-function, but that ended up crashing instead. Probably because it ran out of space in the buffer.

____

## Caching Experiment

A Cache-miss experiment is performed to track how the cache may impact the performance of data structure. For testing the cache, I used the Linux command `perf stat -e task-clock, cycles, instructions, cache-references, cache-misses [input]` on the droplets. Between each step the cache is cleared by using the command `sync; echo 3 > /proc/sys/vm/drop_caches`, to make sure we start from a cold cache and each test is not affected by the previous. The script for running the experiment can be found in appendix X.

##### Expectation
In theory, the cache should not matter if the data set consists of an infinitely large amount of entries because the cache would be thrashed anyway. But on a more realistic scale (like in this project) this can become a factor when it comes to speed.

The immediate thought would be that the trees would benefit from this, since the nodes closer to the root would be read much more often than the rest of the tree, meaning that the data stored in the upper nodes can be retrieved from the cache. 

As mentioned, a few times in the previous experiments, the BST makes better use of locality. This means that the BST should have fewer cache misses than the redblack table, because more nodes are stored on the same pages and children are closer to their parents. We will expect the percentage difference between the trees to increase the bigger the data set, because the redblack trees nodes will be more and more spread out. 

For this implementation, it is difficult to isolate the cache-miss counting only to the searching. This is coursed by the fact that the searching-step (mentioned in testing XXX) includes _generating search input_, _the actual lookups_, and _looking up the payload in the `payload_map`_. 
Both the _generation search input_ and _looking up the payload_ is the same step for all three data three data structures meaning they can be seen as a constant factor in the experiment. This means that generating and looking in `payloads_table` should be stable for all results. This experiment was performed with a payload size of `1 byte` to make sure that almost all access in the `payload_map` were cached. 

##### Results

All tests run with script can be found in Appendix X.

**Dionysos**
| Model         | 1k            | 100k      | 10 mil    | 150 mil   |
| ------------- |--------------:| ---------:|----------:|----------:|
| BST           | 36.693 %      | 56.340 %  | 77.475 %  | 67.873 %  |
| Redblack      | 36.267 %      | 51.276 %  | 79.102 %  | 69.431 %  |
| Table         | 35.864 %      | 79.751 %  | 87.384 %  | 79.413 %  |

**1gb droplet**
| Model         | 1k            | 100k      | 10 mil    | 150 mil           |
| ------------- |--------------:| ---------:|----------:|------------------:|
| BST           | 30.248 %      | 51.339 %  | 46.111 %  | 27.669 %          |
| Redblack      | 39.376 %      | 53.470 %  | 26.038 %  | 43.318 %          |
| Table         | 45.952 %      | 52.400 %  | 9.198 %   | 39.264 %          |

##### Discussion

Starting with Dionysos, again the trees follow along well. But for 10 mil and 150 mil the redblack has more cache misses compared to the BST, again this can be explained by the BST's better use of locality mentioned in both experiments above. On the other hand for 1k ad 100k the redblack tree has less cache-misses than the BST. This may be caused by the fact that there is less nodes and therefore a higher percentage of the nodes is already loaded and therefore a lower tree height may be more important than locality. For 100k, 10mil, and 150 mil the table has the highest cache miss, which is expected because all requests are random, so there is no pattern and i doest not use a concept like locality to "guess" what to load next. 

For the droplet the data is more inconstant. This may be because droplet is a VM stored on digital ocean data center, where all their customers have their own hosted vm on the same machines. This can cause inconsistencies in the resources provided to our droplet. 

______