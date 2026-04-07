echo "Testing test case in file: test/test$1.in and comparing results to test/test$1.out"
echo "If you see nothing, that is good!"
cat test/test$1.in | cargo run -r 2>/dev/null | diff test/test$1.out -

