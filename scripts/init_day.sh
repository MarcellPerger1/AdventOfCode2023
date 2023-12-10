USAGE="usage: $0 dirname"
for arg; do 
    if test "$arg" == '--help' || test "$arg" == '-h'; then 
        echo "$USAGE"
        exit 2
    fi
done
if [ -z "$1" ]; then
    echo "$USAGE"
    exit 2
fi

cargo new "$1" || exit 1
cd "$1" || exit 1
cargo add itertools regex num || exit 1
# build packages in both debug and release mode
cargo run
cargo run -r
