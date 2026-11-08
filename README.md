`easy_server` is a little project that is needed for generating a backend for a ststic html sites

To create a site you need to add html files to the `sources` directory and add filenames in the `list.txt`
if you put files in a subdirectory, then you need to add directory name with the filename to the `list.txt` too!

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
./build.sh setup

# to run server:
./build.sh release "ip:port"

# to see all `build.sh` commands:
./build.sh help
```

Also don't forget to delete example files in `sources/`!
