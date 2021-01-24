# Smerge
Easy to use cross-platform command line tool for creating soft symbolic links

## How to use
- Run `cargo build --release` in the root directory of the project, wait for binary to build
- Put built binary in `PATH`
Now you can run smerge to create symbolic links for a directory from another

## Example
Imagine you want to copy some files from `dir1` to `dir2`, and you know that either you won't change this files, or this copying may take a lot of time. The solution will be to create symbolic links to those file, but linking many files may be time consuming either, so with this tool you easily can link many files from one directory to another in case below you may just run
```sh
smerge path/to/dir1 path/to/dir2
```
| *dir1 *
 &emsp;  |- *innerDir*: 25gb
 &emsp;  |- *file*: 13 gb
| *dir2* # must exists when executing tool
 &emsp;  |- *innerDir*: ~0 gb # linked
 &emsp;  |- *file*: ~0 gb # linked

In case your directories partially match you may add flag `-r`
```sh
smerge -r path/to/dir1 path/to/dir2
```
| *dir1*
 &emsp;  |- *innerDir*
 &emsp; &emsp;  |- *existing file*
 &emsp; &emsp;  |- ...
 &emsp;  |- *file*: 13 gb
| *dir2* # must exists when executing tool
 &emsp;  |- *innerDir*: ~0 gb # will be linked
 &emsp; &emsp;  |- *existing file* # will be skipped
 &emsp; &emsp;  |- ... # will be linked
 &emsp;  |- *file*: ~0 gb # will be linked
