# Table of Contents
- [Overview](#Overview)
	-  [Reports](#Reports)
- [Installation and Setup](#installation-and-setup)
- [Usage](#Usage) 
## Overview
This repository contains benches for [criterion.rs](https://github.com/criterion-rs/criterion.rs) along with the programs used for benchmarking. The goal of this was to get a feel for the library while also applying it to a program we care about.
#### Reports
Since the benchmarking may take a while, we have pre-generated reports for each of the programs for convenience. This will give a feel of what they might look like when generating the reports. To view all of these reports, go to [this](./reports/report) directory and view index.html in your browser. For generating the reports, continue to the [usage](#Usage) section
## Installation and Setup

This assumes Rust has already been installed. If it has not, install it [here](https://rust-lang.org/tools/install/)

Valgrind must also be installed. On debian based distributions, this can be installed with

```bash
apt install valgrind
```

Current releases of valgrind can also be done [here](https://valgrind.org/downloads/current.html)

Gungraun needs its benchmark runner to run benchmarks. To get this, install the following globally with cargo using:

```bash
cargo install --version "0.17.0" gungraun-runner
```

Now all that needs to be done is to clone the repository and go into the top level directory

```bash
git clone https://github.com/JFielding7/SoftwareTestingCriterionProject.git
cd SoftwareTestingCriterionProject
```

Now the benchmarks are ready to be ran!
## Usage


All benches can be ran with:

```bash
cargo bench
```

It is possible to run individual files, groups and bench functions
For example, running the bench functions for the sorts file would look like

```bash
cargo bench --bench "sorts"
```

Here is an example of running the group "sort_random"

```bash
cargo bench -- "sort_random"
```

Here is an example of running a bench function from a group. This runs the `stable` bench function under group `sort_random`

```bash
cargo bench -- "sort_random/stable"
```

It is also possible to use regex to pattern match benches
For example:

```bash
cargo bench -- "sort_random/(stable|unstable)"
```

