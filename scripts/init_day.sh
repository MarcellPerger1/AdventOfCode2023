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

ORIG_PWD=$(pwd)
TARGET_NAME="$1"
cargo new "$TARGET_NAME" || exit 1
cd "$TARGET_NAME" || exit 1
cargo add itertools regex num || exit 1
cat "$ORIG_PWD/scripts/base_main_file.rs.template" > "./src/main.rs"
# build packages in both debug and release mode
cargo run
cargo run -r
