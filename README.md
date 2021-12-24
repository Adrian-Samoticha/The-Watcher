# The-Watcher
Utility for watching files or directories and running commands when changes are detected.

<br>

## Help output
```
USAGE:
    the-watcher [FLAGS] [OPTIONS] <PATH> <COMMAND>

FLAGS:
    -h, --help       Prints help information
    -q, --quiet      Disables output to stdout
    -V, --version    Prints version information

OPTIONS:
    -d, --delay <INT>    Sets the number of milliseconds to wait before executing a command when a change is detected
                         [default: 150]

ARGS:
    <PATH>       The path of the file or directory to watch
    <COMMAND>    The command to be executed when a file or directory change is detected
```

<br>

## Examples
```sh
the-watcher /path/to/file.txt ""
```
Watch a single file. No command is executed, but detected changes are printed to the console.

<br>

```sh
the-watcher -d 100 /path/to/file.txt "echo 'File changed'"
```
Watch a single file and echo `'File changed'` when it is changed (after a 100 ms delay).

<br>

```sh
the-watcher -q -d 500 /path/to/directory "cd /path/to/directory | ls . | pbcopy"
```
Quietly watch an entire directory, `cd` into it and copy the output of `ls .` to the clipboard (using the macOS-only `pbcopy` command) when a change is detected (after a 500 ms delay).
