# space_hogger_logger

## Description

space_hogger_logger is a command-line tool that is similar to
[Daisy Disk](https://daisydiskapp.com).

space_hogger_logger, or "shlogger" for short, recursively runs through
directories specified in the `default_paths.txt` and produces a list of all the
files, sorted by file size, within those directories.  Files at the bottom of
the list are the largest and are considered space hoggers.

Additional user-defined paths can be included in a file with a path of
`Library/Application Support/The Lyons' Den Labs/shlogger_extra_paths.txt`.

## Disclaimers and Notes

space_hogger_logger is not held accountable for any bugs that may result in file
loss or corruption.  space_hogger_logger uses only the Rust std library function
calls and one third-party crate ([dirs](https://crates.io/crates/dirs)) to find
paths and run through files / directories; there is no custom code written to
deal with those tasks, so as long as the std library code and the third-party
crate are both solid and dependable, then there shouldn't be any issues.  I've
not had a single issue with the tool on my machine, but I've not tested it
elsewhere.

It should be noted that space_hogger_logger does not save any information, it
merely prints file paths, and their associated file size, to the terminal
window.

Lastly, space_hogger_logger only works on macOS currently.
