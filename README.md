# rrpl

![GitHub Workflow Status](
https://img.shields.io/github/workflow/status/khrynczenko/rrpl/rust,%20cargo)

## What is **rrpl**

**rrpl (rust replace)** is a CLI application for replacing text matches inside
a file. It's CLI interface is meant to mimic other tool, i.e., **rpl** which
is available on debian distributions.

## How to use

```text
> rrpl --help
rrpl 0.1.0
Krzysztof Hrynczenko <jeniopy@gmail.com>

USAGE:
    rrpl [OPTIONS] <FROM> <TO> [FILE]...

ARGS:
    <FROM>
    <TO>
    <FILE>...

OPTIONS:
    -b, --backup         Rename original file to file~ before replacing
    -h, --help           Print help information
    -i, --ignore-case    Match case-insensitively
    -p, --prompt         Prompt confirmation before changing the file
    -q, --quiet          Disable logging to stdout/stderr
    -V, --version        Print version information
    -w, --whole-words    Match on word boundaries only
```

## How to build

`cargo build`
