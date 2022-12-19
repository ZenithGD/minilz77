# MiniLZ77

Tiny implementation of the LZ77 compression algorithm, written in Rust
![Build Status](https://github.com/ZenithGD/minilz77/actions/workflows/rust.yml/badge.svg?branch=master&event=push)

---

## Task
The main task is to implement LZ77 in a memory efficient language. The algorithm
has to be evaluated against a collection of documents (i.e [project Gutenberg](https://www.gutenberg.org)). 
The performance should be compared to other compression tools, such as `gzip` or `bzip2`.

## Motivation
This is a university assignment, but it's also a personal project for learning to code in Rust. You can find the report [here](APD_prob4.pdf) (in Spanish)

---

## How to run

First ensure you've got a valid Rust installation on your machine (cargo and the Rust compiler). After that, run the main program with the following command:

```
cargo run
```

In order to execute all the tests, run
```
cargo test
```