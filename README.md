# cn
cn (from the word clone) is a fast, memory safe alternative to `cp`, the copy command from UNIX/Linux/BSD. 
It is also a lot more friendly than the original `cp` program by providing omissions to some flags
that should be turned on by default, better error messages.

You liked `cn` but you got the habit of always typing `cp`, alias `cp` as `cn`
and forgot that you're using `cn`, just think `cp` got better.

What? You want to use `cn` but your hard worked scripts may break because `cn` has those coolness,
guess what `cn` can mimic `cp` so you can use `cn` as a drop in replacement for `cp`

## Examples
### Copy one file into another directory
```
cn file1 somedir
```

Copies `file1` into `somedir`

### Copy multiple files into one directory
```
cn file1 file2 file3 somedir
```

Copies `file1` `file2` and `file3` into `somedir`

### Copy entire directory into another directory
```
cn dir1 dir2
```
Copies `dir1` into `dir2`

`cn` does not require the `-r` flag while copying a directory. Although the flag does exist to 
keep compatibility with `cp`

### Rename a file into another
``` sh
cn -m myfile changedfile
```

Renames `myfile` to `changedfile`.

`cn` supports moving files/folders instead of copying them

## Installation
- Grab the binary for the latest release
- Once downloaded, place the executable into one of the `$PATH` directories
To see your path directories, do the following
    - Open a terminal
    ` Run this command
    ```
    echo $PATH
    ```

## Compiling from source
`cm` is written in Rust. So you need the following requirements
- [Git](https://git-scm.org/downloads)
- [Rust](https://rust-lang.org/tools/install)
- Cargo (Automatically installed with Rust)
- A C linker (Only for Linux/UNIX platforms, usually one is already present)

``` sh
# Using SSH
git clone git@gitlab.com:arijit79/cn.git
# Or Using HTTPS
git clone https://gitlab.com/arijit79/cn.git

## Build using Cargo
## Remove `--release` if you want to have debug symbols
cargo build --release
```

## Contributing
Unless specifically stated otherwise, all contributions to `cn` are
under the MIT License. See [CONTRIBUTING](CONTRIBUTING.md)

## License
The project is open sourced under the MIT License. See [LICENSE](LICENSE)

## Author
Arijit Dey <arijid79@gmail.com>