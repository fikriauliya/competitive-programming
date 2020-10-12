project="$1"
testcase="$2"
RUST_BACKTRACE=1 cargo run -p $project --bin $project < $project/$testcase.in >$project/$testcase.out
diff -y --left-column $project/$testcase.ans $project/$testcase.out
