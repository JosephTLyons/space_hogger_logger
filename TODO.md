# TODO

- Only print the path starting from where we are looking, relative to what is
  initially passed into the function of the main.
    - This may not be what we want when I start to actually add paths to
        typical search to make this an automated tool
    - Could resolve it by printing the path we send in, then indenting all items
      in that path once.
- Show files in larger sizes (kb, mb) ?
    - This could be dynamic (if < 1024, convert to next level up)
    - Use a crate for this
        - https://crates.io/crates/bytes
        - https://github.com/banyan/rust-pretty-bytes
- Be able to read other items (applications and such)
- Make a structure to hold all the paths this should actually search on Mac,
  such as a vector of Paths, and put this in a separate module probably?
- Options to show / hide hidden files
- Threshold size option for which files to show and not show
- Make a README.md
- Publish on GitHub.com
- Should this ignore any files?
- Should this use an iterative algorithm over recursion?

# Potential Names

- Space Wrangler
- Space Hogger Logger
- Space Culprit
- Bit Blamer
