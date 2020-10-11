project="$1"
testcase="$2"
cargo run -p $project < $project/$testcase.in >$project/$testcase.out
diff -y --left-column $project/$testcase.exp $project/$testcase.out
