# Bachelor - Designing/implementing/evaluating an external map in Rust on modern hardware ![Rust](https://github.com/DagBAndersen/Rust-Memory-Map/workflows/Rust/badge.svg)

## The Report
[Report PDF edition](docs/Bachelor_Report.pdf)


## The program
### Compiling
Run `cargo build --release` in the root of the repo to compile the program.
The executable binary file will by default be located in `/target/release/`.

### Testing

Run `cargo test --release -- --test-threads 1` in the root of the repo.
Running the tests require a minimum of 40GB free disk space. 

### Running the program

This version of the program can either generate data itself or read it from a file. What the program is supposed to do is specified with _flags_ and _options_ through the command-line arguments. The full list of flags and options available can be found i next below this section.

Here we have some examples of how it works:
* Building the Red-black Tree from a file and searching for a specific IP:
 `./rust_map --build_redblack --search_redblack --input_file MyFile.txt --specific_ip "160.5.211.97"`

* Searching in both BST and Red-black Tree for a specific IP on already built data structures:
`./rust_map --search_BST --search_redblack --specific_ip "160.5.211.97"`

* To generate a data set of 100000 entries, building a table, and search through 2% (1/50) of the data set:
`./rust_map --generate_data --build_redblack --search_redblack -n
 100000 --gap_size 50`

Some flags and options are an invalid combination. The program will tell you what is wrong and help you provide the right input. Here we have two examples of an invalid input:

* `./rust_map --build_redblack` where you do not specify an input file (`--input_file`) and do not generate a new dataset (`--generate_data`). This is an invalid combination, because then the program does not have anything to build off.

* `./target/release/rust_map --specific_ip "160.5.211.97"`, where you tell it to search for that specific IP, but does not tell it which data structure to search in.


### _Flags_ and _Options_
```
Rust Memory Map 1.0.0
Dag Andersen <daga@itu.dk>

USAGE:
    rust_map [FLAGS] [OPTIONS]

FLAGS:
    -b, --build_BST          Builds a BST for given input
    -r, --build_redblack     Builds a Redblack Tree for given input
    -t, --build_table        Builds a Table for given input
    -G, --generate_data      Generates random entries instead of getting the input from a file
    -h, --help               Prints help information
        --print_info         Prints the setup for this run
    -B, --search_BST         Searches down the BST with <number_of_entries / gap_size> number of entries
    -R, --search_redblack    Searches down the Redblack Tree with <number_of_entries / gap_size> number of entries
    -T, --search_table       Searches the Table with <number_of_entries / gap_size> number of entries
    -V, --version            Prints version information

OPTIONS:
    -g, --gap_size <gap_size>
            The number of entries it skips while selecting/collecting which IPs to search for

    -i, --input_file <input_file>                  The file for building the data structure
    -n, --number_of_entries <number_of_entries>    The number of entries
    -p, --payload_size <payload_size>              The number of bytes for each entry
    -s, --specific_ip <specific_ip>                The specific IP you want to search for
```



## Experiments
The raw data from the 4 experiments can be found here: [Experiments](Experiments)

### Cache Experiment
To reproduce the results of [_Experiment #2_](Experiments/Experiment_2) - run the this [_shell script_](run_benchmark_cache_loop.sh) in the root of the repo.

### Lookup and build Experiment
To reproduce the results of [_Experiment #3_ and _Experiment #4_](Experiments/Experiment_2) - run the this [_shell script_](run_benchmark_build_and_search.sh) in the root of the repo.