# TODO

- Only print the path starting from where we are looking, relative to what is
  initially passed into the function of the main.
    - This may not be what we want when I start to actually add paths to
      typical search to make this an automated tool
    - Could resolve it by printing the path we send in, then indenting all items
      in that path once.
- Be able to read other items (applications and such)
- Polish paths file system
- Options to show / hide hidden files
- Threshold size option for which files to show and not show
- Add Space Hogger Logger graphic to README.md
- Should this ignore any files?
- Should this use an iterative algorithm over recursion?
- Research and understand sorting mechanism
- Test that file sizes are correctly reported by Pretty Bytes, and larger sizes
  as well
- Update all the messages from the expect() functions to be more accurate
- Should all results panic?  Maybe if the dir iter doesn't work, but not if a
  single file doesn't work
- Format output with largest expect size string
- Either add an '/' at root or before each file path, probably the root
- Should printing happen in reverse order?
- Add Display trait?
- Divide code into files if I can put them in structs
- Add more default paths
- Rename the file "shlogger_user_defined_paths.txt" or something shorter?
- Get both functions in main to be similar using similar inputs and methods

- Options
    - Set threshold
    - Report hidden files
