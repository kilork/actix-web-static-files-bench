TEST_TIME=`date "+%s"`
echo $TEST_TIME
touch build.rs
cargo build
cargo run
