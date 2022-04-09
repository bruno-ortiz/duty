# Duty

[![Rust](https://github.com/bruno-ortiz/duty/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/bruno-ortiz/duty/actions/workflows/rust.yml)

Simple program to generate a CSV for filling spots in an on-call schedule

## Install

Clone this dir and run:

`cargo install --path .`

This will build and install the executable to your `~/.cargo/bin` path.

# Usage

First you can always do: `duty --help` to see all available options.

Basic example:

`duty -s 2022-04-20 -p Lana -p Bruno -p Xose -p Leticia -p Moana`

Will print:

```
2022-04-20,Lana,Wednesday 19:00:00.0,Thursday 9:00:00.0,14,false
2022-04-21,Bruno,Thursday 19:00:00.0,Friday 9:00:00.0,14,false
2022-04-22,Xose,Friday 19:00:00.0,Friday 23:59:00.0,5,false
2022-04-23,Leticia,Saturday 0:00:00.0,Saturday 23:59:00.0,24,true
2022-04-24,Moana,Sunday 0:00:00.0,Sunday 23:59:00.0,24,true
2022-04-25,Xose,Monday 0:00:00.0,Monday 9:00:00.0,9,false
2022-04-25,Lana,Monday 19:00:00.0,Tuesday 9:00:00.0,14,false
2022-04-26,Bruno,Tuesday 19:00:00.0,Wednesday 9:00:00.0,14,false
2022-04-27,Xose,Wednesday 19:00:00.0,Thursday 9:00:00.0,14,false
2022-04-28,Leticia,Thursday 19:00:00.0,Friday 9:00:00.0,14,false
2022-04-29,Moana,Friday 19:00:00.0,Friday 23:59:00.0,5,false
2022-04-30,Lana,Saturday 0:00:00.0,Saturday 23:59:00.0,24,true
```

## TODO:

* Describe the rules on README;
* Consider holidays when applying rules;