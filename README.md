# pgn
[![crates.io](https://img.shields.io/crates/v/pgn.svg)](https://crates.io/crates/pgn)
command line pass word generator in rust.

# instalation
```shell
cargo install pgn
```

# usage
```
pgn - Command line password generator 0.1.1
Piotr `xeeny` Dudziński

USAGE:
    pgn [FLAGS] [OPTIONS]

FLAGS:
    -A, --alfa       All laters lower and upper case.
    -a, --all        All predefiend characters.
    -h, --help       Prints help information
    -L, --lower      All laters lower case.
    -N, --numbers    All of numbers
    -S, --space      Space charakter.
    -s, --special    Special charakters like `!`.
    -u, --upper      All laters upper case.
    -V, --version    Prints version information

OPTIONS:
    -c, --chars <chars>      Which addional chars should be in password.
    -n, --count <count>      How many posword generate. [default: 1]
    -l, --length <length>    How long posword generate. [default: 10]
```
## example
```shell
$ pgn -a -l 20
&Sp(Q8X*qu<.Ai4\%Mdj
```