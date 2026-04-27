cat tests/test$1.in | cargo run 2>/dev/null | diff - tests/test$1.out
