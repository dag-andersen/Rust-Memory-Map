# Bachelor Report

# Intro
## Motivation
Containers
Speed is first priority 

## Si Rules/Priorities
```
- Language:             Rust.
- Dataset:              A set of IP ranges to firms.
                        No overlapping ranges. 
- Pre-processing-time:  Not important.
                        No new entries after first lookup.
- storage space:        Not important.
- Lookup-time:          Important.
```

# Si goals 
```
- Dataset:              10.mil entries
                        256b payload pr entry.
- Memory:               4gb
- Lookup-time:          p99 in 60ms.
```

## Input Cases
- Payload pr. range
- Distance between ranges
- number of ranges
- Range size
- IPv4 or IPv6
- mutable structure vs. dynamic insertion 

## Test Variables:
- ram 
- Space
- lookup time
- build time

Use profiler


# Data structures
## Trees

![Tree disk usage](../docs/images/bachelor-01.png)

## Tables

best and worst case:
2^32 /8    /1000/1000/1000 = 0.53 GB pr. bit 
bites/bytes/kilo/mega/giga

### sql

# Design

# Testing
## Setup
## Test Results

# Evaluation

# Conclusion  