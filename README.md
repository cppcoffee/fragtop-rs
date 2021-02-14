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

## Example

```shell
$ ./target/debug/fragtop-rs -p '/var/log/**/*'
In progress: /var/log/yum.log-20210101
Scan total file: 657
/var/log/access.log                               266
/var/log/wtmp                                     39
/var/log/messages-20210131                        22
/var/log/messages-20210207                        21
/var/log/messages-20210117                        20
/var/log/audit/audit.log.2                        20
/var/log/audit/audit.log.4                        19
/var/log/messages-20210124                        18
/var/log/audit/audit.log.1                        18
/var/log/nginx/access.log                         17
/var/log/audit/audit.log.3                        17
/var/log/cron-20210207                            14
/var/log/cron-20210131                            14
/var/log/cron-20210124                            14
/var/log/cron-20210117                            14
/var/log/messages                                 13
/var/log/audit/audit.log                          13
/var/log/yum.log-20200511                         10
/var/log/tuned/tuned.log                          10
/var/log/grubby                                   8
```

## Reference

[https://www.kernel.org/doc/Documentation/filesystems/fiemap.txt](https://www.kernel.org/doc/Documentation/filesystems/fiemap.txt)

[https://man7.org/linux/man-pages/man8/filefrag.8.html](https://man7.org/linux/man-pages/man8/filefrag.8.html)

[https://lib.rs/crates/fiemap](https://lib.rs/crates/fiemap)

