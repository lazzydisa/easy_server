#!/usr/bin/env sh

case $1 in
    "help" | "h")
        echo 'Usage:'
        echo '    ./build.sh test "ip:port"     # runs `cargo run --bin dsc_server -- ip:port` (needed for test runs only)'
        echo '    ./build.sh release "ip:port"  # runs `cargo run -r --bin dsc_server -- ip:port` (if you are ready to use server)'
        echo '    ./build.sh setup              # runs `cargo run --bin setup` (needed to update `src/main.rs` to use new `.html` files)'
        echo '    ./build.sh help               # prints this info (and you are reading it!)'
        echo "\nFor info about configuration open README.md!"
    ;;
    "release" | "r")
        case $2 in
            "") $0 help;;
            *) cargo run -r --bin easy_server -- $2;;
        esac
    ;;
    "test" | "t")
        case $2 in
            "") $0 help;;
            *) cargo run --bin easy_server -- $2;;
        esac
    ;;
    "setup" | "s") cargo run --bin setup;;
    *)  echo 'run ./build.sh help for more information'
        exit 1
    ;;
esac
