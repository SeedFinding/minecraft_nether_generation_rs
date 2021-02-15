cargo build --release
valgrind --tool=callgrind --simulate-cache=yes  --callgrind-out-file=call.out target/debug/temp
callgrind_annotate --threshold=100  --auto=yes call.out > bench.out
