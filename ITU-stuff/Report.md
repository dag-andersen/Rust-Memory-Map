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
- Dataset:              150.mil ipv4
                        35.mil  ipv6
                        256bytes payload pr entry.
- Memory:               4gb
- Lookup-time:          p99 in 60ms.`
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
```

ipv6 - solution E
30.000.000*232 / 8 /1000/1000/1000 = 0.87gb 

2^32/256

#table2
30.000.000 * 256/1000/1000/1000 = ~7.7gb
```


## Tables


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
## Setup
## Test Results

# Evaluation

# Conclusion  


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
