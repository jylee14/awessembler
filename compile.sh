echo "compiling"
cargo run inverse.s
cat inverse.m > out.m
cargo run div.s
echo "" >> out.m
cat div.m >> out.m
cargo run sqrt.s
echo "" >> out.m
cat sqrt.m >> out.m
echo "copying"
cp out.m ../../Documents/School/Homework/CSE\ 141L/Awesom-ISA-Processor/out.m
