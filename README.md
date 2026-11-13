`lazy_server` is a little project that is needed for generating a backend for a ststic html sites

To create a site you need to add html files to the `sources` directory and add filenames in the `list.txt`.
If you put files in a subdirectory, then you need to add directory name with the filename to the `list.txt` too!

Example for `list.txt`:
```
# root.html should be in `sources/`:
root.html
foobar.html

# a subdirectory:
foo/bar.html
foo/bar2.html

# more info about `list.txt` read in that file!
```

Usage:
```sh
# to setup/update backend:
cargo setup
# or
cargo s

# to run the server as dev build:
cargo test "ip:port"
# or
cargo t "ip:port"

# to run the server as release build:
cargo release "ip:port"
# or
cargo r "ip:port"
```

Also don't forget to delete example files in `sources/`!
