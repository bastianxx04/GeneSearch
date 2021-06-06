# GeneSearch

GeneSearch is a [something] for matching patterns to genomes

## Installing rust

To run the program you need to install rust. A guide for doing this can be found [here][rust-install].

## Running the program
The program consists of several branches with different versions of the code, mainly:

- main 
- otable-noskips
- sais-branchlessinduce
- [deprecated] otable-nosentinelrow 
- [deprecated] otable-nosentinelrow-branchlessconstruction  

Main has skips implemented. otable-noskips does not implement skips, while sais-branchlessinduce has some branchless features, but is otherwise equivalent to main in functionality.

### File structure

To use file for reads and genomes, they need to be placed correctly in the folder structure of the program. Genomes are placed in the folder GeneSearch/resources/genomes and need to be in FASTA format with an .fa file extension. Reads are placed in GeneSearch/resources/reads and need to be in FASTQ format with an .fq file extension. The folder structure can be seen below.
```
GeneSearch
└───resources
    └───genomes
    │   │   genome-file.fa 
    └───reads
        │   reads-file.fq
```

The code can be run either directly, or by building an optimized executable.

To run the code directly use:
`cargo run`

To build an optimized executable use:
`cargo build --release` with `--quiet` an optional flag to suppress compiler output.

### Console arguments

The application takes arguments when running it. For running it directly these are placed after run, and this is the same for the executable which is located in the folder `target/release/`.

The general structure of the command line arguments is:

| Argument | Value |
| ------ | ------ |
| type | This can be either "sais", "skew", "otable", "approx", "exact", "exact-bwt", "exact-binary" "naive-sa" |
| genome | The name of the genome file to do the operation the type parameter on |
| reads | The reads file to be used if the type is a search algorithm |
| iterations | How many times should the calculation be done. Mainly for testing purposes. |
| spacing | The O-table spacing to be used. Only relevant for branches with spacing. |
| edits | Number of edits. Only relevant for type "approx" |

For type, the options "skew", "exact-bwt", and "exact-binary" are only available on the main branch. For the other branches "exact" will suffice for exact search.
 
The order of the arguments is as follows. If an argument is not relevant to the operation being executed it can simply be omitted from the list.
`{type} {genome} {reads} {iterations} {spacing} {edits} --no-output`
The `--no-output` flag is optional, and if set will only print the runtime of the operation.

## Examples
Running approximate search with genome file HG38-1000000.fa and reads from file reads-100-10-0.fq for one iteration with one edit allowed on branch otable-noskips. This prints the result to the console.
`cargo run approx HG38-1000000 reads-100-10-0 1 1`

Constructing an O-table from HG38-1000000.fa 5 times with skips set to 2, and output the average runtime for constructing the O-table in nanoseconds. On mainbranch .
`cargo build --release --quiet` and
`target/release/gene_search.exe otable HG38-1000000 5 2 --no-output`

[//]: # (These are reference links used in the body of this note)
   [rust-install]: <https://www.rust-lang.org/learn/get-started>
>
