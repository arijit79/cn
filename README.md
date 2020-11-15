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

### Symlink a file or directory into another
``` sh
cn -s myfile changedfile
```

Makes a symbolic link of `myfile` to `changedfile`.

One interesting feature `cn` provides that `cp` dosen't have is where relative paths 
can be used for files which are in same direct ancestor, irrespective of `cn`'s
current directory

Let's take a example
if we run
``` sh
cp -s somedir/file somedir/file2
```
`cp` will complain this, since `cp` must be present in `somedir`
```
cp: somedir/file: can make relative symbolic links only in current directory
```

While `cn` has absolutely no problems doing this  
**Though there is one caveot to this, the source and the destination must have the same direct ancestor**

### Hard link a file or directory into another
``` sh
cn -l myfile hardlinked-file
```

Makes a hard link of `myfile` to `hardlinked-file`.

**Only files could be hard linked. This is not an implementation restriction rather OSs don't support it**

**Only one file could be hard linked at once, and its destination file name should also be present. This restriction will be removed in future releases**
Meaning, you can't run these
``` sh
cn -l file somedir/ # Error
```

``` sh
cn -l file1 file2 somedir/ # Error
```

## Installation
- Grab the binary for the latest release
- Once downloaded, place the executable into one of the `$PATH` directories
To see your path directories, do the following
    - Open a terminal
    - Run this command
    ```
    echo $PATH
    ```

## Compiling from source
`cn` is written in Rust. So you need the following requirements
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