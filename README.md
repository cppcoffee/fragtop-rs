# fragtop-rs

## Introduction

Similar to the top tool, use glob to traverse the file and save the number of fragments, sorted from largest to smallest and output.

## Usage


```shell
$ ./fragtop-rs -h
fragtop-rs 0.1.0
cppcoffee <cppcoffee@gmail.com>

USAGE:
    fragtop-rs [OPTIONS] -p <path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p <path>         Set the glob file path
    -n <top-n>        Top fragment file [default: 20]
```

## Quick Start

View all log files in the /var/log directory and sort them according to the number of fragments, the default output is top 20.

```shell
./fragtop-rs -p '/var/log/**/*'
```

View the jpg file fragments in the home directory top 10.

```shell
./fragtop-rs -p '/home/xyz/*.jpg' -n 10
```

## Reference

[https://www.kernel.org/doc/Documentation/filesystems/fiemap.txt](https://www.kernel.org/doc/Documentation/filesystems/fiemap.txt)
[https://man7.org/linux/man-pages/man8/filefrag.8.html](https://man7.org/linux/man-pages/man8/filefrag.8.html)
[https://lib.rs/crates/fiemap](https://lib.rs/crates/fiemap)


