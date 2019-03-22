echo "compiling"
cargo run inverse.s
cat inverse.m > out.m
cargo run div.s
cat div.m >> out.m
cargo run sqrt.s
cat sqrt.m >> out.m
echo "copying"
cp out.m ../../Documents/School/Homework/CSE\ 141L/testing/Awesom-ISA-Processor/out.m
