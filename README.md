# space_hogger_logger

## Description

space_hogger_logger is a command-line tool that is similar to
[Daisy Disk](https://daisydiskapp.com).

space_hogger_logger, or "shlogger", recursively runs through directories
specified in the `default_paths.txt` and produces a list of all the files,
sorted by file size, within those directories.  Files at the bottom of the list
are the largest and are considered space hoggers.

Additional user-defined paths can be included in a file with a path of
`Library/Application Support/The Lyons' Den Labs/shlogger_extra_paths.txt`.

**Note**: space_hogger_logger only works on macOS currently.
