# cn
cn (from the word clone) is a fast, memory safe alternative to `cp`, the copy command from UNIX/Linux/BSD. 
It is also a lot more friendly than the original `cp` program by providing omissions to some flags
that should be turned on by default, better error messages.

You liked `cn` but you got the habit of always typing `cp`, just alias `cp` as `cn`
and forgot that you're using `cn`, just think `cp` got better.

What? You want to use `cn` but your hard worked scripts may break because `cn` has those coolness,
guess what `cn` can mimic `cp` so you can use `cn` as a drop in replacement for `cp`

## Examples
**Copy one file into another directory**

```
cn file1 somedir
```

Copies `file1` into `somedir`

**Copy multiple files into one directory**

```
cn file1 file2 file3 somedir
```

Copies `file1` `file2` and `file3` into `somedir`

**Copy entire directory into another directory**

```
cn dir1 dir2
```
Copies `dir1` into `dir2`

`cn` does not require the `-r` flag while copying a directory. Although the flag does exist to 
keep compatibility with `cp`

The project does not complete somes of the above features. See the [1.0 Roadmap](https://github.com/arijit79/cn/issues/1)
