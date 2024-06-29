# addnew: A Rust Implementation of `tee -a`

## Overview
------------

`addnew` is a Rust command-line tool that functions like `tee -a`, writing input from a file or standard input to one or more output files, while skipping duplicate lines.

## Usage
-----
`addnew [ -o output_file ] input_file`


## Options

---------

* `-o output_file`: specifies the output file to write to. If omitted, output will be written to standard output.

* `input_file`: the input file to read from.


## Example

-------
`$ addnew -o output.txt input.txt`


This will write the contents of `input.txt` to `output.txt`, skipping duplicate lines.


## Features

---------


* Efficient duplicate detection using a `HashSet`

* Polymorphic writing to either a file or standard output

* Error handling using `std::error::Error`

## Intallation 

-----------------------

`cargo install --git https://github.com/ahmedxkhalid/addnew/`

