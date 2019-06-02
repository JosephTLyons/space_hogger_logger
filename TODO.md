# TODO

## Required:

- Add more default paths
    - Add applications folder (figure out permissions workaround)
- Not all results should panic (expect()), the app should continue if some
  things fail.

## Nice Additions:

- A cleaner way to print paths (use `...` to break up the path by default,
  the full path could be displayed with a verbose option)
- Options to ignore certain files (hidden, items below a size threshold)?

## Things to Ponder

- Should this use an iterative algorithm over recursion?

## Things to Test

- Test that file sizes are correctly reported by Pretty Bytes, and larger sizes
  as well

## Code and Project Cleanup

- Get both functions in main to be similar using similar inputs and methods
- README.md
    - Give more details and clean it up
    - Add Space Hogger Logger graphic
