# `space_hogger_logger` ("shlogger")

`space_hogger_logger` recursively runs through directories specified in the
`default_paths.txt` file and grabs the path and size of every file.  If extra
paths are supplied in the
`Library/Application Support/The Lyons' Den Labs/shlogger_extra_paths.txt` file,
then these will be added to the search.  When complete, `space_hogger_logger`
prints the files out, displaying the largest files at the bottom of the list;
these files are the space hoggers, and should be considered first when looking
for ways to make space.

`space_hogger_logger` is inspired by (Daisy Disk)[https://daisydiskapp.com].

*Note*: `space_hogger_logger` only works on macOS currently.
